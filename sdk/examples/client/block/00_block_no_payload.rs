// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example sends a block with no payload.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example block_no_payload
//! ```

use std::env;

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = env::var("NODE_URL").unwrap();

    // Create a client with that node.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Create and send the block.
    let block = client.block().finish().await?;

    println!("{block:#?}");

    println!(
        "Block with no payload sent: {}/block/{}",
        env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
