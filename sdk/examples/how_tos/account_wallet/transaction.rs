// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we use an account as wallet.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example account_wallet_transaction`

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::{
        node_api::indexer::query_parameters::BasicOutputQueryParameters,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
    },
    types::block::address::{AccountAddress, ToBech32Ext},
    wallet::{AccountSyncOptions, Result, SyncOptions, TransactionOptions},
    Wallet,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let sync_options = SyncOptions {
        account: AccountSyncOptions {
            basic_outputs: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Setup Stronghold secret_manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build(std::env::var("STRONGHOLD_SNAPSHOT_PATH").unwrap())?;

    // Only required the first time, can also be generated with `manager.generate_mnemonic()?`
    let mnemonic = Mnemonic::from(std::env::var("MNEMONIC").unwrap());

    // The mnemonic only needs to be stored the first time
    secret_manager.store_mnemonic(mnemonic).await?;

    // Create the wallet
    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish(&secret_manager)
        .await?;

    let balance = wallet.sync(&secret_manager, Some(sync_options.clone())).await?;

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
            &secret_manager,
            1_000_000,
            "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
            TransactionOptions {
                mandatory_inputs: Some(vec![input]),
                ..Default::default()
            },
        )
        .await?;
    wallet
        .reissue_transaction_until_included(&secret_manager, &transaction.transaction_id, None, None)
        .await?;
    println!(
        "Transaction with custom input: https://explorer.shimmer.network/testnet/transaction/{}",
        transaction.transaction_id
    );

    let total_base_token_balance = wallet
        .sync(&secret_manager, Some(sync_options))
        .await?
        .base_coin()
        .total();
    println!("Balance after sending funds from account: {total_base_token_balance:#?}");

    Ok(())
}
