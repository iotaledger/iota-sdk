// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will spam transactions from multiple threads simultaneously to our own address.
//!
//! `cargo run --example threads --release`

use std::env::var;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        request_funds_from_faucet,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    wallet::{Account, ClientOptions, Result, SendAmountParams, Wallet},
};
use tokio::task::JoinSet;

const ACCOUNT_ALIAS: &str = "spammer";
const NUM_ROUNDS: usize = 4;
const SEND_AMOUNT: u64 = 1_000_000;
const NUM_WORKER_THREADS: usize = 6;

// #[tokio::main]
#[tokio::main(flavor = "multi_thread", worker_threads = 6)]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    println!(
        "{} cores detected. Running spammer with {} worker threads...",
        num_cpus::get(),
        NUM_WORKER_THREADS
    );

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

    // Ensure there's some base coin balance
    let account = get_or_create_account(&wallet, ACCOUNT_ALIAS).await?;

    // One address gets generated during account creation
    let recv_address = account.addresses().await?[0].address().to_string();

    // Ensure there are enough available funds for spamming
    may_request_funds(&account, recv_address.as_str()).await?;

    for i in 1..=NUM_ROUNDS {
        println!("ROUND #{i}/{NUM_ROUNDS}");

        let mut tasks = JoinSet::<std::result::Result<(), (usize, iota_sdk::wallet::Error)>>::new();

        for n in 0..NUM_WORKER_THREADS {
            let account_clone = account.clone();
            let recv_address_clone = recv_address.clone(); //*recv_address.as_ref();

            tasks.spawn(async move {
                println!("Thread {n}: Sending {SEND_AMOUNT} to {recv_address_clone}");
                let outputs = vec![SendAmountParams::new(recv_address_clone, SEND_AMOUNT)];
                let transaction = account_clone.send_amount(outputs, None).await.map_err(|err| (n, err))?;

                println!(
                    "Thread {n}: Transaction sent: {}/transaction/{}",
                    var("EXPLORER_URL").unwrap(),
                    transaction.transaction_id
                );

                Ok(())
            });
        }

        let mut error_state: std::result::Result<(), ()> = Ok(());
        while let Some(Ok(res)) = tasks.join_next().await {
            match res {
                Ok(()) => {}
                Err((n, err)) => {
                    println!("Thread {n}: Failure: {err}");
                    error_state = Err(());
                }
            }
        }

        if error_state.is_err() {
            // Sync when getting an error, because that's probably when no outputs are available anymore
            account.sync(None).await?;
            println!("Account synced");
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

async fn may_request_funds(account: &Account, bech32_address: &str) -> Result<()> {
    let balance = account.sync(None).await?;
    let available_funds_before = balance.base_coin().available();
    println!("Current available funds: {available_funds_before}");

    if available_funds_before < NUM_WORKER_THREADS as u64 * SEND_AMOUNT {
        println!("Requesting funds from faucet...");
        let faucet_response = request_funds_from_faucet(&var("FAUCET_URL").unwrap(), bech32_address).await?;
        println!("Response from faucet: {}", faucet_response.trim_end());
        if faucet_response.contains("error") {
            return Ok(());
        }

        println!("Waiting for funds (timeout=60s)...");
        // Check for changes to the balance
        let start = std::time::Instant::now();
        let available_funds_after = loop {
            if start.elapsed().as_secs() > 60 {
                println!("Timeout: waiting for funds took too long");
                return Ok(());
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
    } else {
        println!("No faucet request necessary");
    }
    Ok(())
}
