// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we request funds from the faucet to the first address in the account.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_wallet` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --all-features --example get_funds --release
//! ```

use iota_sdk::{
    client::request_funds_from_faucet,
    wallet::{Result, Wallet},
};

// The account alias used in this example
const ACCOUNT_ALIAS: &str = "Alice";
// The wallet database folder
const WALLET_DB_PATH: &str = "./example.walletdb";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Access the wallet we generated with `--example create_wallet`
    let wallet = Wallet::builder().with_storage_path(WALLET_DB_PATH).finish().await?;
    let account = wallet.get_account(ACCOUNT_ALIAS).await?;
    let balance = account.sync(None).await?;

    let address = account.addresses().await?;

    let funds_before = balance.base_coin().available();

    println!("Starting available funds: {funds_before}");
    println!("Requesting funds from faucet...");
    let faucet_response =
        request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), &address[0].address().to_string()).await?;

    println!("Response from faucet: {}", faucet_response.trim_end());

    println!("Waiting for funds (timeout=60s)...");
    // Check for changes to the balance
    let start = std::time::Instant::now();
    let balance = loop {
        if start.elapsed().as_secs() > 60 {
            println!("Timeout: waiting for funds took too long");
            return Ok(());
        };
        let balance = account.sync(None).await?;
        if balance.base_coin().available() > funds_before {
            break balance;
        } else {
            tokio::time::sleep(instant::Duration::from_secs(2)).await;
        }
    };

    println!("New available funds: {}", balance.base_coin().available());

    Ok(())
}
