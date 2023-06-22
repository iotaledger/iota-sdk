// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: Example description
//!
//! `cargo run --example generate_addresses --release -- [NODE_URL]`

use iota_sdk::client::{
    api::GetAddressesOptions,
    secret::{GenerateAddressOptions, SecretManager},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| std::env::var("NODE_URL").unwrap());

    // Create a client instance
    let client = Client::builder()
        .with_node(&node_url)? // Insert your node URL here
        .finish()
        .await?;

    let secret_manager =
        SecretManager::try_from_mnemonic(std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    // Generate addresses with default account index and range
    let addresses = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?)
        .await?;

    println!("List of generated public addresses:");
    println!("{addresses:#?}\n");

    // Generate addresses with custom account index and range
    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::from_client(&client)
                .await?
                .with_account_index(0)
                .with_range(0..4),
        )
        .await?;

    println!("List of generated public addresses:\n");
    println!("{addresses:#?}\n");

    // Generate internal addresses with custom account index and range
    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::from_client(&client)
                .await?
                .with_account_index(0)
                .with_range(0..4)
                .with_options(GenerateAddressOptions::internal()),
        )
        .await?;

    println!("List of generated internal addresses:\n");
    println!("{addresses:#?}\n");

    Ok(())
}
