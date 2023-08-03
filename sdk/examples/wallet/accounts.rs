// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will print the details of two accounts in the wallet. If an account doesn't exist yet it will be
//! created. For the second account it will generate as many addresses as defined in the constant.
//!
//! Make sure there's no `STRONGHOLD_SNAPSHOT_PATH` file and no `WALLET_DB_PATH` folder yet!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example accounts
//! ```

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
        utils::request_funds_from_faucet,
    },
    wallet::{ClientOptions, Result, Wallet},
};

// The number of addresses to generate
const NUM_ADDRESSES_TO_GENERATE: u32 = 5;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    let secret_manager = MnemonicSecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Get or create first account
    let _ = wallet.get_or_create_account("Alice").await?;

    // Get or create second account
    let alias2 = "Bob";
    let account2 = wallet.get_or_create_account("Alice").await?;

    let accounts = wallet.get_accounts().await?;
    println!("WALLET ACCOUNTS:");
    for account in accounts {
        let account = account.details().await;
        println!("- {}", account.alias());
    }

    println!("Generating {NUM_ADDRESSES_TO_GENERATE} addresses for account '{alias2}'...");
    let addresses = account2
        .generate_ed25519_addresses(NUM_ADDRESSES_TO_GENERATE, None)
        .await?;

    let balance = account2.sync(None).await?;
    let funds_before = balance.base_coin().available();
    println!("Current available funds: {funds_before}");

    println!("Requesting funds from faucet...");
    let faucet_response =
        request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), addresses[0].address()).await?;
    println!("Response from faucet: {}", faucet_response.trim_end());

    println!("Waiting for funds (timeout=60s)...");
    // Check for changes to the balance
    let start = std::time::Instant::now();
    let balance = loop {
        if start.elapsed().as_secs() > 60 {
            println!("Timeout: waiting for funds took too long");
            return Ok(());
        };
        let now = tokio::time::Instant::now();
        let balance = account2.sync(None).await?;
        if balance.base_coin().available() > funds_before {
            println!("Account synced in: {:.2?}", now.elapsed());
            break balance;
        } else {
            tokio::time::sleep(instant::Duration::from_secs(2)).await;
        }
    };

    println!("New available funds: {}", balance.base_coin().available());

    let addresses = account2.addresses().await?;
    println!("Number of addresses in {alias2}'s account: {}", addresses.len());
    println!("{alias2}'s base coin balance:\n{:#?}", balance.base_coin());

    Ok(())
}
