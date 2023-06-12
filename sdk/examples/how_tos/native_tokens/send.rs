// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send native tokens.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example send_native_tokens`

use iota_sdk::{
    types::block::address::Bech32Address,
    wallet::{Result, SendNativeTokensParams, Wallet},
};
use primitive_types::U256;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;
    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // Get a token with sufficient balance
    if let Some(token_id) = balance
        .native_tokens()
        .iter()
        .find(|t| t.available() >= U256::from(10))
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
            .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        let bech32_address =
            Bech32Address::try_from_str("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu")?;

        let outputs = [SendNativeTokensParams::new(
            bech32_address,
            [(*token_id, U256::from(10))],
        )?];

        let transaction = account.send_native_tokens(outputs, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        // Wait for transaction to get included
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
            .find(|t| t.token_id() == token_id)
            .unwrap()
            .available();
        println!("Balance after sending: {available_balance}",);
    } else {
        println!("Insufficient native token funds");
    }

    Ok(())
}
