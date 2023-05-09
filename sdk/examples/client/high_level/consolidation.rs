// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will consolidate all funds in a range of addresses.
//!
//! `cargo run --example consolidation --release`

use iota_sdk::client::{
    api::GetAddressesBuilderOptions,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    // Configure your own mnemonic in ".env". Since the output amount cannot be zero, the mnemonic must contain non-zero
    // balance.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    let address_range = 0u32..150;
    // Create a client instance
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let secret_manager =
        SecretManager::try_from_hex_seed(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?;

    // Here all funds will be send to the address with the lowest index in the range
    let address = client
        .consolidate_funds(
            &secret_manager,
            GetAddressesBuilderOptions {
                range: Some(address_range),
                ..Default::default()
            },
        )
        .await?;

    println!("Funds consolidated to {}", address);
    Ok(())
}
