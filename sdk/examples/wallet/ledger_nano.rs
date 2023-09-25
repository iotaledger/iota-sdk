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
    crypto::keys::bip44::Bip44,
    wallet::{ClientOptions, Result, Wallet},
};

// The account alias used in this example
const ALIAS: &str = "ledger";
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
        .with_secret_manager(SecretManager::LedgerNano(secret_manager))
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .with_alias(ALIAS)
        .finish()
        .await?;

    println!("{:?}", wallet.get_ledger_nano_status().await?);

    println!("Generating address...");
    let now = tokio::time::Instant::now();
    let address = wallet.generate_ed25519_address(None).await?;
    println!("took: {:.2?}", now.elapsed());

    println!("ADDRESS:\n{address:#?}");

    let now = tokio::time::Instant::now();
    let balance = wallet.sync(None).await?;
    println!("Wallet synced in: {:.2?}", now.elapsed());

    println!("Balance BEFORE:\n{:?}", balance.base_coin());

    println!("Sending the coin-transfer transaction...");
    let transaction = wallet.send(SEND_AMOUNT, RECV_ADDRESS, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    let now = tokio::time::Instant::now();
    let balance = wallet.sync(None).await?;
    println!("Wallet synced in: {:.2?}", now.elapsed());

    println!("Balance AFTER:\n{:?}", balance.base_coin());

    Ok(())
}
