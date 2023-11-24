// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will list all transaction of an account.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example list_transactions
//! ```

use iota_sdk::{
    wallet::{account::SyncOptions, Result},
    Wallet,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH"] {
        if std::env::var(var).is_err() {
            panic!(".env variable '{}' is undefined, see .env.example", var);
        }
    }

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
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
    for transaction in account.transactions().await {
        println!("{}", transaction.transaction_id);
    }

    // Print received transaction ids
    println!("Received transactions:");
    for transaction in account.incoming_transactions().await {
        println!("{}", transaction.transaction_id);
    }

    Ok(())
}
