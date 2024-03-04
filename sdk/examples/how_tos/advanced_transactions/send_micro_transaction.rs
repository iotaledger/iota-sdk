// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send an amount below the minimum amount.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/wallet/create_wallet.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example send_micro_transaction
//! ```

use iota_sdk::{client::api::options::TransactionOptions, Wallet};

// The base coin micro amount to send
const SEND_MICRO_AMOUNT: u64 = 1;
// The address to send the coins to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH", "STRONGHOLD_PASSWORD", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Get the wallet we generated with `create_wallet`.
    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // May want to ensure the wallet is synced before sending a transaction.
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

    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;

    println!(
        "Tx accepted: {}/transactions/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction.transaction_id
    );

    Ok(())
}
