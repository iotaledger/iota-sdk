// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we sync the account and get the balance.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example check_balance`

use iota_sdk::wallet::{Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account
    let account = wallet.get_account("Alice").await?;

    // Sync and get the balance
    let _balance = account.sync(None).await?;
    // If already synced, just get the balance
    let balance = account.balance().await?;

    println!("{balance:#?}");

    Ok(())
}
