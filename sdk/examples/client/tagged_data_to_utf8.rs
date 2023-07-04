// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will UTF-8 encode the tag and the data of an `TaggedDataPayload`.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example tagged_data_to_utf8
//! ```

use iota_sdk::{
    client::{Client, Result},
    types::block::payload::TaggedDataPayload,
};

#[tokio::main]
async fn main() -> Result<()> {
    // `hello` in hexadecimal.
    let tag = prefix_hex::decode::<Vec<u8>>("0x68656c6c6f")?;
    // `world` in hexadecimal.
    let data = prefix_hex::decode::<Vec<u8>>("0x776f726c64")?;

    let (tag_utf8, data_utf8) = Client::tagged_data_to_utf8(&TaggedDataPayload::new(tag, data)?)?;

    println!("tag: {tag_utf8}\ndata: {data_utf8}");

    Ok(())
}
