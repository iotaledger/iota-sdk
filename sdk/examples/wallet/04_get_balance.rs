// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we sync the account and get the balance.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_wallet` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example get_balance
//! ```

use std::env::var;

use iota_sdk::wallet::{Result, Wallet};

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

    // Sync and get the balance
    let _ = account.sync(None).await?;
    // If already synced, just get the balance
    let balance = account.balance().await?;
    println!("{balance:#?}");

    println!("ADDRESSES:");
    let explorer_url = var("EXPLORER_URL").ok();
    let prepended = explorer_url.map(|url| format!("{url}/addr/")).unwrap_or_default();
    for address in account.addresses().await? {
        println!(" - {prepended}{}", address.address());
    }
    Ok(())
}
