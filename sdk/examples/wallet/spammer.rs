// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will spam transactions from multiple threads simultaneously to our own address.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example spammer
//! ```

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        request_funds_from_faucet,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    types::block::{address::Bech32Address, output::BasicOutput, payload::transaction::TransactionId},
    wallet::{account::FilterOptions, Account, ClientOptions, Result, SendAmountParams, Wallet},
};

// The account alias used in this example.
const ACCOUNT_ALIAS: &str = "spammer";
// The number of spamming rounds.
const NUM_ROUNDS: usize = 1000;
// The amount to send in each transaction
const SEND_AMOUNT: u64 = 1_000_000;
// The number of simultaneous transactions
const NUM_SIMULTANEOUS_TXS: usize = 16;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let num_simultaneous_txs = NUM_SIMULTANEOUS_TXS.min(num_cpus::get());

    println!("Spammer set up to issue {num_simultaneous_txs} transactions simultaneously.");

    // Restore wallet from a mnemonic phrase.
    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;
    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;
    let account = get_or_create_account(&wallet, ACCOUNT_ALIAS).await?;

    let recv_address = *account.addresses().await?[0].address();

    // Ensure there are enough available funds for spamming.
    ensure_enough_funds(&account, &recv_address).await?;

    // We make sure that for all threads there are always inputs available to
    // fund the transaction, otherwise we create enough unspent outputs.
    let num_unspent_basic_outputs_with_send_amount = account
        .unspent_outputs(FilterOptions {
            output_types: Some(vec![BasicOutput::KIND]),
            ..Default::default()
        })
        .await?
        .iter()
        .filter(|data| data.output.amount() >= SEND_AMOUNT)
        .count();

    println!("Num unspent basic output holding >={SEND_AMOUNT}: {num_unspent_basic_outputs_with_send_amount}");

    if num_unspent_basic_outputs_with_send_amount < 127 {
        println!("Creating unspent outputs...");

        let transaction = account
            .send_amount(vec![SendAmountParams::new(recv_address, SEND_AMOUNT)?; 127], None)
            .await?;
        wait_for_inclusion(&transaction.transaction_id, &account).await?;

        account.sync(None).await?;
    }

    println!("Spamming transactions...");
    for i in 1..=NUM_ROUNDS {
        println!("ROUND {i}/{NUM_ROUNDS}");
        let round_timer = tokio::time::Instant::now();

        let mut tasks = tokio::task::JoinSet::<std::result::Result<(), (usize, iota_sdk::wallet::Error)>>::new();

        for n in 0..num_simultaneous_txs {
            let account_clone = account.clone();

            tasks.spawn(async move {
                println!("Thread {n}: sending {SEND_AMOUNT} coins to own address");

                let thread_timer = tokio::time::Instant::now();
                let outputs = vec![SendAmountParams::new(recv_address, SEND_AMOUNT).map_err(|err| (n, err))?];
                let transaction = account_clone.send_amount(outputs, None).await.map_err(|err| (n, err))?;
                let elapsed = thread_timer.elapsed();

                println!(
                    "Thread {n}: sent in {elapsed:.2?}: {}/transaction/{}",
                    std::env::var("EXPLORER_URL").unwrap(),
                    transaction.transaction_id
                );

                Ok(())
            });
        }

        let mut error_state: std::result::Result<(), ()> = Ok(());
        let mut sent_transactions = 0;
        while let Some(Ok(res)) = tasks.join_next().await {
            match res {
                Ok(()) => sent_transactions += 1,
                Err((n, err)) => {
                    println!("Thread {n}: {err}");
                    error_state = Err(());
                }
            }
        }

        if error_state.is_err() {
            // Sync when getting an error, because that's probably when no outputs are available anymore
            let mut balance = account.sync(None).await?;
            println!("Account synced");

            while balance.base_coin().available() == 0 {
                println!("No funds available");
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                balance = account.sync(None).await?;
                println!("Account synced");
            }
        }

        println!(
            "==> BPS: {:.2}",
            sent_transactions as f64 / round_timer.elapsed().as_secs_f64()
        );
    }
    Ok(())
}

async fn get_or_create_account(wallet: &Wallet, alias: &str) -> Result<Account> {
    Ok(if let Ok(account) = wallet.get_account(alias).await {
        account
    } else {
        println!("Creating account '{alias}'");
        wallet.create_account().with_alias(alias).finish().await?
    })
}

async fn ensure_enough_funds(account: &Account, bech32_address: &Bech32Address) -> Result<()> {
    let balance = account.sync(None).await?;
    let available_funds = balance.base_coin().available();
    println!("Available funds: {available_funds}");
    let min_required_funds = (1.1f64 * (127u64 * SEND_AMOUNT) as f64) as u64;
    println!("Min required funds: {min_required_funds}");
    if available_funds < min_required_funds {
        println!("Requesting funds from faucet...");
        let faucet_response = request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), bech32_address).await?;
        println!("Response from faucet: {}", faucet_response.trim_end());
        if faucet_response.contains("error") {
            panic!("Requesting funds failed (error response)");
        }

        println!("Waiting for funds (timeout=60s)...");
        // Check for changes to the balance
        let start = std::time::Instant::now();
        let new_available_funds = loop {
            if start.elapsed().as_secs() > 60 {
                panic!("Requesting funds failed (timeout)");
            };
            let balance = account.sync(None).await?;
            let available_funds_after = balance.base_coin().available();
            if available_funds_after > available_funds {
                break available_funds_after;
            } else {
                tokio::time::sleep(instant::Duration::from_secs(2)).await;
            }
        };
        println!("New available funds: {new_available_funds}");
        if new_available_funds < min_required_funds {
            panic!("insufficient funds: pick a smaller SEND_AMOUNT");
        } else {
            Ok(())
        }
    } else {
        println!("No faucet request necessary");
        Ok(())
    }
}

async fn wait_for_inclusion(transaction_id: &TransactionId, account: &Account) -> Result<()> {
    println!(
        "Transaction sent: {}/transaction/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction_id
    );
    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );
    Ok(())
}
