// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send an amount below the minimum storage deposit.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_account.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example send_micro_transaction
//! ```

use iota_sdk::{
    wallet::{account::TransactionOptions, Result},
    Wallet,
};

// The base coin micro amount to send
const SEND_MICRO_AMOUNT: u64 = 1;
// The address to send the coins to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_alias("Alice")
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // May want to ensure the account is synced before sending a transaction.
    wallet.sync(None).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    println!("Sending '{}' coin(s) to '{}'...", SEND_MICRO_AMOUNT, RECV_ADDRESS);

    // Send a micro transaction
    let transaction = wallet
        .send(
            SEND_MICRO_AMOUNT,
            RECV_ADDRESS,
            TransactionOptions {
                allow_micro_amount: true,
                ..Default::default()
            },
        )
        .await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    Ok(())
}
