// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we use an account as wallet.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example account_wallet_request_funds`

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::{request_funds_from_faucet, secret::stronghold::StrongholdSecretManager},
    types::block::address::{AccountAddress, ToBech32Ext},
    wallet::{AccountSyncOptions, Result, SyncOptions},
    Wallet,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let faucet_url = std::env::var("FAUCET_URL").unwrap();

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

    let balance = wallet.sync(&secret_manager, None).await?;

    let total_base_token_balance = balance.base_coin().total();
    println!("Balance before requesting funds on wallet address: {total_base_token_balance:#?}");

    let account_id = balance.accounts().first().unwrap();
    println!("Account Id: {account_id}");

    // Get account address
    let account_address = AccountAddress::new(*account_id).to_bech32(wallet.client().get_bech32_hrp().await.unwrap());
    let faucet_response = request_funds_from_faucet(&faucet_url, &account_address).await?;

    println!("{faucet_response}");

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    let sync_options = SyncOptions {
        account: AccountSyncOptions {
            basic_outputs: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let total_base_token_balance = wallet
        .sync(&secret_manager, Some(sync_options))
        .await?
        .base_coin()
        .total();
    println!("Balance after requesting funds on account address: {total_base_token_balance:#?}");

    Ok(())
}
