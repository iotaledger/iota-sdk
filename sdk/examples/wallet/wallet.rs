// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will:
//! * create a wallet from a mnemonic phrase
//! * print the wallet address (as Bech32)
//! * print funds on the wallet address
//! * issue a coin transaction
//!
//! Make sure there's no `STRONGHOLD_SNAPSHOT_PATH` file and no `WALLET_DB_PATH` folder yet!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example wallet
//! ```

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    types::block::payload::transaction::TransactionId,
    wallet::{ClientOptions, Result, Wallet},
};

// The amount of coins to send
const SEND_AMOUNT: u64 = 1_000_000;
// The address to send the coins to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = create_wallet().await?;

    // Change to `true` to print the full balance report
    sync_print_balance(&wallet, false).await?;

    println!("Sending '{}' coins to '{}'...", SEND_AMOUNT, RECV_ADDRESS);
    let transaction = wallet.send(SEND_AMOUNT, RECV_ADDRESS, None).await?;
    wait_for_inclusion(&transaction.transaction_id, &wallet).await?;

    sync_print_balance(&wallet, false).await?;

    println!("Example finished successfully");
    Ok(())
}

async fn create_wallet() -> Result<Wallet> {
    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;
    Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .with_alias("Alice")
        .finish()
        .await
}

async fn print_address(wallet: &Wallet) -> Result<()> {
    println!(
        "{}'s wallet address: {}",
        wallet.alias().await,
        wallet.address_as_bech32().await
    );
    Ok(())
}

async fn sync_print_balance(wallet: &Wallet, full_report: bool) -> Result<()> {
    let alias = wallet.alias().await;
    let now = tokio::time::Instant::now();
    let balance = wallet.sync(None).await?;
    println!("{alias}'s account synced in: {:.2?}", now.elapsed());
    if full_report {
        println!("{alias}'s balance:\n{balance:#?}");
    } else {
        println!("{alias}'s coin balance:\n{:#?}", balance.base_coin());
    }
    Ok(())
}

async fn wait_for_inclusion(transaction_id: &TransactionId, wallet: &Wallet) -> Result<()> {
    println!(
        "Transaction sent: {}/transaction/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction_id
    );
    // Wait for transaction to get included
    let block_id = wallet
        .reissue_transaction_until_included(transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );
    Ok(())
}
