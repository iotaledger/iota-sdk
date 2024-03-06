// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to generate addresses.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example 01_generate_addresses
//! ```

use iota_sdk::client::{
    api::GetAddressesOptions,
    secret::{GenerateAddressOptions, SecretManager},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL", "MNEMONIC"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Create a node client.
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    // Generate addresses with default account index and range
    let addresses = secret_manager
        .generate_ed25519_addresses_as_bech32(GetAddressesOptions::from_client(&client).await?)
        .await?;

    println!("List of generated public addresses (default):");
    println!("{addresses:#?}\n");

    // Generate addresses with custom account index and range
    let addresses = secret_manager
        .generate_ed25519_addresses_as_bech32(
            GetAddressesOptions::from_client(&client)
                .await?
                .with_account_index(0)
                .with_range(0..4),
        )
        .await?;

    println!("List of generated public addresses (0..4):\n");
    println!("{addresses:#?}\n");

    // Generate internal addresses with custom account index and range
    let addresses = secret_manager
        .generate_ed25519_addresses_as_bech32(
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
