// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will issue a simple base coin transaction.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example simple_transaction
//! ```

use std::env::var;

use iota_sdk::wallet::{Result, SendAmountParams, Wallet};

// The base coin amount to send
const SEND_AMOUNT: u64 = 1_000_000;
// The address to send the coins to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = wallet.sync(None).await?;

    if balance.base_coin().available() >= SEND_AMOUNT {
        // Set the stronghold password
        wallet
            .set_stronghold_password(&var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        println!("Sending '{}' coins to '{}'...", SEND_AMOUNT, RECV_ADDRESS);
        // Send a transaction
        let outputs = [SendAmountParams::new(RECV_ADDRESS, SEND_AMOUNT)?];
        let transaction = account.send_amount(outputs, None).await?;

        // Wait for transaction to get included
        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;

        println!(
            "Transaction included: {}/block/{}",
            var("EXPLORER_URL").unwrap(),
            block_id
        );
    } else {
        println!("Insufficient base coin funds");
    }

    Ok(())
}
