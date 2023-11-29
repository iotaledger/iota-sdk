// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example, we create an implicit account creation address.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example implicit_account_creation
//! ```

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::{constants::SHIMMER_COIN_TYPE, secret::stronghold::StrongholdSecretManager},
    crypto::keys::bip44::Bip44,
    wallet::{ClientOptions, Result, Wallet},
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

    let client_options = ClientOptions::new().with_node("https://api.testnet.shimmer.network")?;

    let wallet = Wallet::builder()
        .with_client_options(client_options)
        .with_storage_path("implicit_account_creation")
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .finish(&secret_manager)
        .await?;

    let implicit_account_creation_address = wallet.implicit_account_creation_address().await?;

    println!("{implicit_account_creation_address}");

    Ok(())
}
