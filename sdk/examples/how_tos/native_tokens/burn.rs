// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will burn an existing native token, this will not increase the melted supply in the foundry,
//! therefore the foundry output is also not required. But this will also make it impossible to destroy the foundry
//! output that minted it.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example burn_native_token
//! ```

use iota_sdk::{types::block::output::NativeToken, wallet::Result, Wallet, U256};

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

    // Get a token with sufficient balance
    if let Some(token_id) = balance
        .native_tokens()
        .iter()
        .find(|t| t.available() >= U256::from(MIN_AVAILABLE_AMOUNT))
        .map(|t| t.token_id())
    {
        let available_balance = balance
            .native_tokens()
            .iter()
            .find(|t| t.token_id() == token_id)
            .unwrap()
            .available();
        println!("Balance before burning: {available_balance:?}");

        // Set the stronghold password
        wallet
            .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        // Burn a native token
        let burn_amount = U256::from(BURN_AMOUNT);
        let transaction = account.burn(NativeToken::new(*token_id, burn_amount)?, None).await?;
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
            .find(|native_token| native_token.token_id() == token_id)
        {
            let available_balance = native_token_balance.available();
            println!("{available_balance}");
        } else {
            println!("No remaining tokens");
        }
    } else {
        println!(
            "No native token exist or there's not at least '{MIN_AVAILABLE_AMOUNT}' tokens of it in account '{alias}'"
        );
    }

    Ok(())
}
