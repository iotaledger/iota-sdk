// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we request funds from the faucet to the first address in the account.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_account.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example request_funds
//! ```

use iota_sdk::{client::request_funds_from_faucet, wallet::Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .with_alias("Alice")
        .finish()
        .await?;

    let balance = wallet.sync(None).await?;
    println!("Wallet synced");

    let bech32_address = wallet.address().await;

    let funds_before = balance.base_coin().available();
    println!("Current available funds: {funds_before}");

    println!("Requesting funds from faucet...");
    let faucet_response = request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), &bech32_address).await?;

    println!("Response from faucet: {}", faucet_response.trim_end());

    println!("Waiting for funds (timeout=60s)...");
    // Check for changes to the balance
    let start = std::time::Instant::now();
    let funds_after = loop {
        if start.elapsed().as_secs() > 60 {
            println!("Timeout: waiting for funds took too long");
            return Ok(());
        };
        let balance = wallet.sync(None).await?;
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
