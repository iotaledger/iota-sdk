// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Finds the metadata of a given block by calling `GET /api/core/v3/blocks/{blockId}/metadata`.
//!
//! `cargo run --example node_api_core_get_block_metadata --release -- [NODE URL]`

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

    // Fetch a block ID from the node.
    let block_id = client.get_tips().await?[0];
    // Send the request.
    let block_metadata = client.get_block_metadata(&block_id).await?;

    println!("{block_metadata:#?}");

    Ok(())
}
