// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we send the signed transaction in a block.
//!
//! Make sure to run `2_transaction_signing` before.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example 3_send_block
//! ```

use iota_sdk::{
    client::{
        api::{verify_semantic, SignedTransactionData, SignedTransactionDataDto},
        Client, Error, Result,
    },
    types::block::{payload::Payload, semantic::ConflictReason},
};

const SIGNED_TRANSACTION_FILE_NAME: &str = "examples/client/offline_signing/signed_transaction.json";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a node client.
    let online_client = Client::builder()
        // Insert your node URL in the .env.
        .with_node(&node_url)?
        .finish()
        .await?;

    let signed_transaction_payload = read_signed_transaction_from_file(SIGNED_TRANSACTION_FILE_NAME).await?;

    let current_time = online_client.get_time_checked().await?;

    let conflict = verify_semantic(
        &signed_transaction_payload.inputs_data,
        &signed_transaction_payload.transaction_payload,
        current_time,
    )?;

    if conflict != ConflictReason::None {
        return Err(Error::TransactionSemantic(conflict));
    }

    // Sends the offline signed transaction online.
    let block = online_client
        .block()
        .finish_block(Some(Payload::Transaction(Box::new(
            signed_transaction_payload.transaction_payload,
        ))))
        .await?;

    println!(
        "Posted block: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}

async fn read_signed_transaction_from_file(path: impl AsRef<std::path::Path>) -> Result<SignedTransactionData> {
    use tokio::io::AsyncReadExt;

    let mut file = tokio::fs::File::open(path).await.expect("failed to open file");
    let mut json = String::new();
    file.read_to_string(&mut json).await.expect("failed to read file");

    let dto = serde_json::from_str::<SignedTransactionDataDto>(&json)?;

    Ok(SignedTransactionData::try_from_dto_unverified(dto)?)
}
