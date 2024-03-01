// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we use an account as wallet.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example account_output_send_amount`

use iota_sdk::{
    client::{api::options::TransactionOptions, node_api::indexer::query_parameters::BasicOutputQueryParameters},
    types::block::address::{AccountAddress, ToBech32Ext},
    wallet::{AccountSyncOptions, SyncOptions},
    Wallet,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH", "STRONGHOLD_PASSWORD", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    let sync_options = SyncOptions {
        account: AccountSyncOptions {
            basic_outputs: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Create the wallet
    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let balance = wallet.sync(Some(sync_options.clone())).await?;

    let total_base_token_balance = balance.base_coin().total();
    println!("Balance before sending funds from account: {total_base_token_balance:#?}");

    let account_id = balance.accounts().first().unwrap();
    println!("Account Id: {account_id}");

    // Get account address
    let account_address = AccountAddress::new(*account_id).to_bech32(wallet.client().get_bech32_hrp().await.unwrap());

    // Find first output unlockable by the account address
    let input = *wallet
        .client()
        .basic_output_ids(BasicOutputQueryParameters::only_address_unlock_condition(
            account_address,
        ))
        .await?
        .items
        .first()
        .unwrap();

    let transaction = wallet
        .send(
            1_000_000,
            "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
            TransactionOptions {
                required_inputs: [input].into(),
                ..Default::default()
            },
        )
        .await?;
    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;

    println!(
        "Transaction with custom input: {}/transaction/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction.transaction_id
    );

    let total_base_token_balance = wallet.sync(Some(sync_options)).await?.base_coin().total();
    println!("Balance after sending funds from account: {total_base_token_balance:#?}");

    Ok(())
}
