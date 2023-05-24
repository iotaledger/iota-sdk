// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will spam transactions from multiple threads simultaneously to our own addresses.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example spammer
//! ```

use std::{env::var, time::Duration};

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        request_funds_from_faucet,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    types::block::{
        address::Bech32Address,
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder},
        payload::transaction::TransactionId,
    },
    wallet::{Account, ClientOptions, Result, SendAmountParams, Wallet},
};
use tokio::{
    task::JoinSet,
    time::{sleep, Instant},
};

// The account alias used in this example.
const ACCOUNT_ALIAS: &str = "spammer";
// The number of spamming rounds.
const NUM_ROUNDS: usize = 1000;
// The amount to send in each transaction
const SEND_AMOUNT: u64 = 1_000_000;
// The number of simultaneous transactions
const NUM_SIMULTANEOUS_TXS: usize = 4;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let num_simultaneous_txs = NUM_SIMULTANEOUS_TXS.min(num_cpus::get());

    println!("Spammer set up to issue {num_simultaneous_txs} transactions simultaneously.");

    // Restore wallet from a mnemonic phrase.
    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;
    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;
    let account = get_or_create_account(&wallet, ACCOUNT_ALIAS).await?;

    let recv_address = *account.addresses().await?[0].address();

    // Ensure there are enough available funds for spamming on each address.
    let available_funds = ensure_enough_funds(&account, &recv_address, num_simultaneous_txs).await?;
    account.sync(None).await?;

    let split_amount = available_funds / 2;
    let output_amount = split_amount / 127u64;

    println!("Splitting funds...");
    let token_supply = account.client().get_token_supply().await?;
    let outputs = std::iter::repeat_with(|| {
        BasicOutputBuilder::new_with_amount(output_amount)
            .add_unlock_condition(AddressUnlockCondition::new(recv_address))
            .finish_output(token_supply)
            .unwrap()
    })
    .take(127)
    .collect::<Vec<_>>();

    let transaction = account.send(outputs, None).await?;
    wait_for_inclusion(&transaction.transaction_id, &account).await?;

    account.sync(None).await?;

    println!("Spamming transactions...");
    for i in 1..=NUM_ROUNDS {
        println!("ROUND {i}/{NUM_ROUNDS}");

        let mut tasks = JoinSet::<std::result::Result<Duration, (usize, iota_sdk::wallet::Error)>>::new();

        for n in 0..num_simultaneous_txs {
            let account_clone = account.clone();

            tasks.spawn(async move {
                println!("Thread {n}: Sending {SEND_AMOUNT} coins to {recv_address}");
                let now = Instant::now();
                let outputs = vec![SendAmountParams::new(recv_address, SEND_AMOUNT)];
                let transaction = account_clone.send_amount(outputs, None).await.map_err(|err| (n, err))?;

                let elapsed = now.elapsed();
                println!(
                    "Thread {n}: Transaction sent in {elapsed:.2?}: {}/transaction/{}",
                    var("EXPLORER_URL").unwrap(),
                    transaction.transaction_id
                );

                Ok(elapsed)
            });
        }

        let mut error_state: std::result::Result<(), ()> = Ok(());
        let mut max_duration = Duration::from_secs(0);
        while let Some(Ok(res)) = tasks.join_next().await {
            match res {
                Ok(elapsed) => max_duration = max_duration.max(elapsed),
                Err((n, err)) => {
                    println!("Thread {n}: {err}");
                    error_state = Err(());
                }
            }
        }

        println!(
            "==> BPS: {:.2}",
            NUM_SIMULTANEOUS_TXS as f64 / max_duration.as_secs_f64()
        );

        if error_state.is_err() {
            // Sync when getting an error, because that's probably when no outputs are available anymore
            let mut balance = account.sync(None).await?;
            while balance.base_coin().available() == 0 {
                println!("No funds available");
                sleep(Duration::from_secs(2)).await;
                balance = account.sync(None).await?;
                println!("Account synced");
            }
        }
    }
    Ok(())
}

async fn get_or_create_account(wallet: &Wallet, alias: &str) -> Result<Account> {
    Ok(if let Ok(account) = wallet.get_account(alias).await {
        account
    } else {
        println!("Creating account '{alias}'");
        wallet.create_account().with_alias(alias.to_string()).finish().await?
    })
}

async fn ensure_enough_funds(
    account: &Account,
    bech32_address: &Bech32Address,
    num_simultaneous_txs: usize,
) -> Result<u64> {
    let balance = account.sync(None).await?;
    let available_funds_before = balance.base_coin().available();
    println!("Current available funds: {available_funds_before}");

    let min_required_funds = available_funds_before / 2 / num_simultaneous_txs as u64;
    if min_required_funds < SEND_AMOUNT {
        println!("Requesting funds from faucet...");
        let faucet_response = request_funds_from_faucet(&var("FAUCET_URL").unwrap(), bech32_address).await?;
        println!("Response from faucet: {}", faucet_response.trim_end());
        if faucet_response.contains("error") {
            panic!("Requesting funds failed (error response)");
        }

        println!("Waiting for funds (timeout=60s)...");
        // Check for changes to the balance
        let start = std::time::Instant::now();
        let available_funds_after = loop {
            if start.elapsed().as_secs() > 60 {
                panic!("Requesting funds failed (timeout)");
            };
            let balance = account.sync(None).await?;
            let available_funds_after = balance.base_coin().available();
            if available_funds_after > available_funds_before {
                break available_funds_after;
            } else {
                tokio::time::sleep(instant::Duration::from_secs(2)).await;
            }
        };
        println!("New available funds: {available_funds_after}");
        Ok(available_funds_after)
    } else {
        println!("No faucet request necessary");
        Ok(available_funds_before)
    }
}

async fn wait_for_inclusion(transaction_id: &TransactionId, account: &Account) -> Result<()> {
    println!(
        "Transaction sent: {}/transaction/{}",
        var("EXPLORER_URL").unwrap(),
        transaction_id
    );
    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(transaction_id, None, None)
        .await?;
    println!(
        "Transaction included: {}/block/{}",
        var("EXPLORER_URL").unwrap(),
        block_id
    );
    Ok(())
}
