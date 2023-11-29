// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will claim all claimable outputs.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example claim_transaction`

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::secret::stronghold::StrongholdSecretManager,
    wallet::{OutputsToClaim, Result},
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

    // Get the wallet we generated with `create_wallet`.
    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish(&secret_manager)
        .await?;

    // May want to ensure the wallet is synced before sending a transaction.
    wallet.sync(&secret_manager, None).await?;

    let output_ids = wallet.claimable_outputs(OutputsToClaim::All).await?;
    println!("Available outputs to claim:");
    for output_id in &output_ids {
        println!("{}", output_id);
    }

    let transaction = wallet.claim_outputs(&secret_manager, output_ids).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = wallet
        .reissue_transaction_until_included(&secret_manager, &transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    Ok(())
}
