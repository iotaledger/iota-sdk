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

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    let address = account.addresses().await?;

    let faucet_response =
        request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), &address[0].address().to_bech32()).await?;

    println!("{faucet_response}");

    Ok(())
}
