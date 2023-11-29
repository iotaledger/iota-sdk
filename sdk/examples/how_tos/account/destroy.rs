// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will try to destroy the first account output there is in the wallet.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_wallet.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example destroy_account_output
//! ```

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::secret::stronghold::StrongholdSecretManager,
    wallet::{Result, Wallet},
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

    // May want to ensure the wallet is synced before sending a transaction.
    let balance = wallet.sync(&secret_manager, None).await?;

    // Get the first account
    if let Some(account_id) = balance.accounts().first() {
        let accounts_before = balance.accounts();
        println!("Accounts BEFORE destroying:\n{accounts_before:#?}",);

        // Set the stronghold password
        secret_manager
            .set_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        println!("Sending account burn transaction...");

        let transaction = wallet.burn(&secret_manager, *account_id, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        let block_id = wallet
            .reissue_transaction_until_included(&secret_manager, &transaction.transaction_id, None, None)
            .await?;

        println!(
            "Block included: {}/block/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            block_id
        );

        println!("Burned Account '{}'", account_id);

        let balance = wallet.sync(&secret_manager, None).await?;

        let accounts_after = balance.accounts();
        println!("Accounts AFTER destroying:\n{accounts_after:#?}",);
    } else {
        println!("No Account available");
    }

    Ok(())
}
