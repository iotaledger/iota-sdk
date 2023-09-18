// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will:
//! * create a wallet from a mnemonic phrase
//! * create an account if it does not exist yet
//! * generate some addresses for that account - if necessary
//! * print all addresses in the account
//! * print all addresses with funds in the account
//! * make a coin transaction
//!
//! Make sure there's no `STRONGHOLD_SNAPSHOT_PATH` file and no `WALLET_DB_PATH` folder yet!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example wallet
//! ```

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    types::block::payload::transaction::TransactionId,
    wallet::{Account, ClientOptions, Result, Wallet},
};

// The number of addresses to generate in this account
const MAX_ADDRESSES_TO_GENERATE: usize = 10;
// The amount of coins to send
const SEND_AMOUNT: u64 = 1_000_000;
// The address to send the coins to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = create_wallet().await?;

    let account = wallet.get_or_create_account("Alice").await?;
    print_accounts(&wallet).await?;

    generate_addresses(&account, MAX_ADDRESSES_TO_GENERATE).await?;
    print_addresses(&account).await?;

    // Change to `true` to print the full balance report
    sync_print_balance(&account, false).await?;

    print_addresses_with_funds(&account).await?;

    println!("Sending '{}' coins to '{}'...", SEND_AMOUNT, RECV_ADDRESS);
    let transaction = account.send(SEND_AMOUNT, RECV_ADDRESS, None).await?;
    wait_for_inclusion(&transaction.transaction_id, &account).await?;

    sync_print_balance(&account, false).await?;

    println!("Example finished successfully");
    Ok(())
}

async fn create_wallet() -> Result<Wallet> {
    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;
    Wallet::builder()
        .load_storage::<SecretManager>(std::env::var("WALLET_DB_PATH").unwrap())
        .await?
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await
}

async fn print_accounts(wallet: &Wallet) -> Result<()> {
    let accounts = wallet.get_accounts().await?;
    println!("Accounts:");
    for account in accounts {
        let details = account.details().await;
        println!("- {}", details.alias());
    }
    Ok(())
}

async fn generate_addresses(account: &Account, max: usize) -> Result<()> {
    if account.addresses().await?.len() < max {
        let num_addresses_to_generate = max - account.addresses().await?.len();
        println!("Generating {num_addresses_to_generate} addresses ...");
        let now = tokio::time::Instant::now();
        account
            .generate_ed25519_addresses(num_addresses_to_generate as u32, None)
            .await?;
        println!("Finished in: {:.2?}", now.elapsed());
    }
    Ok(())
}

async fn print_addresses(account: &Account) -> Result<()> {
    let addresses = account.addresses().await?;
    println!("{}'s addresses:", account.alias().await);
    for address in addresses {
        println!("- {}", address.address());
    }
    Ok(())
}

async fn sync_print_balance(account: &Account, full_report: bool) -> Result<()> {
    let alias = account.alias().await;
    let now = tokio::time::Instant::now();
    let balance = account.sync(None).await?;
    println!("{alias}'s account synced in: {:.2?}", now.elapsed());
    if full_report {
        println!("{alias}'s balance:\n{balance:#?}");
    } else {
        println!("{alias}'s coin balance:\n{:#?}", balance.base_coin());
    }
    Ok(())
}

async fn print_addresses_with_funds(account: &Account) -> Result<()> {
    let addresses_with_unspent_outputs = account.addresses_with_unspent_outputs().await?;
    println!(
        "{}'s addresses with funds/assets: {}",
        account.alias().await,
        addresses_with_unspent_outputs.len()
    );
    for address_with_unspent_outputs in addresses_with_unspent_outputs {
        println!("- {}", address_with_unspent_outputs.address());
        println!("  Output Ids:");
        for output_id in address_with_unspent_outputs.output_ids() {
            println!("  {}", output_id);
        }
    }
    Ok(())
}

async fn wait_for_inclusion(transaction_id: &TransactionId, account: &Account) -> Result<()> {
    println!(
        "Transaction sent: {}/transaction/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction_id
    );
    // Wait for transaction to get included
    let block_id = account
        .reissue_transaction_until_included(transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );
    Ok(())
}
