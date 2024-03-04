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
    types::block::payload::signed_transaction::TransactionId,
    wallet::{ClientOptions, Wallet},
};

// The amount of coins to send
const SEND_AMOUNT: u64 = 1_000_000;
// The address to send the coins to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL", "MNEMONIC", "WALLET_DB_PATH", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

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

async fn create_wallet() -> Result<Wallet, Box<dyn std::error::Error>> {
    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;
    Ok(Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .finish()
        .await?)
}

async fn sync_print_balance(wallet: &Wallet, full_report: bool) -> Result<(), Box<dyn std::error::Error>> {
    let now = tokio::time::Instant::now();
    let balance = wallet.sync(None).await?;
    println!("Wallet synced in: {:.2?}", now.elapsed());
    if full_report {
        println!("Balance:\n{balance:#?}");
    } else {
        println!("Coin balance:\n{:#?}", balance.base_coin());
    }
    Ok(())
}

async fn wait_for_inclusion(transaction_id: &TransactionId, wallet: &Wallet) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Transaction sent: {}/transaction/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction_id
    );
    wallet
        .wait_for_transaction_acceptance(transaction_id, None, None)
        .await?;

    println!(
        "Tx accepted: {}/transactions/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction_id
    );

    Ok(())
}
