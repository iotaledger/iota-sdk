// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will list all transaction of a wallet.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example list_transactions
//! ```

use iota_sdk::{
    wallet::{Result, SyncOptions},
    Wallet,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    #[allow(clippy::single_element_loop)]
    for var in ["WALLET_DB_PATH"] {
        std::env::var(var).unwrap_or_else(|_| panic!(".env variable '{var}' is undefined, see .env.example"));
    }

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // Sync wallet
    wallet
        .sync(Some(SyncOptions {
            sync_incoming_transactions: true,
            ..Default::default()
        }))
        .await?;

    // Print transaction ids
    println!("Sent transactions:");
    for transaction in wallet.transactions().await {
        println!("{}", transaction.transaction_id);
    }

    // Print received transaction ids
    println!("Received transactions:");
    for transaction in wallet.incoming_transactions().await {
        println!("{}", transaction.transaction_id);
    }

    Ok(())
}
