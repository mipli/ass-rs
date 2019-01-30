[![Build Status](https://travis-ci.com/mipli/ass-rs.svg?branch=master)](https://travis-ci.com/mipli/ass-rs)
[![Crate](https://img.shields.io/crates/v/ass-rs.svg)](https://crates.io/crates/ass-rs)
[![API](https://docs.rs/ass-rs/badge.svg)](https://docs.rs/ass-rs)

# ASS-rs

A library for working with Aptoma Smooth Storage.

## Examples
```rust
use ass_rs::{Account, AssError};

let account = Account::create("https://url-to-storage", "account-name", "secretkey")?;

let image_url = account.get_image_url(123)?;
let image_data = account.get_image_data(123)?;

let file_data = account.upload_file("/data/file.pdf", "/destination")?;
let image_data = account.upload_image("/data/image.jpg")?;
```