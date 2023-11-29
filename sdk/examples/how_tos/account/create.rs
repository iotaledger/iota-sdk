// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create an account output.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_wallet.rs` example and that funds are available by running
//! the `get_funds` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example create_account_output
//! ```

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::secret::stronghold::StrongholdSecretManager,
    wallet::{Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    //  This example uses secrets in environment variables for simplicity which should not be done in production.
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

    // May want to ensure the wallet is synced before sending a transaction.
    let balance = wallet.sync(&secret_manager, None).await?;
    println!("Accounts BEFORE:\n{:#?}", balance.accounts());

    println!("Sending the create-account transaction...");

    // Create an account output
    let transaction = wallet.create_account_output(&secret_manager, None, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = wallet
        .reissue_transaction_until_included(&secret_manager, &transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    let balance = wallet.sync(&secret_manager, None).await?;
    println!("Accounts AFTER:\n{:#?}", balance.accounts());

    Ok(())
}
