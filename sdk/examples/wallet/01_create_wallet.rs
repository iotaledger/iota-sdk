// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create a new wallet.
//!
//! Make sure there's no `example.stronghold` file and no `example.walletdb` folder yet!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --all-features --example create_wallet --release
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

    // Only required the first time, can also be generated with `manager.generate_mnemonic()?`
    let mnemonic = std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap();

    // The mnemonic only needs to be stored the first time
    secret_manager.store_mnemonic(mnemonic).await?;

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    // Create the wallet
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path(WALLET_DB_PATH)
        .finish()
        .await?;

    // Create a new account
    let account = wallet
        .create_account()
        .with_alias(ACCOUNT_ALIAS.to_string())
        .finish()
        .await?;

    println!("Generated new account: '{}'", account.alias().await);

    Ok(())
}
