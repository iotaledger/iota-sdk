// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example sends a block with a tagged data payload.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example block_tagged_data [TAG] [DATA]
//! ```

use std::env;

use iota_sdk::{
    client::{Client, Result},
    types::block::payload::Payload,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = env::var("NODE_URL").unwrap();

    let mut args = env::args().skip(1);
    let tag = args.next().unwrap_or_else(|| "Hello".to_string());
    let data = args.next().unwrap_or_else(|| "Tangle".to_string());

    // Create a client with that node.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Create and send the block with tag and data.
    let block = client
        .block()
        .with_tag(tag.as_bytes().to_vec())
        .with_data(data.as_bytes().to_vec())
        .finish()
        .await?;

    println!("{block:#?}\n");

    if let Some(Payload::TaggedData(payload)) = block.payload() {
        println!(
            "Tag: {}",
            String::from_utf8(payload.tag().to_vec()).expect("found invalid UTF-8")
        );
        println!(
            "Data: {}",
            String::from_utf8(payload.data().to_vec()).expect("found invalid UTF-8")
        );
    }

    println!(
        "Block with tag and data sent: {}/block/{}",
        env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
