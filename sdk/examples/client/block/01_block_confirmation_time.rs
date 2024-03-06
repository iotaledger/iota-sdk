// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example sends a block and returns the time at which it got confirmed.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example block_confirmation_time
//! ```

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        constants::IOTA_COIN_TYPE,
        secret::{SecretManager, SignBlock},
        Client,
    },
    types::{api::core::BlockState, block::output::AccountId},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    let node_url = std::env::var("NODE_URL").unwrap();
    let issuer_id = std::env::var("ISSUER_ID").unwrap().parse::<AccountId>().unwrap();

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    // Create and send a block.
    let block = client
        .build_basic_block(issuer_id, None)
        .await?
        .sign_ed25519(&secret_manager, Bip44::new(IOTA_COIN_TYPE))
        .await?;
    let block_id = client.block_id(&block).await?;

    println!("{block:#?}");

    // Wait for the block to get included
    for _ in 0..30 {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        let metadata = client.get_block_metadata(&block_id).await?;
        if let BlockState::Confirmed | BlockState::Finalized = metadata.block_state {
            break;
        }
    }

    println!(
        "Block with no payload included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    Ok(())
}
