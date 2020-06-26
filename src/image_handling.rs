use crate::{AssClient, AssError, ImageData};
use reqwest::multipart::Form;
use reqwest::Url;
use std::path::PathBuf;

pub fn upload_image<T: Into<PathBuf>>(
    ass_client: &AssClient,
    path: T,
) -> Result<ImageData, AssError> {
    let path = path.into();
    let url = Url::parse(&ass_client.url_string())?;
    let url = url.join("images")?;

    let form = Form::new().file("file", path)?;

    let client = reqwest::Client::builder()
        .default_headers(ass_client.get_headers()?)
        .build()?;

    let mut res = client.post(url).multipart(form).send()?;
    res.text()?.parse()
}

pub fn get_image_information(ass_client: &AssClient, image_id: u64) -> Result<ImageData, AssError> {
    let url = Url::parse(&ass_client.url_string())?;
    let url = url.join(&format!("images/{}", image_id))?;

    let client = reqwest::Client::builder()
        .default_headers(ass_client.get_headers()?)
        .build()?;
    let mut res = client.get(url).send()?;
    res.text()?.parse()
}

pub fn get_image_url(ass_client: &AssClient, id: u64) -> Result<String, AssError> {
    let url = Url::parse(&ass_client.url_string())?;
    let url = url.join(&format!("users/{}/images/{}.jpg", ass_client.name, id))?;
    let url = ass_client.sign_url(&url.as_str())?;
    Ok(url.to_string())
}
