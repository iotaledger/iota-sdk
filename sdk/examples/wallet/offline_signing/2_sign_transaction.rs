// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we sign the prepared transaction.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example 2_sign_transaction
//! ```

use iota_sdk::{
    client::{
        api::{
            transaction::validate_transaction_payload_length, PreparedTransactionData, PreparedTransactionDataDto,
            SignedTransactionData, SignedTransactionDataDto,
        },
        secret::{stronghold::StrongholdSecretManager, SecretManage, SecretManager},
    },
    types::{block::payload::TransactionPayload, TryFromDto},
    wallet::Result,
};

const STRONGHOLD_SNAPSHOT_PATH: &str = "./examples/wallet/offline_signing/example.stronghold";
const PREPARED_TRANSACTION_FILE_PATH: &str = "./examples/wallet/offline_signing/example.prepared_transaction.json";
const SIGNED_TRANSACTION_FILE_PATH: &str = "./examples/wallet/offline_signing/example.signed_transaction.json";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Setup Stronghold secret_manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build(STRONGHOLD_SNAPSHOT_PATH)?;

    let prepared_transaction_data = read_prepared_transaction_from_file().await?;

    // Signs prepared transaction offline.
    let unlocks = SecretManager::Stronghold(secret_manager)
        .sign_transaction_essence(&prepared_transaction_data, None)
        .await?;

    let signed_transaction = TransactionPayload::new(prepared_transaction_data.essence.as_regular().clone(), unlocks)?;

    validate_transaction_payload_length(&signed_transaction)?;

    let signed_transaction_data = SignedTransactionData {
        transaction_payload: signed_transaction,
        inputs_data: prepared_transaction_data.inputs_data,
    };

    println!("Signed transaction.");

    write_signed_transaction_to_file(&signed_transaction_data).await?;

    Ok(())
}

async fn read_prepared_transaction_from_file() -> Result<PreparedTransactionData> {
    use tokio::io::AsyncReadExt;

    let mut file = tokio::io::BufReader::new(tokio::fs::File::open(PREPARED_TRANSACTION_FILE_PATH).await?);
    let mut json = String::new();
    file.read_to_string(&mut json).await?;

    Ok(PreparedTransactionData::try_from_dto(serde_json::from_str::<
        PreparedTransactionDataDto,
    >(&json)?)?)
}

async fn write_signed_transaction_to_file(signed_transaction_data: &SignedTransactionData) -> Result<()> {
    use tokio::io::AsyncWriteExt;

    let dto = SignedTransactionDataDto::from(signed_transaction_data);
    let json = serde_json::to_string_pretty(&dto)?;
    let mut file = tokio::io::BufWriter::new(tokio::fs::File::create(SIGNED_TRANSACTION_FILE_PATH).await?);
    println!("example.signed_transaction.json:\n{json}");
    file.write_all(json.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}
