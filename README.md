[![Build Status](https://travis-ci.com/mipli/ass-rs.svg?branch=master)](https://travis-ci.com/mipli/ass-rs)
[![Crate](https://img.shields.io/crates/v/ass-rs.svg)](https://crates.io/crates/ass-rs)
[![API](https://docs.rs/ass-rs/badge.svg)](https://docs.rs/ass-rs)

# ASS-rs

A library for working with Aptoma Smooth Storage.

## Examples
```rust
use ass_rs::{AssClient, AssError, file_handling, image_handling};

let ass_client = AssClient::create("https://url-to-storage", "account-name", "secretkey")?;

let image_url = image_handling::get_image_url(&ass_client, 123)?;
let image_data = image_handling::get_image_data(&ass_client, 123)?;

let file_data = file_handling::upload_file(&ass_client, "/data/file.pdf", "/destination")?;
let image_data = file_handling::upload_image(&ass_client, "/data/image.jpg")?;
```
