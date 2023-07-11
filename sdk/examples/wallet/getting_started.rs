// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create a new wallet, a mnemonic, and an initial account. Then, we'll print the first address
//! of that account.
//!
//! Make sure there's no `STRONGHOLD_SNAPSHOT_PATH` file and no `WALLET_DB_PATH` folder yet!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example wallet_getting_started
//! ```

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
    },
    wallet::{ClientOptions, Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Setup Stronghold secret_manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build(std::env::var("STRONGHOLD_SNAPSHOT_PATH").unwrap())?;

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    // Create the wallet
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(secret_manager))
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Generate a mnemonic and store it in the Stronghold vault
    // INFO: It is best practice to back up the mnemonic somewhere secure
    let mnemonic = wallet.generate_mnemonic()?;
    wallet.store_mnemonic(mnemonic.clone()).await?;
    println!("Created a wallet from the mnemonic:\n'{}'", mnemonic.as_ref());

    // Create an account
    let alias = "Alice";
    let account = wallet.create_account().with_alias(alias).finish().await?;
    println!("Created account '{alias}'");

    // Display the adresses in the account (only 1 for a new account)
    let addresses = account.addresses().await?;
    println!(
        "{alias}'s addresses:\n{:#?}",
        addresses
            .iter()
            .map(|addr| addr.address().to_string())
            .collect::<Vec<_>>()
    );

    Ok(())
}
