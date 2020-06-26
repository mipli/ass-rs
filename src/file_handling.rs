use crate::{AssClient, AssError, FileData};
use reqwest::multipart::Form;
use reqwest::Url;
use std::path::PathBuf;

pub fn search(ass_client: &AssClient, queries: &[(&str, &str)]) -> Result<Vec<FileData>, AssError> {
    let client = reqwest::Client::builder()
        .default_headers(ass_client.get_headers()?)
        .build()?;
    let url = Url::parse(&ass_client.url_string())?;
    let url = url.join("files")?;
    let url = Url::parse_with_params(url.as_str(), queries)?;

    let mut res = client.get(url).send()?;

    Ok(serde_json::from_str(&res.text()?)?)
}

pub fn upload_file<T: Into<PathBuf>>(
    ass_client: &AssClient,
    path: T,
    destination: &str,
) -> Result<FileData, AssError> {
    let path = path.into();
    let url = Url::parse(&ass_client.url_string())?;
    let url = url.join(&format!("files/{}", destination))?;
    let file_name = get_filename_from_path(&path)?;
    let url = url.join(file_name)?;

    let form = Form::new().file("file", path)?;

    let client = reqwest::Client::builder()
        .default_headers(ass_client.get_headers()?)
        .build()?;

    let mut res = client.post(url).multipart(form).send()?;
    res.text()?.parse()
}

pub fn upload_file_with_headers<T: Into<PathBuf>>(
    ass_client: &AssClient,
    path: T,
    destination: &str,
    headers: &[(&str, &str)],
) -> Result<FileData, AssError> {
    let path = path.into();
    let url = Url::parse(&ass_client.url_string())?;
    let url = url.join(&format!("files/{}", destination))?;
    let file_name = get_filename_from_path(&path)?;
    let url = url.join(file_name)?;

    let form = Form::new().file("file", path)?;

    let client = reqwest::Client::builder()
        .default_headers(ass_client.get_headers()?)
        .build()?;

    let mut builder = client.post(url).multipart(form);

    for (k, v) in headers {
        builder = builder.header(*k, *v);
    }

    let mut res = builder.send()?;

    res.text()?.parse()
}

pub fn get_file_url(ass_client: &AssClient, path: &str) -> Result<String, AssError> {
    let url = Url::parse(&ass_client.url_string())?;
    let url = url.join(&format!("users/{}/files/{}", ass_client.name, path))?;
    let url = ass_client.sign_url(&url.as_str())?;
    Ok(url.to_string())
}

pub fn get_file_information(ass_client: &AssClient, file_id: u64) -> Result<FileData, AssError> {
    let url = Url::parse(&ass_client.url_string())?;
    let url = url.join(&format!("files/{}", file_id))?;

    let client = reqwest::Client::builder()
        .default_headers(ass_client.get_headers()?)
        .build()?;
    let mut res = client.get(url).send()?;
    res.text()?.parse()
}

pub fn get_file_analysis(ass_client: &AssClient, file_id: u64) -> Result<FileData, AssError> {
    let url = Url::parse(&ass_client.url_string())?;
    let url = url.join(&format!("files/{}/analysis", file_id))?;

    let client = reqwest::Client::builder()
        .default_headers(ass_client.get_headers()?)
        .build()?;
    let mut res = client.get(url).send()?;
    res.text()?.parse()
}

pub fn get_file_render(ass_client: &AssClient, file_id: u64) -> Result<FileData, AssError> {
    let url = Url::parse(&ass_client.url_string())?;
    let url = url.join(&format!("files/{}/image", file_id))?;

    let client = reqwest::Client::builder()
        .default_headers(ass_client.get_headers()?)
        .build()?;
    let mut res = client.get(url).send()?;
    res.text()?.parse()
}

fn get_filename_from_path(path: &PathBuf) -> Result<&str, AssError> {
    path.file_name().and_then(|s| s.to_str()).ok_or_else(|| {
        AssError::invalid_file_name(
            "Error parsing filename".to_string(),
            path.to_str().unwrap().to_string(),
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::{file_handling, AssClient, AssErrorKind};
    use mockito;

    #[test]
    fn test_file_upload() {
        let _m = mockito::mock("POST", "/files/file-path/account.json")
            .match_header("Authorization", "bearer apikey")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body("{\"id\": 2, \"user_id\": 3, \"path\": \"path\", \"md5\": \"checksum\", \"content_type\": \"text\", \"original_url\": \"url.com\", \"created\": \"2013-08-21T09:30:50.068Z\", \"updated\": \"2013-08-21T09:30:50.068Z\"}")
            .create();

        let url = &mockito::server_url();

        let ass_client =
            AssClient::create(url, "account", "apikey").expect("Could not get AssClient");

        let result = file_handling::upload_file(&ass_client, "./data/account.json", "file-path/")
            .expect("Could not get result");
        assert_eq!(result.id, 2);
    }

    #[test]
    fn test_file_upload_with_headers() {
        let _m = mockito::mock("POST", "/files/file-path/account.json")
            .match_header("Authorization", "bearer apikey")
            .match_header("Cache-Control", "max-age: 234")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body("{\"id\": 2, \"user_id\": 3, \"path\": \"path\", \"md5\": \"checksum\", \"content_type\": \"text\", \"original_url\": \"url.com\", \"created\": \"2013-08-21T09:30:50.068Z\", \"updated\": \"2013-08-21T09:30:50.068Z\"}")
            .create();

        let url = &mockito::server_url();

        let ass_client =
            AssClient::create(url, "account", "apikey").expect("Could not get Account");

        let result = file_handling::upload_file_with_headers(
            &ass_client,
            "./data/account.json",
            "file-path/",
            &[("Cache-Control", "max-age: 234")],
        )
        .expect("Could not get result");
        assert_eq!(result.id, 2);
    }
}