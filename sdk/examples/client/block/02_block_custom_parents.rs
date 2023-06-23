// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example sends a block, with custom parents, which can be used for promoting.
//!
//! `cargo run --example block_custom_parents --release -- [NODE URL]`

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

    // Use tips as custom parents.
    let parents = client.get_tips().await?;

    // Create and send the block with custom strong parents.
    let block = client.block().with_strong_parents(parents)?.finish().await?;

    println!("{block:#?}");

    println!(
        "Block with custom parents sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
