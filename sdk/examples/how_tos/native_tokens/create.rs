// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create a native token.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_wallet` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example create_native_token
//! ```

use iota_sdk::{types::block::output::feature::Irc30Metadata, wallet::CreateNativeTokenParams, Wallet, U256};

// The circulating supply of the native token
const CIRCULATING_SUPPLY: u64 = 100;
// The maximum supply of the native token
const MAXIMUM_SUPPLY: u64 = 100;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH", "STRONGHOLD_PASSWORD", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    let balance = wallet.sync(None).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // We can first check if we already have an account output in our wallet, because an account can have many foundry
    // outputs and therefore we can reuse an existing one
    if balance.accounts().is_empty() {
        // If we don't have an account, we need to create one
        let transaction = wallet.create_account_output(None, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        wallet
            .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
            .await?;
        println!(
            "Tx accepted: {}/transactions/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            transaction.transaction_id
        );

        wallet.sync(None).await?;
        println!("Wallet synced");
    }

    let metadata =
        Irc30Metadata::new("My Native Token", "MNT", 10).with_description("A native token to test the iota-sdk.");

    println!("Preparing transaction to create native token...");

    let params = CreateNativeTokenParams {
        account_id: None,
        circulating_supply: U256::from(CIRCULATING_SUPPLY),
        maximum_supply: U256::from(MAXIMUM_SUPPLY),
        foundry_metadata: Some(metadata.try_into()?),
    };

    let transaction = wallet.create_native_token(params, None).await?;
    println!("Transaction sent: {}", transaction.transaction.transaction_id);

    wallet
        .wait_for_transaction_acceptance(&transaction.transaction.transaction_id, None, None)
        .await?;

    println!(
        "Tx accepted: {}/transactions/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction.transaction.transaction_id
    );
    println!("Created token: {}", transaction.token_id);

    // Ensure the wallet is synced after creating the native token.
    wallet.sync(None).await?;
    println!("Wallet synced");

    Ok(())
}
