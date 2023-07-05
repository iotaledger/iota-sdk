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
        Client,
    },
    types::block::payload::transaction::TransactionId,
    wallet::{Account, Result},
    Wallet,
};

const ONLINE_WALLET_DB_PATH: &str = "./examples/wallet/offline_signing/example-online-walletdb";
const SIGNED_TRANSACTION_FILE_PATH: &str = "./examples/wallet/offline_signing/example.signed_transaction.json";

#[tokio::main]
async fn main() -> Result<()> {
    // Create the wallet with the secret_manager and client options
    let wallet = Wallet::builder()
        .with_storage_path(ONLINE_WALLET_DB_PATH)
        .finish()
        .await?;

    // Create a new account
    let account = wallet.get_account("Alice").await?;

    let signed_transaction_data = read_signed_transaction_from_file(account.client()).await?;

    // Sends offline signed transaction online.
    let transaction = account
        .submit_and_store_transaction(signed_transaction_data, None)
        .await?;
    wait_for_inclusion(&transaction.transaction_id, &account).await?;

    Ok(())
}

async fn read_signed_transaction_from_file(client: &Client) -> Result<SignedTransactionData> {
    use tokio::io::AsyncReadExt;

    let mut file = tokio::io::BufReader::new(tokio::fs::File::open(SIGNED_TRANSACTION_FILE_PATH).await?);
    let mut json = String::new();
    file.read_to_string(&mut json).await?;

    let dto = serde_json::from_str::<SignedTransactionDataDto>(&json)?;

    Ok(SignedTransactionData::try_from_dto(
        dto,
        &client.get_protocol_parameters().await?,
    )?)
}

async fn wait_for_inclusion(transaction_id: &TransactionId, account: &Account) -> Result<()> {
    println!(
        "Transaction sent: {}/transaction/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction_id
    );
    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );
    Ok(())
}
