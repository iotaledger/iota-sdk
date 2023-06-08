// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will destroy an existing foundry output. This is only possible if its circulating supply is 0.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example destroy_foundry --release`

use iota_sdk::wallet::{Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;
    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // Get the first foundry
    if let Some(foundry_id) = balance.foundries().first() {
        let foundry_count = balance.foundries().len();
        println!("Foundries before destroying: {foundry_count}");

        // Set the stronghold password
        wallet
            .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        let transaction = account.burn(*foundry_id, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;

        println!(
            "Block included: {}/block/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            block_id
        );

        let balance = account.sync(None).await?;

        let foundry_count = balance.foundries().len();
        println!("Foundries after destroying: {foundry_count}");
    }

    Ok(())
}
