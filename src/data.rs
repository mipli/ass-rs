use crate::AssError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageData {
    pub id: u64,
    pub user_id: u64,
    pub md5: String,
    pub original_url: String,
    pub width: u64,
    pub height: u64,
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub source_url: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl FromStr for ImageData {
    type Err = AssError;
    fn from_str(s: &str) -> Result<ImageData, AssError> {
        let data: ImageData = serde_json::from_str(s)?;
        Ok(data)
    }
}

impl Display for ImageData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileData {
    pub id: u64,
    pub user_id: u64,
    pub path: String,
    pub md5: String,
    pub content_type: String,
    pub original_url: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl FromStr for FileData {
    type Err = AssError;
    fn from_str(s: &str) -> Result<FileData, AssError> {
        let data: FileData = serde_json::from_str(s)?;
        Ok(data)
    }
}

impl Display for FileData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}
