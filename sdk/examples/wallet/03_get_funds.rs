// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we request funds from the faucet to our address.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example get_funds --release`

use iota_sdk::{
    client::request_funds_from_faucet,
    wallet::{Result, Wallet},
};

const ACCOUNT: &str = "Alice";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account(ACCOUNT).await?;
    let balance = account.sync(None).await?;

    let address = account.addresses().await?;

    let funds_before = balance.base_coin().available();

    println!("Starting available funds: {funds_before}");

    let faucet_response =
        request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), &address[0].address().to_string()).await?;

    println!("Response from faucet: {faucet_response}");

    println!("Waiting for funds...");
    // Check for changes to the balance
    let start = std::time::Instant::now();
    let balance = loop {
        if start.elapsed().as_secs() > 60 {
            panic!("took too long waiting for funds")
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
