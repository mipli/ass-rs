use crate::{AssData, AssError};
use reqwest::header::HeaderMap;
use reqwest::multipart::Form;
use reqwest::Url;
use ring::{digest, hmac};
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    url: String,
    pub name: String,
    pub apikey: String,
}

impl Account {
    pub fn create<T: Into<String>, U: Into<String>, V: Into<String>>(
        url: T,
        name: U,
        apikey: V,
    ) -> Result<Self, AssError> {
        let url = url.into();
        let _ = url.parse::<Url>()?;
        Ok(Account {
            url,
            name: name.into(),
            apikey: apikey.into(),
        })
    }

    pub fn url(&self) -> Url {
        self.url
            .parse::<Url>()
            .expect("Could not parse account URL")
    }

    pub fn url_string(&self) -> String {
        self.url
            .parse::<Url>()
            .expect("Could not parse account URL")
            .to_string()
    }

    pub fn get_headers(&self) -> Result<HeaderMap, AssError> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("bearer {}", self.apikey).parse()?);
        headers.insert("Accept", "application/json".parse()?);
        headers.insert("x-ass-acl", "public".parse()?);

        Ok(headers)
    }

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

    pub fn from_file<T: Into<PathBuf>>(path: T) -> Result<Self, AssError> {
        let path = path.into();
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str::<Account>(&contents).map_err(|err| {
            let path = match path.to_str() {
                Some(p) => p.to_string(),
                None => "Unknown path".to_string(),
            };
            AssError::invalid_account_file(err.to_string(), path)
        })
    }

    pub fn search_files(&self, queries: &[(&str, &str)]) -> Result<Vec<AssData>, AssError> {
        let client = reqwest::Client::builder()
            .default_headers(self.get_headers()?)
            .build()?;
        let url = Url::parse(&self.url_string())?;
        let url = url.join("files")?;
        let url = Url::parse_with_params(url.as_str(), queries)?;

        let mut res = client.get(url).send()?;

        let data: serde_json::Value = res.text()?.parse()?;
        Ok(data.as_array().map_or(vec![], |arr| {
            arr.iter()
                .map(|a| AssData::new((*a).clone()))
                .collect::<Vec<_>>()
        }))
    }

    pub fn upload_file<T: Into<PathBuf>>(
        &self,
        path: T,
        destination: &str,
    ) -> Result<AssData, AssError> {
        let path = path.into();
        let url = Url::parse(&self.url_string())?;
        let url = url.join(&format!("files/{}", destination))?;
        let file_name = self.get_filename_from_path(&path)?;
        let url = url.join(file_name)?;

        let form = Form::new().file("file", path)?;

        let client = reqwest::Client::builder()
            .default_headers(self.get_headers()?)
            .build()?;

        let mut res = client.post(url).multipart(form).send()?;
        let data: AssData = res.text()?.parse()?;
        Ok(data)
    }

    pub fn upload_file_with_cache<T: Into<PathBuf>>(
        &self,
        path: T,
        destination: &str,
        expiration: u32,
    ) -> Result<AssData, AssError> {
        let path = path.into();
        let url = Url::parse(&self.url_string())?;
        let url = url.join(&format!("files/{}", destination))?;
        let file_name = self.get_filename_from_path(&path)?;
        let url = url.join(file_name)?;

        let form = Form::new().file("file", path)?;

        let client = reqwest::Client::builder()
            .default_headers(self.get_headers()?)
            .build()?;

        let mut res = client
            .post(url)
            .multipart(form)
            .header("Cache-Control", format!("max-age: {}", expiration))
            .send()?;

        let data: AssData = res.text()?.parse()?;
        Ok(data)
    }

    pub fn get_file_url(&self, path: &str) -> Result<String, AssError> {
        let url = Url::parse(&self.url_string())?;
        let url = url.join(&format!("users/{}/files/{}", self.name, path))?;
        let url = self.sign_url(&url.as_str())?;
        Ok(url.to_string())
    }

    pub fn upload_image<T: Into<PathBuf>>(&self, path: T) -> Result<AssData, AssError> {
        let path = path.into();
        let url = Url::parse(&self.url_string())?;
        let url = url.join("images")?;

        let form = Form::new().file("file", path)?;

        let client = reqwest::Client::builder()
            .default_headers(self.get_headers()?)
            .build()?;

        let mut res = client.post(url).multipart(form).send()?;
        let data: AssData = res.text()?.parse()?;
        Ok(data)
    }

    pub fn get_image_data(&self, image_id: u64) -> Result<AssData, AssError> {
        let url = Url::parse(&self.url_string())?;
        let url = url.join(&format!("images/{}", image_id))?;

        let client = reqwest::Client::builder()
            .default_headers(self.get_headers()?)
            .build()?;
        let mut res = client.get(url).send()?;
        let data: AssData = res.text()?.parse()?;
        Ok(data)
    }

    pub fn get_image_url(&self, id: u64) -> Result<String, AssError> {
        let url = Url::parse(&self.url_string())?;
        let url = url.join(&format!("users/{}/images/{}.jpg", self.name, id))?;
        let url = self.sign_url(&url.as_str())?;
        Ok(url.to_string())
    }

    fn get_filename_from_path<'a>(&self, path: &'a PathBuf) -> Result<&'a str, AssError> {
        path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| {
                AssError::invalid_file_name(
                    "Error parsing filename".to_string(),
                    path.to_str().unwrap().to_string(),
                )
            })
    }
}
