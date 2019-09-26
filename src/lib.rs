//! # ASS-rs
//!
//! A library for working with Aptoma Smooth Storage.
//!
//! # Examples
//! ```ignore
//! use ass_rs::{Account, AssError};
//!
//! # fn main() -> Result<(), AssError> {
//!
//! let account = Account::create("https://url-to-storage", "account-name", "secretkey")?;
//!
//! let image_url = account.get_image_url(123)?;
//! let image_data = account.get_image_data(123)?;
//!
//! let file_data = account.upload_file("/data/file.pdf", "/destination")?;
//! let image_data = account.upload_image("/data/image.jpg")?;
//!
//! # Ok(())
//! # }
//!
//! ```

mod account;
mod data;
mod error;

pub use crate::account::Account;
pub use crate::data::AssData;
pub use crate::error::{AssError, AssErrorKind};
