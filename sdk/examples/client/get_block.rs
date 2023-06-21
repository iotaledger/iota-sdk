// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will get a block and its metadata.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example get_block
//! ```

use std::env;

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create a node client.
    let client = Client::builder()
        .with_node(&env::var("NODE_URL").unwrap())?
        .with_pow_worker_count(1)
        .finish()
        .await?;

    // Fetch a block ID from the node.
    let block_id = client.get_tips().await?[0];

    // Get the block.
    let block = client.get_block(&block_id).await?;
    println!("{block:#?}");

    let block_metadata = client.get_block_metadata(&block_id).await?;
    println!("{block_metadata:#?}");

    Ok(())
}
