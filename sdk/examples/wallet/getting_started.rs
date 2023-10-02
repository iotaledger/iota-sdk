// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create a new wallet, a mnemonic, and an initial account. Then, we'll print the first address
//! of that account.
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
    // Setup Stronghold secret manager.
    // WARNING: Never hardcode passwords in production code.
    let secret_manager = StrongholdSecretManager::builder()
        .password("password".to_owned()) // A password to encrypt the stored data.
        .build("vault.stronghold")?; // The path to store the account snapshot.

    let client_options = ClientOptions::new().with_node("https://api.testnet.shimmer.network")?;

    // Set up and store the wallet.
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path("getting-started-db")
        .finish()
        .await?;

    // Generate a mnemonic and store its seed in the Stronghold vault.
    // INFO: It is best practice to back up the mnemonic somewhere secure.
    let mnemonic = wallet.generate_mnemonic()?;
    println!("Mnemonic: {}", mnemonic.as_ref());
    wallet.store_mnemonic(mnemonic).await?;

    // Create an account.
    let account = wallet
        .create_account()
        .with_alias("Alice") // A name to associate with the created account.
        .finish()
        .await?;

    let first_address = &account.addresses().await?[0];
    println!("{}", first_address.address());

    Ok(())
}
