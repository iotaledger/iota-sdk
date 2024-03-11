// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we send the signed transaction in a block.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example 3_send_transaction
//! ```

use iota_sdk::{
    client::{
        api::{SignedTransactionData, SignedTransactionDataDto},
        secret::SecretManager,
        Client,
    },
    types::{block::payload::signed_transaction::TransactionId, TryFromDto},
    Wallet,
};

const ONLINE_WALLET_DB_PATH: &str = "./examples/wallet/offline_signing/example-online-walletdb";
const SIGNED_TRANSACTION_FILE_PATH: &str = "./examples/wallet/offline_signing/example.signed_transaction.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Create the wallet with the secret_manager and client options
    let wallet = Wallet::builder()
        .with_storage_path(ONLINE_WALLET_DB_PATH)
        .with_secret_manager(SecretManager::Placeholder)
        .finish()
        .await?;

    let signed_transaction_data = read_signed_transaction_from_file(wallet.client()).await?;

    // Sends offline signed transaction online.
    let transaction = wallet
        .submit_and_store_transaction(signed_transaction_data, None)
        .await?;
    wait_for_inclusion(&transaction.transaction_id, &wallet).await?;

    Ok(())
}

async fn read_signed_transaction_from_file(
    client: &Client,
) -> Result<SignedTransactionData, Box<dyn std::error::Error>> {
    use tokio::io::AsyncReadExt;

    let mut file = tokio::io::BufReader::new(tokio::fs::File::open(SIGNED_TRANSACTION_FILE_PATH).await?);
    let mut json = String::new();
    file.read_to_string(&mut json).await?;

    let dto = serde_json::from_str::<SignedTransactionDataDto>(&json)?;

    Ok(SignedTransactionData::try_from_dto_with_params(
        dto,
        &client.get_protocol_parameters().await?,
    )?)
}

async fn wait_for_inclusion(transaction_id: &TransactionId, wallet: &Wallet) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Transaction sent: {}/transaction/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction_id
    );
    wallet
        .wait_for_transaction_acceptance(transaction_id, None, None)
        .await?;

    println!(
        "Tx accepted: {}/transactions/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction_id
    );

    Ok(())
}
