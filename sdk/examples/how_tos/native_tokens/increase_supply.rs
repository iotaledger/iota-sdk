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

use iota_sdk::{types::block::output::TokenId, wallet::Result, Wallet, U256};

// The amount of native tokens to mint
const MINT_AMOUNT: u64 = 10;

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

    // Set the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Find first foundry and corresponding token id
    let token_id = TokenId::from(*balance.foundries().first().unwrap());

    let available_balance = balance
        .native_tokens()
        .iter()
        .find(|t| t.token_id() == &token_id)
        .unwrap()
        .available();
    println!("Balance before minting: {available_balance}",);

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
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );
    println!("Minted {} native tokens ({})", mint_amount, transaction.token_id);

    let balance = account.sync(None).await?;
    let available_balance = balance
        .native_tokens()
        .iter()
        .find(|t| t.token_id() == &token_id)
        .unwrap()
        .available();
    println!("Balance after minting: {available_balance:?}",);

    Ok(())
}
