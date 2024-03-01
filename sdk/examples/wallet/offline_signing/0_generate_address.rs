// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we generate addresses which will be used later to find inputs.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example 0_generate_address
//! ```

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
    },
    crypto::keys::{bip39::Mnemonic, bip44::Bip44},
    types::block::address::Bech32Address,
    wallet::{ClientOptions, Wallet},
};

const OFFLINE_WALLET_DB_PATH: &str = "./examples/wallet/offline_signing/example-offline-walletdb";
const STRONGHOLD_SNAPSHOT_PATH: &str = "./examples/wallet/offline_signing/example.stronghold";
const ADDRESS_FILE_PATH: &str = "./examples/wallet/offline_signing/example.address.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["STRONGHOLD_PASSWORD", "MNEMONIC"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

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
        .finish()
        .await?;
    println!("Generated a new wallet");

    write_wallet_address_to_file(&wallet.address().await).await?;

    Ok(())
}

async fn write_wallet_address_to_file(address: &Bech32Address) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::io::AsyncWriteExt;

    let json = serde_json::to_string_pretty(address)?;
    let mut file = tokio::io::BufWriter::new(tokio::fs::File::create(ADDRESS_FILE_PATH).await?);
    println!("example.address.json:\n{json}");
    file.write_all(json.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}
