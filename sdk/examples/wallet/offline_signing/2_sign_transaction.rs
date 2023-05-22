// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we sign the prepared transaction.
//!
//! `cargo run --example 2_sign_transaction --release`

use std::{
    fs::File,
    io::{prelude::*, BufWriter},
    path::Path,
};

use iota_sdk::{
    client::{
        api::{
            transaction::validate_transaction_payload_length, PreparedTransactionData, PreparedTransactionDataDto,
            SignedTransactionData, SignedTransactionDataDto,
        },
        secret::{stronghold::StrongholdSecretManager, SecretManager, SignTransactionEssence},
    },
    types::block::{output::RentStructureBuilder, payload::TransactionPayload, protocol::ProtocolParameters},
    wallet::Result,
};

const PREPARED_TRANSACTION_FILE_NAME: &str = "examples/wallet/offline_signing/prepared_transaction.json";
const SIGNED_TRANSACTION_FILE_NAME: &str = "examples/wallet/offline_signing/signed_transaction.json";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Setup Stronghold secret_manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build("examples/wallet/offline_signing/offline_signing.stronghold")?;

    // Load snapshot file
    secret_manager.read_stronghold_snapshot().await?;

    // TODO: read from file, similar to https://github.com/iotaledger/iota.rs/issues/1267
    // Make sure that these values match the network you use.
    let protocol_parameters = ProtocolParameters::new(
        2,
        String::from("testnet"),
        String::from("smr"),
        1500,
        15,
        RentStructureBuilder::new()
            .byte_cost(100)
            .byte_factor_key(1)
            .byte_factor_data(10)
            .finish(),
        1813620509061365,
    )
    .unwrap();

    let prepared_transaction_data =
        read_prepared_transaction_from_file(PREPARED_TRANSACTION_FILE_NAME, &protocol_parameters)?;

    // Signs prepared transaction offline.
    let unlocks = SecretManager::Stronghold(secret_manager)
        .sign_transaction_essence(&prepared_transaction_data, None)
        .await?;
    let signed_transaction = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlocks)?;

    validate_transaction_payload_length(&signed_transaction)?;

    let signed_transaction_data = SignedTransactionData {
        transaction_payload: signed_transaction,
        inputs_data: prepared_transaction_data.inputs_data,
    };

    println!("Signed transaction.");

    write_signed_transaction_to_file(SIGNED_TRANSACTION_FILE_NAME, &signed_transaction_data)?;

    Ok(())
}

fn read_prepared_transaction_from_file<P: AsRef<Path>>(
    path: P,
    protocol_parameters: &ProtocolParameters,
) -> Result<PreparedTransactionData> {
    let mut file = File::open(&path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;

    Ok(PreparedTransactionData::try_from_dto(
        &serde_json::from_str::<PreparedTransactionDataDto>(&json)?,
        protocol_parameters,
    )?)
}

fn write_signed_transaction_to_file<P: AsRef<Path>>(
    path: P,
    signed_transaction_data: &SignedTransactionData,
) -> Result<()> {
    let dto = SignedTransactionDataDto::from(signed_transaction_data);
    let json = serde_json::to_string_pretty(&dto)?;
    let mut file = BufWriter::new(File::create(path)?);

    println!("{json}");

    file.write_all(json.as_bytes())?;

    Ok(())
}
