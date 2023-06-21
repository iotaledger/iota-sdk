// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint a native token.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example mint_native_token
//! ```

use iota_sdk::{
    wallet::{MintNativeTokenParams, Result},
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
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    println!("Sending alias output transaction...");

    // First create an alias output, this needs to be done only once, because an alias can have many foundry outputs
    let transaction = account.create_alias_output(None, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Transaction included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    account.sync(None).await?;
    println!("Account synced");

    println!("Sending the minting transaction...");

    let params = MintNativeTokenParams {
        alias_id: None,
        circulating_supply: U256::from(CIRCULATING_SUPPLY),
        maximum_supply: U256::from(MAXIMUM_SUPPLY),
        foundry_metadata: None,
    };

    let transaction = account.mint_native_token(params, None).await?;
    println!("Transaction sent: {}", transaction.transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction.transaction_id, None, None)
        .await?;
    println!(
        "Transaction included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );
    println!("Minted token: {} ", transaction.token_id);

    // Ensure the account is synced after minting.
    account.sync(None).await?;
    println!("Account synced");

    Ok(())
}
