// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will list all transaction of a wallet.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example list_transactions
//! ```

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::secret::stronghold::StrongholdSecretManager,
    wallet::{Result, SyncOptions},
    Wallet,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Setup Stronghold secret_manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build(std::env::var("STRONGHOLD_SNAPSHOT_PATH").unwrap())?;

    // Only required the first time, can also be generated with `manager.generate_mnemonic()?`
    let mnemonic = Mnemonic::from(std::env::var("MNEMONIC").unwrap());

    // The mnemonic only needs to be stored the first time
    secret_manager.store_mnemonic(mnemonic).await?;

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish(&secret_manager)
        .await?;

    // Sync wallet
    wallet
        .sync(
            &secret_manager,
            Some(SyncOptions {
                sync_incoming_transactions: true,
                ..Default::default()
            }),
        )
        .await?;

    // Print transaction ids
    println!("Sent transactions:");
    for transaction in wallet.transactions().await {
        println!("{}", transaction.transaction_id);
    }

    // Print received transaction ids
    println!("Received transactions:");
    for transaction in wallet.incoming_transactions().await {
        println!("{}", transaction.transaction_id);
    }

    Ok(())
}
