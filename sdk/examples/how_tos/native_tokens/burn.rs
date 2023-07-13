// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will burn an existing native token, this will not increase the melted supply in the foundry,
//! therefore the foundry output is also not required. But this will also make it impossible to destroy the foundry
//! output that minted it.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_account.rs` example!
//!
//! You may provide a TOKEN_ID that is available in the account. You can check this by running the
//! `get_balance` example. You can create a new native token by running the `create_native_token` example.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example burn_native_token [TOKEN_ID]
//! ```

use iota_sdk::{
    types::block::output::{NativeToken, TokenId},
    wallet::Result,
    Wallet, U256,
};

// The minimum available native token amount to search for in the account
const MIN_AVAILABLE_AMOUNT: u64 = 11;
// The amount of the native token to burn
const BURN_AMOUNT: u64 = 1;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let alias = "Alice";
    let account = wallet.get_account(alias.to_string()).await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // Take the given token id, or use a default.
    let token_id = std::env::args()
        .nth(1)
        .map(|s| s.parse::<TokenId>().expect("invalid token id"))
        .unwrap_or_else(|| TokenId::from(*balance.foundries().first().unwrap()));

    if let Some(native_token_balance) = balance.native_tokens().iter().find(|native_token| {
        native_token.token_id() == &token_id && native_token.available() >= U256::from(MIN_AVAILABLE_AMOUNT)
    }) {
        println!("Balance before burning: {native_token_balance:#?}");

        // Set the stronghold password
        wallet
            .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        // Burn a native token
        let transaction = account.burn(NativeToken::new(token_id, BURN_AMOUNT)?, None).await?;
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

        print!("Balance after burning: ");
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
            "Native token '{token_id}' doesn't exist or there's not at least '{MIN_AVAILABLE_AMOUNT}' tokens of it in account '{alias}'"
        );
    }

    Ok(())
}
