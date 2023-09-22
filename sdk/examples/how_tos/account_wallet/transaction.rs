// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we use an account as wallet.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example account_wallet_transaction`

use iota_sdk::{
    client::node_api::indexer::query_parameters::QueryParameter,
    types::block::address::{AccountAddress, ToBech32Ext},
    wallet::{
        account::{AliasSyncOptions, SyncOptions, TransactionOptions},
        Result,
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
        .with_alias("Alice")
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
        .basic_output_ids([QueryParameter::Address(account_address)])
        .await?
        .items
        .first()
        .unwrap();

    let transaction = wallet
        .send(
            1_000_000,
            "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
            TransactionOptions {
                mandatory_inputs: Some(vec![input]),
                ..Default::default()
            },
        )
        .await?;
    wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Transaction with custom input: https://explorer.shimmer.network/testnet/transaction/{}",
        transaction.transaction_id
    );

    let total_base_token_balance = wallet.sync(Some(sync_options)).await?.base_coin().total();
    println!("Balance after sending funds from account: {total_base_token_balance:#?}");

    Ok(())
}
