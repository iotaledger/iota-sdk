// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send an amount below the minimum storage deposit.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example send_micro_transaction --release`

use iota_sdk::wallet::{account::TransactionOptions, Result, SendAmountParams, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;
    // May want to ensure the account is synced before sending a transaction.
    account.sync(None).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Send a micro transaction with amount 1
    let outputs = vec![SendAmountParams::new(
        "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu".to_string(),
        1,
    )];

    let transaction = account
        .send_amount(
            outputs,
            TransactionOptions {
                allow_micro_amount: true,
                ..Default::default()
            },
        )
        .await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    Ok(())
}
