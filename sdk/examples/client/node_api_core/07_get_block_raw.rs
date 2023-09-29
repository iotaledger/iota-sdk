// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Returns block data as raw bytes by its identifier by querying the `/api/core/v3/blocks/{blockId}` node endpoint.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_get_block_raw [BLOCK ID] [NODE URL]
//! ```

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args()
        .nth(2)
        .unwrap_or_else(|| std::env::var("NODE_URL").unwrap());

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the block ID from command line argument or...
    let block_id = if let Some(Ok(block_id)) = std::env::args().nth(1).map(|s| s.parse()) {
        block_id
    } else {
        // ... fetch one from the node.
        client.get_issuance().await?.strong_parents.into_iter().next().unwrap()
    };

    // Get the block as raw bytes.
    let block_bytes = client.get_block_raw(&block_id).await?;

    println!("Block bytes:\n{block_bytes:?}");

    Ok(())
}
