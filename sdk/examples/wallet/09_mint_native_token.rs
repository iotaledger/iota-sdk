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

    account.sync(None).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // First create an alias output, this needs to be done only once, because an alias can have many foundry outputs
    let transaction = account.create_alias_output(None, None).await?;
    println!(
        "Transaction: {} Block sent: {}/api/core/v2/blocks/{}",
        transaction.transaction_id,
        &std::env::var("NODE_URL").unwrap(),
        transaction.block_id.expect("no block created yet")
    );

    // Wait for transaction to get included
    account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    account.sync(None).await?;

    let native_token_options = NativeTokenOptions {
        alias_id: None,
        circulating_supply: U256::from(100),
        maximum_supply: U256::from(100),
        foundry_metadata: None,
    };

    let transaction = account.mint_native_token(native_token_options, None).await?;
    println!(
        "Transaction: {} Block sent: {}/api/core/v2/blocks/{}",
        transaction.transaction.transaction_id,
        &std::env::var("NODE_URL").unwrap(),
        transaction.transaction.block_id.expect("no block created yet")
    );
    Ok(())
}
