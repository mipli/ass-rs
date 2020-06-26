use crate::{AssError, FileData, ImageData};
use reqwest::header::HeaderMap;
use reqwest::multipart::Form;
use reqwest::Url;
use ring::{digest, hmac};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn search_files(&self, queries: &[(&str, &str)]) -> Result<Vec<FileData>, AssError> {
    let client = reqwest::Client::builder()
        .default_headers(self.get_headers()?)
        .build()?;
    let url = Url::parse(&self.url_string())?;
    let url = url.join("files")?;
    let url = Url::parse_with_params(url.as_str(), queries)?;

    let mut res = client.get(url).send()?;

    Ok(serde_json::from_str(&res.text()?)?)
}

pub fn upload_file<T: Into<PathBuf>>(
    &self,
    path: T,
    destination: &str,
) -> Result<FileData, AssError> {
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
    res.text()?.parse()
}

pub fn upload_file_with_headers<T: Into<PathBuf>>(
    &self,
    path: T,
    destination: &str,
    headers: &[(&str, &str)],
) -> Result<FileData, AssError> {
    let path = path.into();
    let url = Url::parse(&self.url_string())?;
    let url = url.join(&format!("files/{}", destination))?;
    let file_name = self.get_filename_from_path(&path)?;
    let url = url.join(file_name)?;

    let form = Form::new().file("file", path)?;

    let client = reqwest::Client::builder()
        .default_headers(self.get_headers()?)
        .build()?;

    let mut builder = client.post(url).multipart(form);

    for (k, v) in headers {
        builder = builder.header(*k, *v);
    }

    let mut res = builder.send()?;

    res.text()?.parse()
}

pub fn get_file_url(&self, path: &str) -> Result<String, AssError> {
    let url = Url::parse(&self.url_string())?;
    let url = url.join(&format!("users/{}/files/{}", self.name, path))?;
    let url = self.sign_url(&url.as_str())?;
    Ok(url.to_string())
}

pub fn get_file_information(&self, file_id: u64) -> Result<FileData, AssError> {
    let url = Url::parse(&self.url_string())?;
    let url = url.join(&format!("files/{}", file_id))?;

    let client = reqwest::Client::builder()
        .default_headers(self.get_headers()?)
        .build()?;
    let mut res = client.get(url).send()?;
    res.text()?.parse()
}

pub fn get_file_analysis(&self, file_id: u64) -> Result<FileData, AssError> {
    let url = Url::parse(&self.url_string())?;
    let url = url.join(&format!("files/{}/analysis", file_id))?;

    let client = reqwest::Client::builder()
        .default_headers(self.get_headers()?)
        .build()?;
    let mut res = client.get(url).send()?;
    res.text()?.parse()
}

pub fn get_file_render(&self, file_id: u64) -> Result<FileData, AssError> {
    let url = Url::parse(&self.url_string())?;
    let url = url.join(&format!("files/{}/image", file_id))?;

    let client = reqwest::Client::builder()
        .default_headers(self.get_headers()?)
        .build()?;
    let mut res = client.get(url).send()?;
    res.text()?.parse()
}

fn get_filename_from_path<'a>(&self, path: &'a PathBuf) -> Result<&'a str, AssError> {
    path.file_name().and_then(|s| s.to_str()).ok_or_else(|| {
        AssError::invalid_file_name(
            "Error parsing filename".to_string(),
            path.to_str().unwrap().to_string(),
        )
    })
}
