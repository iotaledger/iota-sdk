// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Returns block data as raw bytes by its identifier by calling `GET /api/core/v2/blocks/{blockId}`.
//!
//! `cargo run --example node_api_core_get_block_raw --release -- [NODE URL] [BLOCK ID]`

use std::str::FromStr;

use iota_sdk::{
    client::{Client, Result},
    types::block::BlockId,
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
    let client = Client::builder().with_node(&node_url)?.finish()?;

    // Take the block ID from command line argument or...
    let block_id = if let Some(Ok(block_id)) = std::env::args().nth(2).map(|s| BlockId::from_str(&s)) {
        block_id
    } else {
        // ... fetch one from the node.
        client.get_tips().await?[0]
    };

    // Get the block as raw bytes.
    let block_bytes = client.get_block_raw(&block_id).await?;

    println!("Block bytes: {block_bytes:?}");

    Ok(())
}
