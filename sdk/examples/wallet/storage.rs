// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will verify the integrity of the wallet database.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example storage
//! ```

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    crypto::keys::bip44::Bip44,
    wallet::{account::types::Bip44Address, ClientOptions, Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let secret_manager = MnemonicSecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .with_alias("Alice")
        .finish()
        .await?;

    let bech32_address = wallet.address_as_bech32().await;

    println!("ADDRESS:\n{bech32_address}");

    sync_print_balance(&wallet).await?;

    // TODO: remove?

    // #[cfg(debug_assertions)]
    // wallet.verify_integrity().await?;

    println!("Example finished successfully");
    Ok(())
}

async fn sync_print_balance(wallet: &Wallet) -> Result<()> {
    let alias = wallet.alias().await;
    let now = tokio::time::Instant::now();
    let balance = wallet.sync(None).await?;
    println!("{alias}'s wallet synced in: {:.2?}", now.elapsed());
    println!("{alias}'s balance:\n{:#?}", balance.base_coin());
    Ok(())
}
