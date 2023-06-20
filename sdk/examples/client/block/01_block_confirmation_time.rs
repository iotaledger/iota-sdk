// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example sends a block and returns the time at which it got confirmed.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example block_confirmation_time
//! ```

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Create and send a block.
    let block = client.block().finish().await?;
    let block_id = block.id();

    println!("{block:#?}");

    // Try to check if the block has been confirmed.
    client.retry_until_included(&block_id, None, None).await?;
    println!(
        "Block with no payload included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    // TODO uncomment when we have a new confirmation logic
    // Get the block metadata.
    // let metadata = client.get_block_metadata(&block_id).await?;

    // if let Some(ms_index) = metadata.referenced_by_milestone_index {
    //     let ms = client.get_milestone_by_index(ms_index).await?;
    //     println!(
    //         "Block {block_id} got confirmed by milestone {ms_index} at timestamp {}.",
    //         ms.essence().timestamp()
    //     );
    // } else {
    //     println!("Block {block_id} is not confirmed.")
    // }

    Ok(())
}
