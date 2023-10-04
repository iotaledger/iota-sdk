// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example sends a block with no payload.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example block_no_payload
//! ```

use crypto::keys::bip44::Bip44;
use iota_sdk::client::{
    constants::IOTA_COIN_TYPE,
    secret::{SecretManager, SignBlock},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    // Create and send the block.
    let block = client
        .basic_block_builder(todo!("issuer id"), todo!("issuing time"), None, None)
        .await?
        .sign_ed25519(&secret_manager, Bip44::new(IOTA_COIN_TYPE))
        .await?;

    println!("{block:#?}");

    println!(
        "Block with no payload sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        client.block_id(&block).await?
    );

    Ok(())
}
