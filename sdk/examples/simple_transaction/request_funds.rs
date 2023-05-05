// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: Example description
//!
//! `cargo run --example request_funds --release`

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
    println!("Account ID");

    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let address = account.generate_addresses(1, None).await?;
    println!("Generated address: {}", address[0].address());

    let faucet_response = request_funds_from_faucet(&faucet_url, &address[0].address().to_string()).await?;

    println!("{faucet_response}");
    Ok(())
}
