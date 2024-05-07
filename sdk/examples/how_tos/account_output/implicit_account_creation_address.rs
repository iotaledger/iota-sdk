// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example, we create an implicit account creation address.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example implicit_account_creation_address
//! ```

use iota_sdk::{
    client::{constants::SHIMMER_COIN_TYPE, secret::SecretManager},
    crypto::keys::bip44::Bip44,
    wallet::{ClientOptions, Wallet},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //  This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;
    let client_options = ClientOptions::new().with_node("https://api.testnet.shimmer.network")?;

    let wallet = Wallet::builder()
        .with_secret_manager(secret_manager)
        .with_client_options(client_options)
        .with_storage_path("implicit_account_creation")
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .finish()
        .await?;

    let implicit_account_creation_address = wallet.implicit_account_creation_address().await?;

    println!("{implicit_account_creation_address}");

    Ok(())
}
