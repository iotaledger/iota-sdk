// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will claim all claimable outputs.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example claim_transaction`

use iota_sdk::wallet::{account::OutputsToClaim, Result, Wallet};

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
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let output_ids = account
        .get_unlockable_outputs_with_additional_unlock_conditions(OutputsToClaim::All)
        .await?;
    println!("Available outputs to claim:");
    for output_id in &output_ids {
        println!("{}", output_id);
    }

    let transaction = account.claim_outputs(output_ids).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    Ok(())
}
