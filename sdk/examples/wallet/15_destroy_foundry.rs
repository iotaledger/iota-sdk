// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will destroy an existing foundry output. This is only possible if its circulating supply is 0.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example destroy_foundry --release`

use iota_sdk::wallet::{Result, Wallet};

const ACCOUNT: &str = "Alice";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account(ACCOUNT).await?;
    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // Get the first foundry
    if let Some(foundry_id) = balance.foundries().first() {
        println!("Balance before destroying:\n{balance:?}",);

        // Set the stronghold password
        wallet
            .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        let transaction = account.destroy_foundry(*foundry_id, None).await?;
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

        println!("Balance after destroying:\n{balance:?}",);
    }

    Ok(())
}
