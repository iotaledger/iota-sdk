// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will burn a native token. This will not increase the melted supply in the foundry,
//! therefore the foundry output is also not required. But this will also make it impossible to destroy the foundry
//! output that minted it.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_wallet` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --all-features --example burn_native_token --release
//! ```

use std::str::FromStr;

use iota_sdk::{
    types::block::output::TokenId,
    wallet::{Result, Wallet, U256},
};

// The account alias used in this example
const ACCOUNT_ALIAS: &str = "Alice";
// The native token id. Replace it with a TokenId that is available in the account, the foundry output which minted it,
// also needs to be available. You can check this by running the `get_balance` example. You can mint a new native token
// by running the `mint_native_token` example.
const TOKEN_ID: &str = "0x08847bd287c912fadedb6bf38900bda9f2d377b75b2a0bece8738699f56ebca4130100000000";
// The minimum available native token amount to search for in the account
const MIN_AVAILABLE_AMOUNT: u64 = 11;
// The amount of the native token to burn
const BURN_AMOUNT: u64 = 1;
// The wallet database folder
const WALLET_DB_PATH: &str = "./example.walletdb";

#[tokio::main]
async fn main() -> Result<()> {
    if TOKEN_ID == "0x08847bd287c912fadedb6bf38900bda9f2d377b75b2a0bece8738699f56ebca4130100000000" {
        println!("You need to change the token id before you can run this example!");
        return Ok(());
    }

    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Access the wallet we generated with `--example create_wallet`
    let wallet = Wallet::builder().with_storage_path(WALLET_DB_PATH).finish().await?;
    let account = wallet.get_account(ACCOUNT_ALIAS).await?;

    let token_id = TokenId::from_str(TOKEN_ID)?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    if let Some(native_token_balance) = balance.native_tokens().iter().find(|native_token| {
        native_token.token_id() == &token_id && native_token.available() >= U256::from(MIN_AVAILABLE_AMOUNT)
    }) {
        println!("Balance BEFORE burning:\n{native_token_balance:#?}",);

        // Set the stronghold password
        wallet
            .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        println!("Preparing burning transaction ...");

        // Burn a native token
        let burn_amount = U256::from(BURN_AMOUNT);
        let transaction = account
            .burn_native_token(*native_token_balance.token_id(), burn_amount, None)
            .await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;
        println!(
            "Block included: {}/block/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            block_id
        );
        println!(
            "Burned {} native token(s) ({})",
            burn_amount,
            native_token_balance.token_id()
        );

        let balance = account.sync(None).await?;

        println!("Balance AFTER burning:");
        if let Some(native_token_balance) = balance
            .native_tokens()
            .iter()
            .find(|native_token| native_token.token_id() == native_token_balance.token_id())
        {
            println!("{native_token_balance:#?}");
        } else {
            println!("No remaining tokens");
        }
    } else {
        println!(
            "Native token '{TOKEN_ID}' doesn't exist or there's not at least '{MIN_AVAILABLE_AMOUNT}' tokens of it in account '{ACCOUNT_ALIAS}'"
        );
    }

    Ok(())
}
