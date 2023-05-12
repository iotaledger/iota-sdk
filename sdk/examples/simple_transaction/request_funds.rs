// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: Example description
//! 
//! `cargo run --example request_funds --release`

use iota_sdk::client::{secret::SecretManager, utils::request_funds_from_faucet, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();
    let faucet_url = std::env::var("FAUCET_URL").unwrap();

    // Create a client instance
    let client = Client::builder()
        .with_node(&node_url)? // Insert the node here
        .finish()?;

    let secret_manager =
        SecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let addresses = client
        .get_addresses(&secret_manager)
        .with_account_index(1)
        .with_range(0..1)
        .finish()
        .await?;
    println!("{}", addresses[0]);

    let faucet_response = request_funds_from_faucet(&faucet_url, &addresses[0]).await?;

    println!("{faucet_response}");
    Ok(())
}
