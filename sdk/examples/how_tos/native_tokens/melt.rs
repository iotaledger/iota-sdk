// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will melt an existing native token with its foundry.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_account.rs` example!
//!
//! You may provide a TOKEN_ID that is available in the account. The foundry
//! output which minted it needs to be available as well. You can check this by
//! running the `get_balance` example. You can create a new native token by running
//! the `create_native_token` example.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example melt_native_token [TOKEN_ID]
//! ```

use iota_sdk::{types::block::output::TokenId, wallet::Result, Wallet};

// The amount of native tokens to melt
const MELT_AMOUNT: u64 = 10;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // Find first foundry and corresponding token id
    let token_id = std::env::args()
        .nth(1)
        .map(|s| s.parse::<TokenId>().expect("invalid token id"))
        .unwrap_or_else(|| TokenId::from(*balance.foundries().first().unwrap()));

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
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Melt some of the circulating supply

    let transaction = account.melt_native_token(token_id, MELT_AMOUNT, None).await?;
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
    let available_balance = balance
        .native_tokens()
        .iter()
        .find(|t| t.token_id() == &token_id)
        .unwrap()
        .available();
    println!("Balance after melting: {available_balance}",);

    Ok(())
}
