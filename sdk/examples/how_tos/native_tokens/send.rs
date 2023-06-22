// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send native tokens.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example send_native_tokens
//! ```

use std::env::var;

use iota_sdk::{
    types::block::address::Bech32Address,
    wallet::{Result, SendNativeTokensParams},
    Wallet,
};
use primitive_types::U256;

// The native token amount to send
const SEND_NATIVE_TOKEN_AMOUNT: u64 = 10;
// The address to send the tokens to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // Get a token with sufficient balance
    if let Some(token_id) = balance
        .native_tokens()
        .iter()
        .find(|t| t.available() >= U256::from(SEND_NATIVE_TOKEN_AMOUNT))
        .map(|t| t.token_id())
    {
        let available_balance = balance
            .native_tokens()
            .iter()
            .find(|t| t.token_id() == token_id)
            .unwrap()
            .available();
        println!("Balance before sending: {available_balance}");

        // Set the stronghold password
        wallet
            .set_stronghold_password(var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        let bech32_address = RECV_ADDRESS.parse::<Bech32Address>()?;

        let outputs = [SendNativeTokensParams::new(
            bech32_address,
            [(*token_id, U256::from(SEND_NATIVE_TOKEN_AMOUNT))],
        )?];

        let transaction = account.send_native_tokens(outputs, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        // Wait for transaction to get included
        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;
        println!("Block included: {}/block/{}", var("EXPLORER_URL").unwrap(), block_id);

        let balance = account.sync(None).await?;

        let available_balance = balance
            .native_tokens()
            .iter()
            .find(|t| t.token_id() == token_id)
            .unwrap()
            .available();
        println!("Balance after sending: {available_balance}",);
    } else {
        println!("Insufficient native token funds");
    }

    Ok(())
}
