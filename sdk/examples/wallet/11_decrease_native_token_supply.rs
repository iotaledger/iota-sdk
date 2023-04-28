// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will melt an existing native token with its foundry.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example decrease_native_token_supply --release`

use std::str::FromStr;

use iota_sdk::{
    types::block::output::TokenId,
    wallet::{Result, Wallet, U256},
};

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

    let balance = account.balance().await?;
    println!("Balance before melting:\n{balance:?}",);

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Replace with a TokenId that is available in the account, the foundry output which minted it, also needs to be
    // available.
    let token_id = TokenId::from_str("0x08847bd287c912fadedb6bf38900bda9f2d377b75b2a0bece8738699f56ebca4130100000000")?;

    // Melt some of the circulating supply
    let melt_amount = U256::from(10);
    let transaction = account
        .decrease_native_token_supply(token_id, melt_amount, None)
        .await?;
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

    println!("Balance after melting:\n{balance:?}",);

    Ok(())
}
