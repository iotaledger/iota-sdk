// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will claim all claimable outputs.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example claim_transaction`

use iota_sdk::{
    wallet::{account::OutputsToClaim, Result},
    Wallet,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().with_alias("Alice").finish().await?;

    // May want to ensure the account is synced before sending a transaction.
    wallet.sync(None).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let output_ids = wallet.claimable_outputs(OutputsToClaim::All).await?;
    println!("Available outputs to claim:");
    for output_id in &output_ids {
        println!("{}", output_id);
    }

    let transaction = wallet.claim_outputs(output_ids).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    Ok(())
}
