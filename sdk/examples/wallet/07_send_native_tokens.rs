// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send native tokens.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example send_native_tokens --release`

use iota_sdk::{
    types::block::{
        address::Bech32Address,
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NativeToken},
    },
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
        // Set the stronghold password
        wallet
            .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        let bech32_address =
            Bech32Address::try_from_str("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu")?;

        let outputs = vec![SendNativeTokensParams {
            address: bech32_address,
            native_tokens: vec![(*token_id, U256::from(10))],
            return_address: Default::default(),
            expiration: Default::default(),
        }];

        println!("Preparing native token transaction...");

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

        account.sync(None).await?;
        println!("Account synced");

        println!("Preparing basic output transaction...");

        // Send native tokens together with the required storage deposit
        let rent_structure = account.client().get_rent_structure().await?;

        let outputs = vec![
            BasicOutputBuilder::new_with_minimum_storage_deposit(rent_structure)
                .add_unlock_condition(AddressUnlockCondition::new(bech32_address))
                .with_native_tokens(vec![NativeToken::new(*token_id, U256::from(10))?])
                .finish_output(account.client().get_token_supply().await?)?,
        ];

        let transaction = account.send(outputs, None).await?;
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
    } else {
        println!("Insufficient native token funds");
    }

    Ok(())
}
