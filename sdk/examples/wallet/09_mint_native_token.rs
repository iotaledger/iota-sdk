// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint a native token.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example mint_native_token --release`

use iota_sdk::wallet::{NativeTokenOptions, Result, Wallet, U256};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    println!("Preparing alias output transaction...");

    // First create an alias output, this needs to be done only once, because an alias can have many foundry outputs
    let transaction = account.create_alias_output(None, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        &std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    account.sync(None).await?;
    println!("Account synced");

    println!("Preparing minting transaction...");

    let native_token_options = NativeTokenOptions {
        alias_id: None,
        circulating_supply: U256::from(100),
        maximum_supply: U256::from(100),
        foundry_metadata: None,
    };

    let mint_txn = account.mint_native_token(native_token_options, None).await?;
    println!("Transaction sent: {}", mint_txn.transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&mint_txn.transaction.transaction_id, None, None)
        .await?;

    println!(
        "Block included: {}/block/{}",
        &std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    // Ensure the account is synced after minting.
    account.sync(None).await?;
    println!("Account synced");

    Ok(())
}
