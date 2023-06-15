// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will melt an existing native token with its foundry.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example decrease_native_token_supply
//! ```

use std::env::var;

use iota_sdk::{types::block::output::TokenId, wallet::Result, Wallet, U256};

// The amount of native tokens to melt
const MELT_AMOUNT: u64 = 10;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    // May want to ensure the account is synced before sending a transaction.
    account.sync(None).await?;

    let balance = account.balance().await?;

    // Find first foundry and corresponding token id
    let token_id = TokenId::from(*balance.foundries().first().unwrap());

    if let Some(native_token_balance) = balance
        .native_tokens()
        .iter()
        .find(|native_token| native_token.token_id() == &token_id)
    {
        let available_balance = native_token_balance.available();
        println!("Balance before melting: {available_balance}");
    } else {
        println!("Couldn't find native token '{token_id}' in the account");
        return Ok(());
    }

    // Set the stronghold password
    wallet
        .set_stronghold_password(var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Melt some of the circulating supply
    let melt_amount = U256::from(MELT_AMOUNT);
    let transaction = account
        .decrease_native_token_supply(token_id, melt_amount, None)
        .await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    println!("Block included: {}/block/{}", var("EXPLORER_URL").unwrap(), block_id);

    let balance = account.sync(None).await?;
    let available_balance = balance
        .native_tokens()
        .iter()
        .find(|t| t.token_id() == &token_id)
        .unwrap()
        .available();
    println!("Balance after melting: {available_balance}",);

    Ok(())
}
