// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create addresses with a ledger nano hardware wallet.
//!
//! To use the ledger nano simulator
//! * clone https://github.com/iotaledger/ledger-shimmer-app,
//! * run `git submodule init && git submodule update --recursive`,
//! * run `./build.sh -m nanos|nanox|nanosplus -s`, and
//! * use `true` in `LedgerSecretManager::new(true)`.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example ledger_nano
//! ```

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{ledger_nano::LedgerSecretManager, SecretManager},
    },
    wallet::{ClientOptions, Result, Wallet},
};

// The account alias used in this example
const ACCOUNT_ALIAS: &str = "ledger";
// The number of addresses to generate
const NUM_ADDRESSES_TO_GENERATE: u32 = 1;
// The address to send coins to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";
// The amount of base coins we'll send
const SEND_AMOUNT: u64 = 1_000_000;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;
    let secret_manager = LedgerSecretManager::new(true);
    let wallet = Wallet::builder()
        .load_storage::<SecretManager>(std::env::var("WALLET_DB_PATH").unwrap())
        .await?
        .with_secret_manager(SecretManager::LedgerNano(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    println!("{:?}", wallet.get_ledger_nano_status().await?);

    // Get or create a new account
    let account = wallet.get_or_create_account(ACCOUNT_ALIAS).await?;

    println!("Generating {NUM_ADDRESSES_TO_GENERATE} addresses...");
    let now = tokio::time::Instant::now();
    let addresses = account
        .generate_ed25519_addresses(NUM_ADDRESSES_TO_GENERATE, None)
        .await?;
    println!("took: {:.2?}", now.elapsed());

    println!("ADDRESSES:\n{addresses:#?}");

    let now = tokio::time::Instant::now();
    let balance = account.sync(None).await?;
    println!("Account synced in: {:.2?}", now.elapsed());

    println!("Balance BEFORE:\n{:?}", balance.base_coin());

    println!("Sending the coin-transfer transaction...");
    let transaction = account.send(SEND_AMOUNT, RECV_ADDRESS, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = account
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    let now = tokio::time::Instant::now();
    let balance = account.sync(None).await?;
    println!("Account synced in: {:.2?}", now.elapsed());

    println!("Balance AFTER:\n{:?}", balance.base_coin());

    Ok(())
}
