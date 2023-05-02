// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we sync the account and get the balance.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example get_balance --release`

use iota_sdk::wallet::{Result, Wallet};

const ACCOUNT: &str = "Alice";

#[tokio::main]
async fn main() -> Result<()> {
    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account(ACCOUNT).await?;

    // Sync and get the balance
    let _account_balance = account.sync(None).await?;
    // If already synced, just get the balance
    let account_balance = account.balance().await?;

    println!("{account_balance:#?}");

    let explorer_url = std::env::var("EXPLORER_URL").ok();
    let prepended = explorer_url.map(|url| format!("{url}/addr/")).unwrap_or_default();

    println!("Addresses:");
    for address in account.addresses().await? {
        println!(" - {prepended}{}", address.address());
    }

    Ok(())
}
