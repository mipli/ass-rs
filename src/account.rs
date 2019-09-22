use reqwest::{Url};
use ring::{digest, hmac};
use std::path::PathBuf;
use reqwest::header::{HeaderMap};
use reqwest::multipart::Form;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use failure::{Error};
use crate::{AssError, AssData};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    url: String,
    pub name: String,
    pub apikey: String
}

impl Account {
    pub fn create<T: Into<String>>(url: T, name: T, apikey: T) -> Result<Self, Error> {
        let url = url.into();
        let _ = url.parse::<Url>()?;
        Ok(Account {
            url,
            name: name.into(),
            apikey: apikey.into()
        })
    }

    pub fn url(&self) -> Url {
        self.url.parse::<Url>().expect("Could not parse account URL")
    }

    pub fn url_string(&self) -> String {
        self.url.parse::<Url>().expect("Could not parse account URL").to_string()
    }

    pub fn get_headers(&self) -> Result<HeaderMap, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("bearer {}", self.apikey).parse()?);
        headers.insert("Accept", "application/json".parse()?);
        headers.insert("x-ass-acl", "public".parse()?);

        Ok(headers)
    }

    pub fn sign_url(&self, url: Url) -> Result<Url, Error> {
        let key = hmac::SigningKey::new(&digest::SHA256, &self.apikey.as_bytes());
        let signature = hmac::sign(&key, &url.as_str().as_bytes());
        let s: String = signature.as_ref().iter().map(|s| format!("{:02x}", s)).collect();
        Url::parse_with_params(url.as_str(), &[("accessToken", &s)])
            .map_err(|err| err.into())
    }

    pub fn from_file<T: Into<PathBuf>>(file: T) -> Result<Self, AssError> {
        let file = file.into();
        let mut file = File::open(&file)
            .map_err(|err| AssError::NotFound{ 
                err: err.to_string(), 
                file: file.to_str().unwrap().to_string() 
            })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| AssError::PermissionDenied{ 
                err: err.to_string(), 
                file: err.to_string() 
            })?;
        serde_json::from_str::<Account>(&contents)
            .map_err(|err| AssError::InvalidAccountFile{
                err: err.to_string(), 
                file: err.to_string() 
            })
    }

    pub fn search_files(&self, queries: &[(&str, &str)]) -> Result<Vec<AssData>, Error> {
        let client = reqwest::Client::builder()
            .default_headers(self.get_headers()?)
            .build()?;
        let url = Url::parse(&self.url_string())?;
        let url = url.join("files")?;
        let url = Url::parse_with_params(url.as_str(), queries)?;

        let mut res = client
            .get(url)
            .send()?;

        let data: serde_json::Value = res.text()?.parse()?;
        Ok(data.as_array().map_or(vec![], |arr| {
            arr.iter().map(|a| {
                AssData::new((*a).clone())
            }).collect::<Vec<_>>()
        }))
    }

    pub fn upload_file<T: Into<PathBuf>>(&self, path: T, destination: &str) -> Result<AssData, Error> {
        let path = path.into();
        let url = Url::parse(&self.url_string())?;
        let url = url.join(&format!("files/{}", destination))?;
        let file_name = path
            .file_name().ok_or_else(|| AssError::InvalidFile {
                err: "Error parsing filename".to_string(),
                file: path.to_str().unwrap().to_string() 
            })?.to_str().ok_or_else(|| AssError::InvalidFile {
                err: "Error parsing filename".to_string(),
                file: path.to_str().unwrap().to_string() 
            })?;
        let url = url.join(file_name)?;

        let form = Form::new().file("file", path)?;

        let client = reqwest::Client::builder()
            .default_headers(self.get_headers()?)
            .build()?;

        let mut res = client
            .post(url)
            .multipart(form)
            .send()?;
        let data: AssData = res.text()?.parse()?;
        Ok(data)
    }

    pub fn upload_file_with_cache<T: Into<PathBuf>>(&self, path: T, destination: &str, expiration: u32) -> Result<AssData, Error> {
        let path = path.into();
        let url = Url::parse(&self.url_string())?;
        let url = url.join(&format!("files/{}", destination))?;
        let file_name = path
            .file_name().ok_or_else(|| AssError::InvalidFile {
                err: "Error parsing filename".to_string(),
                file: path.to_str().unwrap().to_string() 
            })?.to_str().ok_or_else(|| AssError::InvalidFile {
                err: "Error parsing filename".to_string(),
                file: path.to_str().unwrap().to_string() 
            })?;
        let url = url.join(file_name)?;

        let form = Form::new().file("file", path)?;

        let client = reqwest::Client::builder()
            .default_headers(self.get_headers()?)
            .build()?;

        let mut res = client
            .post(url)
            .multipart(form)
            .header("Content-Type", format!("max-age: {}", expiration))
            .send()? ;

        let data: AssData = res.text()?.parse()?;
        Ok(data)
    }

    pub fn get_file_url(&self, path: &str) -> Result<String, Error> {
        let url = Url::parse(&self.url_string())?;
        let url = url.join(&format!("users/{}/files/{}", self.name, path))?;
        let url = self.sign_url(url)?;
        Ok(url.to_string())
    }

    pub fn upload_image<T: Into<PathBuf>>(&self, path: T) -> Result<AssData, Error> {
        let path = path.into();
        let url = Url::parse(&self.url_string())?;
        let url = url.join("images")?;

        let form = Form::new().file("file", path)?;

        let client = reqwest::Client::builder()
            .default_headers(self.get_headers()?)
            .build()?;

        let mut res = client
            .post(url)
            .multipart(form)
            .send()?;
        let data: AssData = res.text()?.parse()?;
        Ok(data)
    }

    pub fn get_image_data(&self, image_id: u64) -> Result<AssData, Error>  {
        let url = Url::parse(&self.url_string())?;
        let url = url.join(&format!("images/{}", image_id))?;

        let client = reqwest::Client::builder()
            .default_headers(self.get_headers()?)
            .build()?;
        let mut res = client.get(url).send()?;
        let data: AssData = res.text()?.parse()?;
        Ok(data)
    }

    pub fn get_image_url(&self, id: u64) -> Result<String, Error> {
        let url = Url::parse(&self.url_string())?;
        let url = url.join(&format!("users/{}/images/{}.jpg", self.name, id))?;
        let url = self.sign_url(url)?;
        Ok(url.to_string())
    }
}
