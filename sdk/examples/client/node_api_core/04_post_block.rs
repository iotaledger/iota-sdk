// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Submits a block as a JSON payload using the `/api/core/v3/blocks` node endpoint.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_post_block [NODE URL]
//! ```

use crypto::keys::bip44::Bip44;
use iota_sdk::client::{
    constants::IOTA_COIN_TYPE,
    secret::{SecretManager, SignBlock},
    Client, Result,
};

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
        .basic_block_builder(todo!("issuer id"), todo!("issuing time"), None, None)
        .await?
        .sign_ed25519(&secret_manager, Bip44::new(IOTA_COIN_TYPE))
        .await?;
    // Post the block.
    let block_id = client.post_block(&block).await?;

    println!(
        "Posted block: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    Ok(())
}
