// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we use an alias as wallet.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example alias_wallet_request_funds`

use std::env::var;

use iota_sdk::{
    client::request_funds_from_faucet,
    types::block::address::{AccountAddress, ToBech32Ext},
    wallet::{
        account::{AliasSyncOptions, SyncOptions},
        Result,
    },
    Wallet,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let faucet_url = var("FAUCET_URL").unwrap();

    // Create the wallet
    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // Get the account
    let account = wallet.get_account("Alice").await?;
    let balance = account.sync(None).await?;

    let total_base_token_balance = balance.base_coin().total();
    println!("Balance before requesting funds on alias address: {total_base_token_balance:#?}");

    let alias_id = balance.aliases().first().unwrap();
    println!("Alias Id: {alias_id}");

    // Get alias address
    let alias_address = AccountAddress::new(*alias_id).to_bech32(account.client().get_bech32_hrp().await.unwrap());
    let faucet_response = request_funds_from_faucet(&faucet_url, &alias_address).await?;

    println!("{faucet_response}");

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    let sync_options = SyncOptions {
        alias: AliasSyncOptions {
            basic_outputs: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let total_base_token_balance = account.sync(Some(sync_options)).await?.base_coin().total();
    println!("Balance after requesting funds on alias address: {total_base_token_balance:#?}");

    Ok(())
}
