// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: Example description
//!
//! `cargo run --release --all-features --example request_funds`

use iota_sdk::{
    client::utils::request_funds_from_faucet,
    wallet::{Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let faucet_url = std::env::var("FAUCET_URL").unwrap();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    let address = account.addresses().await?[0].address().to_string();
    println!("{address}");

    let faucet_response = request_funds_from_faucet(&faucet_url, &address).await?;

    println!("{faucet_response}");
    Ok(())
}
