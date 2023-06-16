// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Submits a block as a JSON payload using the `/api/core/v2/blocks` node endpoint.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_post_block [NODE URL]
//! ```

use std::env;

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = env::args().nth(1).unwrap_or_else(|| env::var("NODE_URL").unwrap());

    // Create a client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Create the block.
    let block = client.block().finish().await?;

    // Post the block.
    let block_id = client.post_block(&block).await?;

    println!("Posted block: {}/block/{}", env::var("EXPLORER_URL").unwrap(), block_id);

    Ok(())
}
