// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will:
//! 1. create a wallet
//! 2. create and store a mnemonic
//! 3. create an initial account
//! 4. print the first address of that account
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! `cargo run --example getting_started`

use std::path::PathBuf;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
    },
    wallet::{ClientOptions, Result, Wallet},
};

// A name to associate with the created account.
const ACCOUNT_ALIAS: &str = "Alice";
// A password to encrypt the stored data.
// WARNING: Never hardcode passwords in production code.
const STRONGHOLD_PASSWORD: &str = "a-secure-password";
// The path to store the account snapshot.
const STRONGHOLD_SNAPSHOT_PATH: &str = "vault.stronghold";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Setup Stronghold secret manager.
    let secret_manager = StrongholdSecretManager::builder()
        .password(STRONGHOLD_PASSWORD)
        .build(PathBuf::from(STRONGHOLD_SNAPSHOT_PATH))?;

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    // Set up and store the wallet.
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Generate a mnemonic and store it in the Stronghold vault.
    // INFO: It is best practice to back up the mnemonic somewhere secure.
    let mnemonic = wallet.generate_mnemonic()?;
    wallet.store_mnemonic(mnemonic).await?;

    // Create an account.
    let account = wallet
        .create_account()
        .with_alias(ACCOUNT_ALIAS.to_string())
        .finish()
        .await?;

    // Get the first address and print it.
    let address = &account.addresses().await?[0];
    println!("Address:\n{}\n", address.address());

    Ok(())
}
