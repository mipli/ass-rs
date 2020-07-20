use crate::AssError;
use reqwest::header::HeaderMap;
use reqwest::Url;
use ring::{digest, hmac};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

///
/// Client talking to Aptoma Smooth Storage.
///
/// Client contains authentication information and functions for a few basic actions. Most
/// communcation will be done using the `file_handling` and `image_handling` modules.
///
/// ```rust
/// # use ass_rs::AssError;
/// # fn main() -> Result<(), AssError> {
/// use ass_rs::{AssClient};
///
/// let ass_client = AssClient::create("https://storage.url.com", "username", "password")?;
///
/// let signed_url = ass_client.sign_url("https://storage.url.com/users/username/image/5.jpg")?;
/// assert_eq!(signed_url.to_string(), "https://storage.url.com/users/username/image/5.jpg?accessToken=462082d1754bd61893094a26763480e745c679be31a483c25268205eaa7ccb08");
///
/// # Ok(())
/// # }
/// ```
///
#[derive(Debug, Serialize, Deserialize)]
pub struct AssClient {
    url: String,
    pub name: String,
    pub apikey: String,
}

impl AssClient {
    /// Create a new client based on suppied `url`, `name` and `password`
    pub fn create<T: Into<String>, U: Into<String>, V: Into<String>>(
        url: T,
        name: U,
        apikey: V,
    ) -> Result<Self, AssError> {
        let url = url.into();
        let _ = url.parse::<Url>()?;
        Ok(AssClient {
            url,
            name: name.into(),
            apikey: apikey.into(),
        })
    }

    /// Create a new client, fetch `url`, `name` and `password` from account file at the given path
    pub fn from_file<T: Into<PathBuf>>(path: T) -> Result<Self, AssError> {
        let path = path.into();
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str::<AssClient>(&contents).map_err(|err| {
            let path = match path.to_str() {
                Some(p) => p.to_string(),
                None => "Unknown path".to_string(),
            };
            AssError::invalid_account_file(err.to_string(), path)
        })
    }

    /// Fetch the client's base url
    pub fn url(&self) -> Url {
        self.url
            .parse::<Url>()
            .expect("Could not parse account URL")
    }

    /// Fetch the client's base url as `String`
    pub fn url_string(&self) -> String {
        self.url
            .parse::<Url>()
            .expect("Could not parse account URL")
            .to_string()
    }

    /// Fetch base headers to use when interacting with the smooth storage
    pub(crate) fn get_headers(&self) -> Result<HeaderMap, AssError> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("bearer {}", self.apikey).parse()?);
        headers.insert("Accept", "application/json".parse()?);
        headers.insert("x-ass-acl", "public".parse()?);

        Ok(headers)
    }

    /// Sign the given url using the client's credentials.
    /// Ensures that the url matches the client's credentials before signing
    pub fn sign_url(&self, url: &str) -> Result<Url, AssError> {
        let key = hmac::SigningKey::new(&digest::SHA256, &self.apikey.as_bytes());
        let signature = hmac::sign(&key, url.as_bytes());
        if !url.contains(&self.url) || !url.contains(&self.name) {
            return Err(AssError::url_does_not_match_account(url.to_string()));
        }
        let s: String = signature
            .as_ref()
            .iter()
            .map(|s| format!("{:02x}", s))
            .collect();
        Url::parse_with_params(url, &[("accessToken", &s)]).map_err(|err| err.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{AssClient, AssErrorKind};

    #[test]
    fn create_ass_client() {
        let ass_client =
            AssClient::create("http://url", "name", "key").expect("Could not get AssClient");
        assert_eq!(ass_client.url().to_string(), "http://url/");
        assert_eq!(ass_client.name, "name");
        assert_eq!(ass_client.apikey, "key");
    }

    #[test]
    fn create_ass_client_from_file() {
        let ass_client =
            AssClient::from_file("./data/account.json").expect("Could not get AssClient");
        assert_eq!(ass_client.url().to_string(), "http://url/");
        assert_eq!(ass_client.name, "name");
        assert_eq!(ass_client.apikey, "apikey");
    }

    #[test]
    fn get_headers() {
        let ass_client = AssClient::from_file("./data/account.json");
        let ass_client = ass_client.expect("Could not get AssClient");

        let headers = ass_client.get_headers().expect("Could not get headers");
        assert_eq!(headers.keys_len(), 3);
        assert_eq!(
            headers
                .get("Authorization")
                .expect("Failed to get Authorization header"),
            &"bearer apikey"
        );
        assert_eq!(
            headers.get("Accept").expect("Failed to get Accept header"),
            &"application/json"
        );
        assert_eq!(
            headers
                .get("x-ass-acl")
                .expect("Failed to get x-ass-acl header"),
            &"public"
        );
    }

    #[test]
    fn sign_url() {
        let ass_client = AssClient::from_file("./data/account.json");
        let ass_client = ass_client.expect("Could not get AssClient");

        let url = ass_client
            .sign_url("http://url.com/name/image/2")
            .expect("Could not sign url");
        assert_eq!(url.to_string(), "http://url.com/name/image/2?accessToken=6ea029fcb85dd473116edbc80a500b99ef7f8c32dacbca51bf2be622a38ab6c9");
    }

    #[test]
    fn sign_url_fails_on_wrong_ass_client_url() {
        let ass_client = AssClient::from_file("./data/account.json");
        let ass_client = ass_client.expect("Could not get AssClient");

        match ass_client.sign_url("http://url.com/foobar/images/") {
            Err(e) => match e.kind {
                AssErrorKind::UrlDoesNotMatchAccount(_) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }
}
