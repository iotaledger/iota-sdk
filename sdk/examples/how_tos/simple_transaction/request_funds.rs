// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we request funds from the faucet to the first address in the account.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example request_funds
//! ```

use std::env::var;

use iota_sdk::{
    client::request_funds_from_faucet,
    wallet::{Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Access the wallet we generated with `--example create_wallet`
    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account(&var("ACCOUNT_ALIAS_1").unwrap()).await?;

    let balance = account.sync(None).await?;
    println!("Account synced");

    let addresses = account.addresses().await?;

    let funds_before = balance.base_coin().available();
    println!("Current available funds: {funds_before}");

    println!("Requesting funds from faucet...");
    let faucet_response = request_funds_from_faucet(&var("FAUCET_URL").unwrap(), addresses[0].address()).await?;

    println!("Response from faucet: {}", faucet_response.trim_end());

    println!("Waiting for funds (timeout=60s)...");
    // Check for changes to the balance
    let start = std::time::Instant::now();
    let funds_after = loop {
        if start.elapsed().as_secs() > 60 {
            println!("Timeout: waiting for funds took too long");
            return Ok(());
        };
        let balance = account.sync(None).await?;
        let funds_after = balance.base_coin().available();
        if funds_after > funds_before {
            break funds_after;
        } else {
            tokio::time::sleep(instant::Duration::from_secs(2)).await;
        }
    };
    println!("New available funds: {funds_after}");

    Ok(())
}
