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
        api::{PreparedTransactionData, SignedTransactionData, SignedTransactionDataDto},
        secret::{stronghold::StrongholdSecretManager, SecretManage, SecretManager},
    },
    types::{block::payload::SignedTransactionPayload, TryFromDto},
};

const STRONGHOLD_SNAPSHOT_PATH: &str = "./examples/wallet/offline_signing/example.stronghold";
const PREPARED_TRANSACTION_FILE_PATH: &str = "./examples/wallet/offline_signing/example.prepared_transaction.json";
const PROTOCOL_PARAMETERS_FILE_PATH: &str = "./examples/wallet/offline_signing/example.protocol_parameters.json";
const SIGNED_TRANSACTION_FILE_PATH: &str = "./examples/wallet/offline_signing/example.signed_transaction.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["STRONGHOLD_PASSWORD"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Setup Stronghold secret_manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build(STRONGHOLD_SNAPSHOT_PATH)?;

    let prepared_transaction_data = PreparedTransactionData::try_from_dto(serde_json::from_str(
        &read_data_from_file(PREPARED_TRANSACTION_FILE_PATH).await?,
    )?)?;

    let protocol_parameters = serde_json::from_str(&read_data_from_file(PROTOCOL_PARAMETERS_FILE_PATH).await?)?;

    // Signs prepared transaction offline.
    let unlocks = SecretManager::Stronghold(secret_manager)
        .transaction_unlocks(&prepared_transaction_data, &protocol_parameters)
        .await?;

    let signed_transaction = SignedTransactionPayload::new(prepared_transaction_data.transaction.clone(), unlocks)?;

    let signed_transaction_data = SignedTransactionData {
        payload: signed_transaction,
        inputs_data: prepared_transaction_data.inputs_data,
        mana_rewards: prepared_transaction_data.mana_rewards,
        issuer_id: prepared_transaction_data.issuer_id,
    };

    println!("Signed transaction.");

    write_signed_transaction_to_file(&signed_transaction_data).await?;

    Ok(())
}

async fn read_data_from_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    use tokio::io::AsyncReadExt;

    let mut file = tokio::io::BufReader::new(tokio::fs::File::open(path).await?);
    let mut json = String::new();
    file.read_to_string(&mut json).await?;

    Ok(json)
}

async fn write_signed_transaction_to_file(
    signed_transaction_data: &SignedTransactionData,
) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::io::AsyncWriteExt;

    let dto = SignedTransactionDataDto::from(signed_transaction_data);
    let json = serde_json::to_string_pretty(&dto)?;
    let mut file = tokio::io::BufWriter::new(tokio::fs::File::create(SIGNED_TRANSACTION_FILE_PATH).await?);
    println!("example.signed_transaction.json:\n{json}");
    file.write_all(json.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}
