// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Submits a block as raw bytes using the `/api/core/v3/blocks` node endpoint.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_post_block_raw [NODE URL]
//! ```

use iota_sdk::client::{constants::IOTA_COIN_TYPE, secret::SecretManager, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| std::env::var("NODE_URL").unwrap());

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    // Create the block.
    let block = client
        .finish_basic_block_builder(
            todo!("issuer id"),
            todo!("issuing time"),
            None,
            None,
            IOTA_COIN_TYPE,
            &secret_manager,
        )
        .await?;
    // Post the block as raw bytes.
    let block_id = client.post_block_raw(&block).await?;

    println!(
        "Posted raw block: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    Ok(())
}
