// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will verify the integrity of the wallet database.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example storage
//! ```

use iota_sdk::{
    client::{constants::SHIMMER_COIN_TYPE, secret::mnemonic::MnemonicSecretManager},
    crypto::keys::bip44::Bip44,
    wallet::{ClientOptions, Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let secret_manager = MnemonicSecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .finish(&secret_manager)
        .await?;

    let bech32_address = wallet.address().await;

    println!("ADDRESS:\n{bech32_address}");

    sync_print_balance(&wallet, &secret_manager).await?;

    println!("Example finished successfully");
    Ok(())
}

async fn sync_print_balance(wallet: &Wallet, secret_manager: &MnemonicSecretManager) -> Result<()> {
    let now = tokio::time::Instant::now();
    let balance = wallet.sync(secret_manager, None).await?;
    println!("Wallet synced in: {:.2?}", now.elapsed());
    println!("Balance:\n{:#?}", balance.base_coin());
    Ok(())
}
