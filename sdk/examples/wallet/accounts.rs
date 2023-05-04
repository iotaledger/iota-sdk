// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will print the details of two accounts in the wallet. If an account doesn't exist yet it will be
//! created. For the second account it will generate as many addresses as defined in the constant.
//!
//! Make sure there's no `example.stronghold` file and no `example.walletdb` folder yet!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --all-features --example accounts --release
//! ```

use std::time::Instant;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
        utils::request_funds_from_faucet,
    },
    wallet::{ClientOptions, Result, Wallet},
};

// The account aliases used in this example
const ACCOUNT_ALIAS_1: &str = "Alice";
const ACCOUNT_ALIAS_2: &str = "Bob";
// The wallet database folder
const WALLET_DB_PATH: &str = "./example.walletdb";
// The number of addresses to generate
const NUM_ADDRESSES_TO_GENERATE: u32 = 5;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path(WALLET_DB_PATH)
        .finish()
        .await?;

    // Get or create first account
    let _account1 = if let Ok(account) = wallet.get_account(ACCOUNT_ALIAS_1).await {
        account
    } else {
        println!("Creating account '{ACCOUNT_ALIAS_1}'");
        wallet
            .create_account()
            .with_alias(ACCOUNT_ALIAS_1.to_string())
            .finish()
            .await?
    };

    // Get or create second account
    let account2 = if let Ok(account) = wallet.get_account(ACCOUNT_ALIAS_2).await {
        account
    } else {
        println!("Creating account '{ACCOUNT_ALIAS_2}'");
        wallet
            .create_account()
            .with_alias(ACCOUNT_ALIAS_2.to_string())
            .finish()
            .await?
    };

    let accounts = wallet.get_accounts().await?;
    println!("WALLET ACCOUNTS:");
    for account in accounts {
        let account = account.read().await;
        println!("- {}", account.alias());
    }

    println!("Generating {NUM_ADDRESSES_TO_GENERATE} addresses for account '{ACCOUNT_ALIAS_2}'...");
    let addresses = account2.generate_addresses(NUM_ADDRESSES_TO_GENERATE, None).await?;

    let balance = account2.sync(None).await?;
    let funds_before = balance.base_coin().available();

    println!("Requesting funds from faucet...");
    let faucet_response = request_funds_from_faucet(
        &std::env::var("FAUCET_URL").unwrap(),
        &addresses[0].address().to_string(),
    )
    .await?;
    println!("Response from faucet: {}", faucet_response.trim_end());

    println!("Waiting for funds (timeout=60s)...");
    // Check for changes to the balance
    let start = std::time::Instant::now();
    let balance = loop {
        if start.elapsed().as_secs() > 60 {
            println!("Timeout: waiting for funds took too long");
            return Ok(());
        };
        let now = Instant::now();
        let balance = account2.sync(None).await?;
        if balance.base_coin().available() > funds_before {
            println!("Syncing took: {:.2?}", now.elapsed());
            break balance;
        } else {
            tokio::time::sleep(instant::Duration::from_secs(2)).await;
        }
    };

    println!("New available funds: {}", balance.base_coin().available());

    let addresses = account2.addresses().await?;
    println!(
        "Number of addresses in {ACCOUNT_ALIAS_2}'s account: {}",
        addresses.len()
    );
    println!("{ACCOUNT_ALIAS_2}'s base coin balance:\n{:#?}", balance.base_coin());

    Ok(())
}
