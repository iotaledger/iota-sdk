// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will recover a wallet from a given mnemonic.
//!
//! Make sure there's no folder yet at `WALLET_DB_PATH`.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example recover_accounts
//! ```

use std::time::Instant;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    wallet::{ClientOptions, Result, Wallet},
};

// The wallet database folder created in this example
const WALLET_DB_PATH: &str = "./example.walletdb";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_storage_path(WALLET_DB_PATH)
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    let accounts = wallet.recover_accounts(0, 2, 2, None).await?;

    println!("Recovered {} accounts", accounts.len());
    for account in accounts.iter() {
        println!("ACCOUNT #{}:", account.read().await.index());
        let now = Instant::now();
        let balance = account.sync(None).await?;
        println!("Account synced in: {:.2?}", now.elapsed());
        println!("Balance: {balance:#?}");
    }

    Ok(())
}
