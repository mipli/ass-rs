use crate::AssError;
use serde_json::Value;
use std::fmt::{Debug, Display};
use std::str::FromStr;

pub struct AssData(Value);

impl AssData {
    pub fn new(v: Value) -> Self {
        AssData(v)
    }

    pub fn get_id(&self) -> Option<u64> {
        self.0.get("id")?.as_u64()
    }

    pub fn get_path(&self) -> Option<&str> {
        self.0.get("path")?.as_str()
    }
}

impl FromStr for AssData {
    type Err = AssError;
    fn from_str(s: &str) -> Result<AssData, AssError> {
        let data: Value = serde_json::from_str(s)?;
        Ok(AssData { 0: data })
    }
}

impl Display for AssData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl Debug for AssData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", serde_json::to_string_pretty(&self.0).unwrap_or_else(|_| "Invalid JSON".to_string()))
    }
}
