// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we sync the wallet and get the balance.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_wallet.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example check_balance
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

    // Sync and get the balance
    let balance = wallet.sync(&secret_manager, None).await?;
    println!("{balance:#?}");

    let explorer_url = std::env::var("EXPLORER_URL").ok();
    let addr_base_url = explorer_url.map(|url| format!("{url}/addr/")).unwrap_or_default();
    println!("{addr_base_url}{}", wallet.address().await);

    Ok(())
}
