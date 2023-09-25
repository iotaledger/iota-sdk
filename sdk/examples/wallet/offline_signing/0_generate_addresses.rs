// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we generate addresses which will be used later to find inputs.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example 0_generate_addresses
//! ```

use iota_sdk::{
    client::{
        constants::{SHIMMER_BECH32_HRP, SHIMMER_COIN_TYPE},
        secret::{stronghold::StrongholdSecretManager, SecretManager},
    },
    crypto::keys::{bip39::Mnemonic, bip44::Bip44},
    wallet::{ClientOptions, Result, Wallet},
};

const OFFLINE_WALLET_DB_PATH: &str = "./examples/wallet/offline_signing/example-offline-walletdb";
const STRONGHOLD_SNAPSHOT_PATH: &str = "./examples/wallet/offline_signing/example.stronghold";
const ADDRESS_FILE_PATH: &str = "./examples/wallet/offline_signing/example.address.json";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let offline_client = ClientOptions::new();

    // Setup Stronghold secret_manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build(STRONGHOLD_SNAPSHOT_PATH)?;

    let mnemonic = Mnemonic::from(std::env::var("MNEMONIC").unwrap());

    // The mnemonic only needs to be stored the first time
    secret_manager.store_mnemonic(mnemonic).await?;

    // Create the wallet with the secret_manager and client options
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(secret_manager))
        .with_storage_path(OFFLINE_WALLET_DB_PATH)
        .with_client_options(offline_client)
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .with_bech32_hrp(SHIMMER_BECH32_HRP)
        .with_alias("Alice")
        .finish()
        .await?;

    println!("Generated a new account '{}'", wallet.alias().await);

    write_addresses_to_file(&wallet).await
}

async fn write_addresses_to_file(wallet: &Wallet) -> Result<()> {
    use tokio::io::AsyncWriteExt;

    let wallet_address = wallet.address().await;
    let json = serde_json::to_string_pretty(&wallet_address)?;
    let mut file = tokio::io::BufWriter::new(tokio::fs::File::create(ADDRESS_FILE_PATH).await?);
    println!("example.address.json:\n{json}");
    file.write_all(json.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}
