// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example, we will split funds TODO
//!
//! Make sure there's no folder yet at `WALLET_DB_PATH`.
//! For this example it's best to use a fresh mnemonic and start with a balance on the first address only.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example split_funds
//! ```

use std::time::Instant;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    types::block::output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder},
    wallet::{account::types::AccountAddress, Account, ClientOptions, Result, Wallet},
};

// The account alias used in this example
const ACCOUNT_ALIAS: &str = "Alice";
// The wallet database folder created in this example
const WALLET_DB_PATH: &str = "./example.walletdb";
// The base coin amount to send
const SEND_AMOUNT: u64 = 1_000_000;
// The maximum number of addresses funds are distributed to
const MAX_ADDRESSES_TO_SPLIT_FUNDS: usize = 150;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_storage_path(WALLET_DB_PATH)
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Get account or create a new one
    let account = create_account(&wallet, ACCOUNT_ALIAS).await?;

    let _ = generate_max_addresses(&account, ACCOUNT_ALIAS, MAX_ADDRESSES_TO_SPLIT_FUNDS).await?;

    let addresses = account.addresses().await?;
    println!("Total address count: {}", addresses.len());

    sync_print_balance(&account, ACCOUNT_ALIAS).await?;

    let addresses_with_unspent_outputs = account.addresses_with_unspent_outputs().await?;
    println!(
        "Addresses with balance count (before): {}",
        addresses_with_unspent_outputs.len()
    );

    let token_supply = account.client().get_token_supply().await?;

    // Send split transactions
    for addresses_chunk in addresses.chunks(2).map(|chunk| chunk.to_vec()) {
        let outputs_per_transaction = addresses_chunk
            .into_iter()
            .map(|address| {
                BasicOutputBuilder::new_with_amount(SEND_AMOUNT)
                    .add_unlock_condition(AddressUnlockCondition::new(*address.address().as_ref()))
                    .finish_output(token_supply)
                    .unwrap()
            })
            .collect::<Vec<_>>();

        println!(
            "Sending '{}' coins in {} outputs...",
            SEND_AMOUNT,
            outputs_per_transaction.len()
        );
        let transaction = account.send(outputs_per_transaction, None).await?;
        println!(
            "Transaction sent: {}/transaction/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            transaction.transaction_id
        );

        // Wait for transaction to get included
        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;

        println!(
            "Transaction included: {}/block/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            block_id
        );
    }

    sync_print_balance(&account, ACCOUNT_ALIAS).await?;

    let addresses_with_unspent_outputs = account.addresses_with_unspent_outputs().await?;
    println!(
        "Addresses with balance count (after): {}",
        addresses_with_unspent_outputs.len()
    );

    println!("Example finished successfully");

    Ok(())
}

async fn create_account(wallet: &Wallet, alias: &str) -> Result<Account> {
    Ok(if let Ok(account) = wallet.get_account(alias).await {
        account
    } else {
        println!("Creating account '{alias}'");
        wallet.create_account().with_alias(alias.to_string()).finish().await?
    })
}

async fn sync_print_balance(account: &Account, alias: &str) -> Result<()> {
    let now = Instant::now();
    let balance = account.sync(None).await?;
    println!("{alias}'s account synced in: {:.2?}", now.elapsed());
    println!("{alias}'s balance:\n{:#?}", balance.base_coin());
    Ok(())
}

async fn generate_max_addresses(account: &Account, alias: &str, max: usize) -> Result<Vec<AccountAddress>> {
    if account.addresses().await?.len() < max {
        let num_addresses_to_generate = max - account.addresses().await?.len();
        println!("Generating {num_addresses_to_generate} addresses for account '{alias}'...");
        account
            .generate_addresses(num_addresses_to_generate as u32, None)
            .await?;
    }
    account.addresses().await
}
