// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example, we'll create a new wallet, a mnemonic, and an initial account. Then, we'll print the first address
//! of that account.
//!
//! Make sure there's no `example.stronghold` file and no `example.walletdb` folder yet!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --all-features --example wallet_getting_started --release
//! ```

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
    },
    wallet::{ClientOptions, Result, Wallet},
};

// The account alias created in this example
const ACCOUNT_ALIAS: &str = "Alice";
// The Stronghold snapshot file created in this example
const STRONGHOLD_SNAPSHOT_PATH: &str = "./example.stronghold";
// The wallet database folder created in this example
const WALLET_DB_PATH: &str = "./example.walletdb";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Setup Stronghold secret_manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build(STRONGHOLD_SNAPSHOT_PATH)?;

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    // Create the wallet
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(secret_manager))
        .with_storage_path(WALLET_DB_PATH)
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
    let addresses = account.addresses().await?;
    println!("ADDRESSES:\n{:#?}", addresses);

    Ok(())
}
