// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to generate addresses.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example 01_generate_addresses
//! ```

use iota_sdk::{
    client::{
        constants::IOTA_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, MultiKeyOptions, SecretManageExt},
        Client, Result,
    },
    types::block::address::{Ed25519Address, ToBech32Ext},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL", "MNEMONIC"] {
        std::env::var(var).unwrap_or_else(|_| panic!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Create a node client.
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let secret_manager = MnemonicSecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    let hrp = client.get_bech32_hrp().await?;

    // Generate addresses with default account index and range
    let addresses = secret_manager
        .generate::<Vec<Ed25519Address>>(&MultiKeyOptions::new(IOTA_COIN_TYPE))
        .await?
        .into_iter()
        .map(|a| a.to_bech32(hrp))
        .collect::<Vec<_>>();

    println!("List of generated public addresses (default):");
    println!("{addresses:#?}\n");

    // Generate addresses with custom account index and range
    let addresses = secret_manager
        .generate::<Vec<Ed25519Address>>(
            &MultiKeyOptions::new(IOTA_COIN_TYPE)
                .with_account_index(0)
                .with_address_range(0..4),
        )
        .await?
        .into_iter()
        .map(|a| a.to_bech32(hrp))
        .collect::<Vec<_>>();

    println!("List of generated public addresses (0..4):\n");
    println!("{addresses:#?}\n");

    // Generate internal addresses with custom account index and range
    let addresses = secret_manager
        .generate::<Vec<Ed25519Address>>(
            &MultiKeyOptions::new(IOTA_COIN_TYPE)
                .with_account_index(0)
                .with_address_range(0..4)
                .with_internal(true),
        )
        .await?
        .into_iter()
        .map(|a| a.to_bech32(hrp))
        .collect::<Vec<_>>();

    println!("List of generated internal addresses:\n");
    println!("{addresses:#?}\n");

    Ok(())
}
