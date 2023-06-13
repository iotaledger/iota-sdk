// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example sends a block, with custom parents, which can be used for promoting.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example block_custom_parents
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

    // Use tips as custom parents.
    let tips = client.get_tips().await?;
    println!("Custom tips:\n{tips:#?}");

    // Create and send the block with custom parents.
    let block = client.block().with_parents(tips)?.finish().await?;

    println!("{block:#?}");

    println!(
        "Block with custom parents sent: {}/block/{}",
        env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
