// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create addresses with a ledger nano hardware wallet.
//! To use the ledger nano simulator clone https://github.com/iotaledger/ledger-shimmer-app, run `git submodule init && git submodule update --recursive`,
//! then `./build.sh -m nanos|nanox|nanosplus -s` and use `true` in `LedgerSecretManager::new(true)`.
//!
//! `cargo run --example ledger_nano --release --features=ledger_nano`

use std::time::Instant;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{ledger_nano::LedgerSecretManager, SecretManager},
    },
    wallet::{AddressWithAmount, ClientOptions, Result, Wallet},
};

// the account alias used in this example
const ACCOUNT_ALIAS: &str = "ledger";
// the number of addresses to generate
const NUM_ADDRESSES_TO_GENERATE: u32 = 1;
// the address to send to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";
// the amount to send
const SEND_AMOUNT: u64 = 1_000_000;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;
    let secret_manager = LedgerSecretManager::new(true);
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::LedgerNano(secret_manager))
        .with_storage_path("ledger_nano_walletdb")
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    println!("{:?}", wallet.get_ledger_nano_status().await?);

    // Get account or create a new one
    let account = if let Ok(account) = wallet.get_account(ACCOUNT_ALIAS).await {
        account
    } else {
        // first we'll create an example account and store it
        wallet
            .create_account()
            .with_alias(ACCOUNT_ALIAS.to_string())
            .finish()
            .await?
    };

    let address = account.generate_addresses(NUM_ADDRESSES_TO_GENERATE, None).await?;

    println!("{address:?}");

    let now = Instant::now();
    let balance = account.sync(None).await?;
    println!("Syncing took: {:.2?}", now.elapsed());
    println!("Balance: {balance:?}");

    // send transaction
    let outputs = vec![AddressWithAmount::new(RECV_ADDRESS.to_string(), SEND_AMOUNT)];
    let transaction = account.send_amount(outputs, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    Ok(())
}
