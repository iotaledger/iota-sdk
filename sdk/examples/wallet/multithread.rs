// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send coins from the first address of the 1st account  to several different addresses
//! of the 2nd account in parallel using up to a configured number of threads.
//!
//! Non-existing accounts will be created and funded automatically.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example multithread
//! ```

use std::env::var;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        request_funds_from_faucet,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    wallet::{account::types::AccountAddress, Account, ClientOptions, Result, Wallet},
};
use tokio::task::JoinSet;

// The maximum number of addresses to send funds to
const NUM_RECV_ADDRESSES: usize = 3;
// The base amount of coins to send (the actual amount will be multiples of that)
const BASE_AMOUNT: u64 = 1_000_000;
// The maximum number of CPU cores to use
const MAX_CPUS_TO_USE: usize = 4;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new().with_node(&var("NODE_URL").unwrap())?;

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(&var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    let alias1 = var("ACCOUNT_ALIAS_1").unwrap();
    let account1 = get_or_create_account(&wallet, &alias1).await?;
    let alias2 = var("ACCOUNT_ALIAS_2").unwrap();
    let account2 = get_or_create_account(&wallet, &alias2).await?;

    let account1_send_address = &account1.addresses().await?[0];
    let account2_recv_addresses = generate_addresses(&account2).await?;

    sync_and_print_balance(&account1).await?;
    sync_and_print_balance(&account2).await?;

    may_request_funds(&account1, &account1_send_address.address().to_string()).await?;

    let mut tasks: JoinSet<Result<(usize, usize)>> = JoinSet::new();
    let num_threads_per_address = num_cpus::get().min(MAX_CPUS_TO_USE);

    for address_index in 0..NUM_RECV_ADDRESSES {
        for thread_index in 1..=num_threads_per_address {
            let account1_clone = account1.clone();
            let account2_recv_addresses_clone = account2_recv_addresses.clone();

            tasks.spawn(async move {
                let amount = ((address_index + thread_index) % 3 + 1) as u64 * BASE_AMOUNT;
                let recv_address = account2_recv_addresses_clone[address_index % NUM_RECV_ADDRESSES].address();
                println!("Sending '{amount}' coins to '{recv_address}'...");

                let transaction = if (address_index + thread_index) % 2 == 0 {
                    // ALTERNATIVE 1: using `account.send_amount``
                    let outputs = vec![iota_sdk::wallet::SendAmountParams::new(
                        recv_address.to_string(),
                        amount,
                    )];
                    account1_clone.send_amount(outputs, None).await?
                } else {
                    // ALTERNATIVE 2: using `account.send`
                    let outputs = vec![
                        iota_sdk::types::block::output::BasicOutputBuilder::new_with_amount(amount)
                            .add_unlock_condition(
                                iota_sdk::types::block::output::unlock_condition::AddressUnlockCondition::new(
                                    *recv_address.as_ref(),
                                ),
                            )
                            .finish_output(account1_clone.client().get_token_supply().await?)?,
                    ];
                    account1_clone.send(outputs, None).await?
                };

                println!(
                    "Transaction to address {} from thread {thread_index}/{num_threads_per_address} sent: {}",
                    recv_address, transaction.transaction_id
                );

                // Wait for transaction to get included
                let block_id = account1_clone
                    .retry_transaction_until_included(&transaction.transaction_id, None, None)
                    .await?;

                println!(
                    "Transaction included: {}/block/{}",
                    var("EXPLORER_URL").unwrap(),
                    block_id
                );

                iota_sdk::wallet::Result::Ok((address_index, thread_index))
            });
        }
    }

    while let Some(Ok(result)) = tasks.join_next().await {
        match result {
            Ok((address_index, thread_index)) => println!("Thread {address_index}:{thread_index} finished"),
            Err(e) => println!("{e}"),
        }
    }

    sync_and_print_balance(&account1).await?;
    sync_and_print_balance(&account2).await?;

    println!("Example finished successfully");
    Ok(())
}

async fn sync_and_print_balance(account: &Account) -> Result<()> {
    let alias = account.alias().await;
    let balance = account.sync(None).await?;
    println!("{alias}'s account synced");
    println!("{alias}'s balance:\n{:#?}", balance.base_coin());
    Ok(())
}

async fn get_or_create_account(wallet: &Wallet, alias: &str) -> Result<Account> {
    let account = if let Ok(account) = wallet.get_account(alias).await {
        account
    } else {
        println!("Creating account '{alias}'");
        wallet.create_account().with_alias(alias.to_string()).finish().await?
    };
    Ok(account)
}

async fn generate_addresses(account: &Account) -> Result<Vec<AccountAddress>> {
    let alias = account.alias().await;
    if account.addresses().await?.len() < NUM_RECV_ADDRESSES {
        let num_addresses_to_generate = NUM_RECV_ADDRESSES - account.addresses().await?.len();
        println!("Generating {num_addresses_to_generate} addresses for {alias}...");
        account
            .generate_addresses(num_addresses_to_generate as u32, None)
            .await?;
    }
    account.addresses().await
}

async fn may_request_funds(account: &Account, address: &str) -> Result<()> {
    let balance = account.sync(None).await?;
    let funds_before = balance.base_coin().available();
    println!("Current available funds: {funds_before}");

    if funds_before < NUM_RECV_ADDRESSES as u64 * num_cpus::get().min(MAX_CPUS_TO_USE) as u64 * BASE_AMOUNT {
        println!("Requesting funds from faucet...");
        let faucet_response = request_funds_from_faucet(&var("FAUCET_URL").unwrap(), address).await?;
        println!("Response from faucet: {}", faucet_response.trim_end());
        if faucet_response.contains("error") {
            return Ok(());
        }

        println!("Waiting for funds (timeout=60s)...");
        // Check for changes to the balance
        let start = std::time::Instant::now();
        let funds_after = loop {
            if start.elapsed().as_secs() > 60 {
                println!("Timeout: waiting for funds took too long");
                return Ok(());
            };
            let balance = account.sync(None).await?;
            let funds_after = balance.base_coin().available();
            if funds_after > funds_before {
                break funds_after;
            } else {
                tokio::time::sleep(instant::Duration::from_secs(2)).await;
            }
        };
        println!("New available funds: {funds_after}");
    } else {
        println!("No faucet request necessary");
    }

    Ok(())
}
