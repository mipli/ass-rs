use crate::{AssClient, AssError, ImageData};
use reqwest::multipart::Form;
use reqwest::Url;
use std::path::PathBuf;

pub async fn upload_image<T: Into<PathBuf>>(
    ass_client: &AssClient,
    path: T,
) -> Result<ImageData, AssError> {
    let path = path.into();
    let url = Url::parse(&ass_client.url_string())?;
    let url = url.join("images")?;

    let stream = std::fs::read(path)?;
    let form = Form::new().part("file", reqwest::multipart::Part::stream(stream));

    let client = reqwest::Client::builder()
        .default_headers(ass_client.get_headers()?)
        .build()?;

    let res = client.post(url).multipart(form).send().await?;
    let data: ImageData = res.json().await?;
    Ok(data)
}

pub async fn get_image_information(
    ass_client: &AssClient,
    image_id: u64,
) -> Result<ImageData, AssError> {
    let url = Url::parse(&ass_client.url_string())?;
    let url = url.join(&format!("images/{}", image_id))?;

    let client = reqwest::Client::builder()
        .default_headers(ass_client.get_headers()?)
        .build()?;
    let res = client.get(url).send().await?;
    let data: ImageData = res.json().await?;
    Ok(data)
}

pub fn get_image_url(ass_client: &AssClient, id: u64) -> Result<String, AssError> {
    let url = Url::parse(&ass_client.url_string())?;
    let url = url.join(&format!("users/{}/images/{}.jpg", ass_client.name, id))?;
    let url = ass_client.sign_url(&url.as_str())?;
    Ok(url.to_string())
}
