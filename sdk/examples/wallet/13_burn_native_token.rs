// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will burn an existing native token, this will not increase the melted supply in the foundry,
//! therefore the foundry output is also not required. But this will also make it impossible to destroy the foundry
//! output that minted it.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example burn_native_token --release`

use iota_sdk::wallet::{Result, Wallet, U256};

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

    // Get a token with sufficient balance
    if let Some(token_id) = balance
        .native_tokens()
        .iter()
        .find(|t| t.available() >= U256::from(11))
        .map(|t| t.token_id())
    {
        println!("Balance before burning:\n{balance:?}",);

        // Set the stronghold password
        wallet
            .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        // Burn a native token
        let burn_amount = U256::from(1);
        let transaction = account.burn_native_token(*token_id, burn_amount, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;

        println!(
            "Block included: {}/block/{}",
            &std::env::var("EXPLORER_URL").unwrap(),
            block_id
        );

        let balance = account.sync(None).await?;

        println!("Balance after burning:\n{balance:?}",);
    }

    Ok(())
}
