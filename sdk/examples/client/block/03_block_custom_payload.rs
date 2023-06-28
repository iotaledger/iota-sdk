// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example sends a block with a custom payload.
//!
//! `cargo run --example block_custom_payload --release -- [NODE URL]`

use iota_sdk::{
    client::{Client, Result},
    types::block::payload::{Payload, TaggedDataPayload},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args().nth(1).unwrap_or_else(|| {
        // This example uses secrets in environment variables for simplicity which should not be done in production.
        dotenvy::dotenv().ok();
        std::env::var("NODE_URL").unwrap()
    });

    // Create a client with that node.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Create a custom payload.
    let tagged_data_payload = TaggedDataPayload::new(b"Your tag".to_vec(), b"Your data".to_vec())?;

    // Create and send the block with the custom payload.
    let block = client
        .finish_block_builder(None, Some(Payload::from(tagged_data_payload)))
        .await?;

    println!("{block:#?}");

    println!(
        "Block with custom payload sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
