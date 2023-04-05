// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example get_funds --release
// In this example we request funds from the faucet to our address
// Rename `.env.example` to `.env` first

use std::env;

use dotenv::dotenv;
use iota_sdk::{
    client::request_funds_from_faucet,
    wallet::{account_manager::AccountManager, Result},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    // Create the account manager
    let manager = AccountManager::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = manager.get_account("Alice").await?;
    let balance = account.sync(None).await?;

    let address = account.addresses().await?;

    let funds_before = balance.base_coin.available;

    println!("Starting available funds: {funds_before}");

    let faucet_response =
        request_funds_from_faucet(&env::var("FAUCET_URL").unwrap(), &address[0].address().to_bech32()).await?;

    println!("Response from faucet: {faucet_response}");

    println!("Waiting for funds...");
    // Check for changes to the balance
    let start = std::time::Instant::now();
    let balance = loop {
        if start.elapsed().as_secs() > 60 {
            panic!("took too long waiting for funds")
        };
        let balance = account.sync(None).await?;
        if balance.base_coin.available > funds_before {
            break balance;
        }
    };

    println!("New available funds: {}", balance.base_coin.available);

    Ok(())
}
