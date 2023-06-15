// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we use an alias as wallet.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example alias_wallet_transaction`

use std::env::var;

use iota_sdk::{
    client::node_api::indexer::query_parameters::QueryParameter,
    types::block::address::{AliasAddress, ToBech32Ext},
    wallet::{
        account::{AliasSyncOptions, SyncOptions, TransactionOptions},
        Result, SendAmountParams,
    },
    Wallet,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let sync_options = SyncOptions {
        alias: AliasSyncOptions {
            basic_outputs: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Create the wallet
    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    wallet
        .set_stronghold_password(var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Get the account
    let account = wallet.get_account("Alice").await?;
    let balance = account.sync(Some(sync_options.clone())).await?;

    let total_base_token_balance = balance.base_coin().total();
    println!("Balance before sending funds from alias: {total_base_token_balance:#?}");

    let alias_id = balance.aliases().first().unwrap();
    println!("Alias Id: {alias_id}");

    // Get alias address
    let alias_address = AliasAddress::new(*alias_id).to_bech32(account.client().get_bech32_hrp().await.unwrap());

    // Find first output unlockable by the alias address
    let input = account
        .client()
        .basic_output_ids([QueryParameter::Address(alias_address)])
        .await?
        .items
        .first()
        .unwrap()
        .clone();

    let transaction = account
        .send_amount(
            SendAmountParams::new(
                "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
                1_000_000,
            ),
            TransactionOptions {
                mandatory_inputs: Some(vec![input]),
                ..Default::default()
            },
        )
        .await?;
    account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Transaction with custom input: https://explorer.shimmer.network/testnet/transaction/{}",
        transaction.transaction_id
    );

    let total_base_token_balance = account.sync(Some(sync_options)).await?.base_coin().total();
    println!("Balance after sending funds from alias: {total_base_token_balance:#?}");

    Ok(())
}
