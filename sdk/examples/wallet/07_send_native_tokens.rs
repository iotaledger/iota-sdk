// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send native tokens.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_wallet` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --all-features --example send_native_tokens --release
//! ```

use iota_sdk::{
    types::block::{
        address::Address,
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NativeToken},
    },
    wallet::{AddressNativeTokens, Result, Wallet},
};
use primitive_types::U256;

// The account alias used in this example
const ACCOUNT_ALIAS: &str = "Alice";
// The native token amount to send
const SEND_NATIVE_TOKEN_AMOUNT: u64 = 10;
// The address to send the tokens to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";
// The wallet database folder
const WALLET_DB_PATH: &str = "./example.walletdb";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Access the wallet we generated with `--example create_wallet`
    let wallet = Wallet::builder().with_storage_path(WALLET_DB_PATH).finish().await?;
    let account = wallet.get_account(ACCOUNT_ALIAS).await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // Get a token with sufficient balance
    if let Some(token_id) = balance
        .native_tokens()
        .iter()
        .find(|t| t.available() >= U256::from(SEND_NATIVE_TOKEN_AMOUNT))
        .map(|t| t.token_id())
    {
        // Set the stronghold password
        wallet
            .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        let bech32_address = RECV_ADDRESS.to_string();

        let outputs = vec![AddressNativeTokens {
            address: bech32_address.clone(),
            native_tokens: vec![(*token_id, U256::from(SEND_NATIVE_TOKEN_AMOUNT))],
            ..Default::default()
        }];

        println!(
            "Sending '{}' native tokens to '{}'...",
            SEND_NATIVE_TOKEN_AMOUNT, bech32_address
        );

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
                .add_unlock_condition(AddressUnlockCondition::new(Address::try_from_bech32(bech32_address)?))
                .with_native_tokens(vec![NativeToken::new(*token_id, U256::from(SEND_NATIVE_TOKEN_AMOUNT))?])
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
