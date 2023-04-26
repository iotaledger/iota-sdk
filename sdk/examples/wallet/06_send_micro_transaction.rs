// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send an amount below the minimum storage deposit.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example send_micro_transaction --release`

use iota_sdk::wallet::{account::TransactionOptions, AddressWithAmount, Result, Wallet};

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
    let outputs = vec![AddressWithAmount::new(
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

    // Wait for transaction to get included
    account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    println!("Transaction: {}", transaction.transaction_id);
    println!(
        "Block sent: {}/block/{}",
        &std::env::var("EXPLORER_URL").unwrap(),
        transaction.block_id.expect("no block created yet")
    );

    Ok(())
}
