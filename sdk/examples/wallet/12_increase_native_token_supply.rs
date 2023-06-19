// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint an existing native token with its foundry.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example increase_native_token_supply
//! ```

use std::{env::var, str::FromStr};

use iota_sdk::{types::block::output::TokenId, wallet::Result, Wallet, U256};

// The native token id. Replace it with a TokenId that is available in the account, the foundry output which minted it,
// also needs to be available. You can check this by running the `get_balance` example. You can mint a new native token
// by running the `mint_native_token` example.
const TOKEN_ID: &str = "0x08847bd287c912fadedb6bf38900bda9f2d377b75b2a0bece8738699f56ebca4130100000000";
// The amount of native tokens to mint
const MINT_AMOUNT: u64 = 10;

#[tokio::main]
async fn main() -> Result<()> {
    if TOKEN_ID == "0x08847bd287c912fadedb6bf38900bda9f2d377b75b2a0bece8738699f56ebca4130100000000" {
        println!("You need to change the TOKEN_ID constant before you can run this example successfully!");
        return Ok(());
    }

    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    let token_id = TokenId::from_str(TOKEN_ID)?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    if let Some(native_token_balance) = balance
        .native_tokens()
        .iter()
        .find(|native_token| native_token.token_id() == &token_id)
    {
        println!("Balance BEFORE minting:\n{native_token_balance:#?}");
    } else {
        println!("Couldn't find native token '{TOKEN_ID}' in the account");
        return Ok(());
    }

    // Set the stronghold password
    wallet
        .set_stronghold_password(var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    println!("Sending the minting transaction...");

    // Mint some more native tokens
    let mint_amount = U256::from(MINT_AMOUNT);
    let transaction = account
        .increase_native_token_supply(token_id, mint_amount, None)
        .await?;
    println!("Transaction sent: {}", transaction.transaction.transaction_id);

    let block_id = account
        .retry_transaction_until_included(&transaction.transaction.transaction_id, None, None)
        .await?;
    println!(
        "Transaction included: {}/block/{}",
        var("EXPLORER_URL").unwrap(),
        block_id
    );
    println!("Minted {} native tokens ({})", mint_amount, transaction.token_id);

    let balance = account.sync(None).await?;
    let native_token_balance = balance
        .native_tokens()
        .iter()
        .find(|native_token| native_token.token_id() == &token_id)
        .unwrap();
    println!("Balance AFTER minting:\n{native_token_balance:#?}");

    Ok(())
}
