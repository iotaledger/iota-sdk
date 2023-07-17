// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we sign the prepared transaction.
//!
//! Make sure to run `1_transaction_preparation` before.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example 2_transaction_signing
//! ```

use iota_sdk::{
    client::{
        api::{PreparedTransactionData, PreparedTransactionDataDto, SignedTransactionData, SignedTransactionDataDto},
        secret::{SecretManage, SecretManager},
        Result,
    },
    types::{block::payload::transaction::TransactionPayload, TryFromDto},
};

const PREPARED_TRANSACTION_FILE_NAME: &str = "examples/client/offline_signing/prepared_transaction.json";
const SIGNED_TRANSACTION_FILE_NAME: &str = "examples/client/offline_signing/signed_transaction.json";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    let prepared_transaction_data = read_prepared_transaction_from_file(PREPARED_TRANSACTION_FILE_NAME).await?;

    // Signs the prepared transaction offline.
    let unlocks = secret_manager
        .sign_transaction_essence(&prepared_transaction_data, None)
        .await?;
    let signed_transaction = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlocks)?;

    let signed_transaction_data = SignedTransactionData {
        transaction_payload: signed_transaction,
        inputs_data: prepared_transaction_data.inputs_data,
    };

    println!("Signed transaction.");

    write_signed_transaction_to_file(SIGNED_TRANSACTION_FILE_NAME, &signed_transaction_data).await?;

    Ok(())
}

async fn read_prepared_transaction_from_file(path: impl AsRef<std::path::Path>) -> Result<PreparedTransactionData> {
    use tokio::io::AsyncReadExt;

    let mut file = tokio::fs::File::open(&path).await.expect("failed to open file");
    let mut json = String::new();
    file.read_to_string(&mut json).await.expect("failed to read file");

    Ok(PreparedTransactionData::try_from_dto(serde_json::from_str::<
        PreparedTransactionDataDto,
    >(&json)?)?)
}

async fn write_signed_transaction_to_file(
    path: impl AsRef<std::path::Path>,
    signed_transaction_data: &SignedTransactionData,
) -> Result<()> {
    use tokio::io::AsyncWriteExt;

    let dto = SignedTransactionDataDto::from(signed_transaction_data);
    let json = serde_json::to_string_pretty(&dto)?;
    let mut file = tokio::io::BufWriter::new(tokio::fs::File::create(path).await.expect("failed to create file"));
    println!("{json}");
    file.write_all(json.as_bytes()).await.expect("failed to write file");

    Ok(())
}
