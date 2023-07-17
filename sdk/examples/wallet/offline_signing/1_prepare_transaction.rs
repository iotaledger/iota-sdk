// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will get inputs and prepare a transaction.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example 1_prepare_transaction
//! ```

use iota_sdk::{
    client::{
        api::{PreparedTransactionData, PreparedTransactionDataDto},
        constants::SHIMMER_COIN_TYPE,
        secret::SecretManager,
    },
    wallet::{account::types::AccountAddress, ClientOptions, Result, SendParams, Wallet},
};

const ONLINE_WALLET_DB_PATH: &str = "./examples/wallet/offline_signing/example-online-walletdb";
const ADDRESSES_FILE_PATH: &str = "./examples/wallet/offline_signing/example.addresses.json";
const PREPARED_TRANSACTION_FILE_PATH: &str = "./examples/wallet/offline_signing/example.prepared_transaction.json";
// Address to which we want to send the amount.
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";
// The amount to send.
const SEND_AMOUNT: u64 = 1_000_000;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let params = [SendParams::new(SEND_AMOUNT, RECV_ADDRESS)?];

    // Recovers addresses from example `0_address_generation`.
    let addresses = read_addresses_from_file().await?;

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    // Create the wallet with the secret_manager and client options
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Placeholder)
        .with_storage_path(ONLINE_WALLET_DB_PATH)
        .with_client_options(client_options.clone())
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Create a new account
    let account = wallet
        .create_account()
        .with_alias("Alice")
        .with_addresses(addresses)
        .finish()
        .await?;

    // Sync the account to get the outputs for the addresses
    account.sync(None).await?;

    let prepared_transaction = account.prepare_send(params.clone(), None).await?;

    println!("Prepared transaction sending {params:?}");

    write_transaction_to_file(prepared_transaction).await?;

    Ok(())
}

async fn read_addresses_from_file() -> Result<Vec<AccountAddress>> {
    use tokio::io::AsyncReadExt;

    let mut file = tokio::io::BufReader::new(tokio::fs::File::open(ADDRESSES_FILE_PATH).await?);
    let mut json = String::new();
    file.read_to_string(&mut json).await?;

    Ok(serde_json::from_str(&json)?)
}

async fn write_transaction_to_file(prepared_transaction: PreparedTransactionData) -> Result<()> {
    use tokio::io::AsyncWriteExt;

    let json = serde_json::to_string_pretty(&PreparedTransactionDataDto::from(&prepared_transaction))?;
    let mut file = tokio::io::BufWriter::new(tokio::fs::File::create(PREPARED_TRANSACTION_FILE_PATH).await?);
    println!("example.prepared_transaction.json:\n{json}");
    file.write_all(json.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}
