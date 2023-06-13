// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Finds the metadata of a given block by calling `GET /api/core/v2/blocks/{blockId}/metadata`.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_get_block_metadata [BLOCK ID] [NODE URL]
//! ```

use std::{env, str::FromStr};

use iota_sdk::{
    client::{Client, Result},
    types::block::BlockId,
};

#[tokio::main]
async fn main() -> Result<()> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = env::args().nth(2).unwrap_or_else(|| env::var("NODE_URL").unwrap());

    // Create a client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the block ID from command line argument or...
    let block_id = if let Some(Ok(block_id)) = env::args().nth(1).map(|s| BlockId::from_str(&s)) {
        block_id
    } else {
        // ... fetch one from the node.
        client.get_tips().await?[0]
    };

    // Send the request.
    let block_metadata = client.get_block_metadata(&block_id).await?;

    println!("{block_metadata:#?}");

    Ok(())
}
