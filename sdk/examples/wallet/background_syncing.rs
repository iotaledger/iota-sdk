// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example, we will sync an account in the background.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --all-features --example background_syncing --release
//! ```

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        request_funds_from_faucet,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    wallet::{ClientOptions, Result, Wallet},
};

// The account aliases used in this example
const ACCOUNT_ALIAS: &str = "logger";
// The wallet database folder
const WALLET_DB_PATH: &str = "./example.walletdb";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create a wallet
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
    let account = if let Ok(account) = wallet.get_account(ACCOUNT_ALIAS).await {
        account
    } else {
        println!("Creating account '{ACCOUNT_ALIAS}'");
        wallet
            .create_account()
            .with_alias(ACCOUNT_ALIAS.to_string())
            .finish()
            .await?
    };
    let addresses = account.addresses().await?;

    // Manually sync to ensure we have the correct funds to start with
    let balance = account.sync(None).await?;
    let funds_before = balance.base_coin().available();
    println!("Funds BEFORE: {funds_before}");

    wallet.start_background_syncing(None, None).await?;
    println!("Started background syncing");

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
    let funds_after = loop {
        if start.elapsed().as_secs() > 60 {
            println!("Timeout: waiting for funds took too long");
            return Ok(());
        };
        // We just query the balance and don't manually sync
        let balance = account.balance().await?;
        let funds_after = balance.base_coin().available();
        if funds_after > funds_before {
            break funds_after;
        } else {
            tokio::time::sleep(instant::Duration::from_secs(2)).await;
        }
    };
    println!("Funds AfTER: {funds_after}");

    wallet.stop_background_syncing().await?;
    println!("Stopped background syncing");

    Ok(())
}
