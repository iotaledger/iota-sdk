// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Submits a block as a JSON payload by calling `POST /api/core/v3/blocks`.
//!
//! `cargo run --example node_api_core_post_block --release -- [NODE URL]`

use iota_sdk::client::{Client, Result};

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

    // Create the block.
    let block = client.block().finish().await?;
    // Post the block.
    let block_id = client.post_block(&block).await?;

    println!(
        "Posted block: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    Ok(())
}
