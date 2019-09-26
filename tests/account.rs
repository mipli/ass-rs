#[cfg(test)]
mod account_tests {
    use ass_rs::{Account, AssErrorKind};
    use mockito;

    #[test]
    fn create_account() {
        let account = Account::create("http://url", "name", "key");
        assert!(account.is_ok());
        let account = account.expect("Could not get Account");
        assert_eq!(account.url().to_string(), "http://url/");
        assert_eq!(account.name, "name");
        assert_eq!(account.apikey, "key");
    }

    #[test]
    fn create_account_from_file() {
        let account = Account::from_file("./data/account.json");
        assert!(account.is_ok());
        let account = account.expect("Could not get Account");
        assert_eq!(account.url().to_string(), "http://url/");
        assert_eq!(account.name, "name");
        assert_eq!(account.apikey, "apikey");
    }

    #[test]
    fn get_headers() {
        let account = Account::from_file("./data/account.json");
        let account = account.expect("Could not get Account");

        let headers = account.get_headers().expect("Could not get headers");
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
        let account = Account::from_file("./data/account.json");
        let account = account.expect("Could not get Account");

        let url = account
            .sign_url("http://url.com/name/image/2")
            .expect("Could not sign url");
        assert_eq!(url.to_string(), "http://url.com/name/image/2?accessToken=6ea029fcb85dd473116edbc80a500b99ef7f8c32dacbca51bf2be622a38ab6c9");
    }

    #[test]
    fn sign_url_fails_on_wrong_account_url() {
        let account = Account::from_file("./data/account.json");
        let account = account.expect("Could not get Account");

        match account.sign_url("http://url.com/foobar/images/") {
            Err(e) => match e.kind {
                AssErrorKind::UrlDoesNotMatchAccount(_) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_file_upload() {
        let _m = mockito::mock("POST", "/files/file-path/account.json")
            .match_header("Authorization", "bearer apikey")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body("{\"id\": 2, \"path\": \"path\"}")
            .create();

        let url = &mockito::server_url();

        let account = Account::create(url, "account", "apikey");
        assert!(account.is_ok());
        let account = account.expect("Could not get Account");

        let result = account.upload_file("./data/account.json", "file-path/");
        assert!(result.is_ok());
        let result = result.expect("Could not get result");
        assert_eq!(result.get_id(), Some(2));
    }

    #[test]
    fn test_file_upload_with_headers() {
        let _m = mockito::mock("POST", "/files/file-path/account.json")
            .match_header("Authorization", "bearer apikey")
            .match_header("Cache-Control", "max-age: 234")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body("{\"id\": 2, \"path\": \"path\"}")
            .create();

        let url = &mockito::server_url();

        let account = Account::create(url, "account", "apikey");
        assert!(account.is_ok());
        let account = account.expect("Could not get Account");

        let result = account.upload_file_with_headers("./data/account.json", "file-path/", &[("Cache-Control", "max-age: 234")]);
        assert!(result.is_ok());
        let result = result.expect("Could not get result");
        assert_eq!(result.get_id(), Some(2));
    }
}
