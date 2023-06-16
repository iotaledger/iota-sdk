// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we get inputs and prepare a transaction.
//!
//! Make sure to run `0_address_generation` before.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example 1_transaction_preparation [RECV ADDRESS] [AMOUNT]
//! ```

use std::{env, path::Path};

use iota_sdk::{
    client::{
        api::{PreparedTransactionData, PreparedTransactionDataDto},
        Client, Result,
    },
    types::block::address::Bech32Address,
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
};

const ADDRESS_FILE_NAME: &str = "examples/client/offline_signing/address.json";
const PREPARED_TRANSACTION_FILE_NAME: &str = "examples/client/offline_signing/prepared_transaction.json";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = env::var("NODE_URL").unwrap();

    // Address to which we want to send the amount.
    let recv_address = env::args()
        .nth(1)
        .unwrap_or_else(|| "rms1qruzprxum2934lr3p77t96pzlecxv8pjzvtjrzdcgh2f5exa22n6ga0vm69".to_string());
    let recv_address = recv_address.as_str();

    // The amount to send.
    let amount = env::args()
        .nth(2)
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap_or(1_000_000u64);

    // Create a client instance.
    let online_client = Client::builder()
        // Insert your node URL in the .env.
        .with_node(&node_url)?
        .finish()
        .await?;

    // Recovers addresses from the previous example.
    let addresses = read_addresses_from_file(ADDRESS_FILE_NAME).await?;

    // Gets enough inputs related to these addresses to cover the amount.
    let inputs = online_client.find_inputs(addresses, amount).await?;

    // Prepares the transaction.
    let mut transaction_builder = online_client.block();
    for input in inputs {
        transaction_builder = transaction_builder.with_input(input)?;
    }
    let prepared_transaction = transaction_builder
        .with_output(recv_address, amount)
        .await?
        .prepare_transaction()
        .await?;

    println!("Prepared transaction sending {amount} to {recv_address}.");

    write_prepared_transaction_to_file(PREPARED_TRANSACTION_FILE_NAME, &prepared_transaction).await?;

    Ok(())
}

async fn read_addresses_from_file(path: impl AsRef<Path>) -> Result<Vec<Bech32Address>> {
    let mut file = File::open(&path).await.expect("failed to open file");
    let mut json = String::new();
    file.read_to_string(&mut json).await.expect("failed to read file");

    Ok(serde_json::from_str(&json)?)
}

async fn write_prepared_transaction_to_file(
    path: impl AsRef<Path>,
    prepared_transaction: &PreparedTransactionData,
) -> Result<()> {
    let json = serde_json::to_string_pretty(&PreparedTransactionDataDto::from(prepared_transaction))?;
    let mut file = BufWriter::new(File::create(path).await.expect("failed to create file"));

    println!("{json}");

    file.write_all(json.as_bytes()).await.expect("failed to write file");

    Ok(())
}
