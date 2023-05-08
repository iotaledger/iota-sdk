// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will verifiy the integrity of the wallet database.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example storage
//! ```

use std::time::Instant;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    wallet::{account::types::AccountAddress, Account, ClientOptions, Result, Wallet},
};

// The account alias used in this example
const ACCOUNT_ALIAS: &str = "Alice";
// The wallet database folder created in this example
const WALLET_DB_PATH: &str = "./example.walletdb";
// The maximum number of addresses to generate
const MAX_ADDRESSES_TO_GENERATE: usize = 3;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_storage_path(WALLET_DB_PATH)
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Get account or create a new one
    let account = get_or_create_account(&wallet, ACCOUNT_ALIAS).await?;

    let addresses = generate_max_addresses(&account, ACCOUNT_ALIAS, MAX_ADDRESSES_TO_GENERATE).await?;
    let bech32_addresses = addresses
        .iter()
        .map(|address| address.address().to_string())
        .collect::<Vec<_>>();

    println!("Total address count:\n{:?}", account.addresses().await?.len());
    println!("ADDRESSES:\n{bech32_addresses:#?}");

    sync_print_balance(&account, ACCOUNT_ALIAS).await?;

    #[cfg(debug_assertions)]
    wallet.verify_integrity().await?;

    println!("Example finished successfully");
    Ok(())
}

async fn get_or_create_account(wallet: &Wallet, alias: &str) -> Result<Account> {
    Ok(if let Ok(account) = wallet.get_account(alias).await {
        account
    } else {
        println!("Creating account '{alias}'");
        wallet.create_account().with_alias(alias.to_string()).finish().await?
    })
}

async fn generate_max_addresses(account: &Account, alias: &str, max: usize) -> Result<Vec<AccountAddress>> {
    if account.addresses().await?.len() < max {
        let num_addresses_to_generate = max - account.addresses().await?.len();
        println!("Generating {num_addresses_to_generate} addresses for account '{alias}'...");
        account
            .generate_addresses(num_addresses_to_generate as u32, None)
            .await?;
    }
    account.addresses().await
}

async fn sync_print_balance(account: &Account, alias: &str) -> Result<()> {
    let now = Instant::now();
    let balance = account.sync(None).await?;
    println!("{alias}'s account synced in: {:.2?}", now.elapsed());
    println!("{alias}'s balance:\n{:#?}", balance.base_coin());
    Ok(())
}
