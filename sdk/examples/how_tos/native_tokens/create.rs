// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create a native token.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example create_native_token
//! ```

use std::env::var;

use iota_sdk::{
    wallet::{CreateNativeTokenParams, Result},
    Wallet, U256,
};

// The circulating supply of the native token
const CIRCULATING_SUPPLY: u64 = 100;
// The maximum supply of the native token
const MAXIMUM_SUPPLY: u64 = 100;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;
    let balance = account.sync(None).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // We can first check if we already have an alias in our account, because an alias can have many foundry outputs and
    // therefore we can reuse an existing one
    if balance.aliases().is_empty() {
        // If we don't have an alias, we need to create one
        let transaction = account.create_alias_output(None, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        // Wait for transaction to get included
        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;
        println!("Block included: {}/block/{}", var("EXPLORER_URL").unwrap(), block_id);

        account.sync(None).await?;
        println!("Account synced");
    }

    println!("Preparing transaction to create native token...");

    let params = CreateNativeTokenParams {
        alias_id: None,
        circulating_supply: U256::from(CIRCULATING_SUPPLY),
        maximum_supply: U256::from(MAXIMUM_SUPPLY),
        foundry_metadata: None,
    };

    let transaction = account.create_native_token(params, None).await?;
    println!("Transaction sent: {}", transaction.transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction.transaction_id, None, None)
        .await?;
    println!("Block included: {}/block/{}", var("EXPLORER_URL").unwrap(), block_id);
    println!("Created token: {}", transaction.token_id);

    // Ensure the account is synced after creating the native token.
    account.sync(None).await?;
    println!("Account synced");

    Ok(())
}
