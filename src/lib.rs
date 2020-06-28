//! # ASS-rs
//!
//! A library for working with Aptoma Smooth Storage.
//!
//! # Examples
//! ```ignore
//! use ass_rs::{AssClient, AssError, file_handling, image_handling};
//!
//! # fn main() -> Result<(), AssError> {
//!
//! let ass_client = AssClient::create("https://url-to-storage", "account-name", "secretkey")?;
//!
//! let image_url = image_handling::get_image_url(&ass_client, 123)?;
//! let image_data = image_handling::get_image_data(&ass_client, 123)?;
//!
//! let file_data = file_handling::upload_file(&ass_client, "/data/file.pdf", "/destination")?;
//! let image_data = file_handling::upload_image(&ass_client, "/data/image.jpg")?;
//!
//! # Ok(())
//! # }
//!
//! ```

mod client;
mod data;
mod error;
pub mod file_handling;
pub mod image_handling;

pub use crate::client::AssClient;
pub use crate::data::{FileData, ImageData};
pub use crate::error::{AssError, AssErrorKind};
