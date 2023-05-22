// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will list all transaction of an account.
//!
//! `cargo run --release --all-features --example list_transactions`

use iota_sdk::wallet::{account::SyncOptions, Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account
    let account = wallet.get_account("Alice").await?;

    // Sync account
    account
        .sync(Some(SyncOptions {
            sync_incoming_transactions: true,
            ..Default::default()
        }))
        .await?;

    // Print transaction ids
    println!("Sent transactions:");
    for transaction in account.transactions().await? {
        println!("{}", transaction.transaction_id);
    }

    // Print received transaction ids
    println!("Received transactions:");
    for transaction_id in account.incoming_transactions().await?.keys() {
        println!("{}", transaction_id);
    }

    Ok(())
}
