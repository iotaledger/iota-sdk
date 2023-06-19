// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to list the generated addresses of an account.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example client_list_addresses
//! ```

use std::env;

use iota_sdk::client::{
    api::GetAddressesOptions,
    secret::{GenerateAddressOptions, SecretManager},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create a client instance.
    let client = Client::builder()
        .with_node(&env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let secret_manager =
        SecretManager::try_from_mnemonic(env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    // Generate addresses with default account index and range
    let addresses = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?)
        .await?;

    println!("List of generated public addresses (default):");
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

    println!("List of generated public addresses (0..4):\n");
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
