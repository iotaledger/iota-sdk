// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we generate addresses which will be used later to find inputs.
//!
//! `cargo run --example 0_generate_addresses --release`

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use iota_sdk::{
    client::{
        constants::{SHIMMER_BECH32_HRP, SHIMMER_COIN_TYPE},
        secret::{stronghold::StrongholdSecretManager, SecretManager},
    },
    wallet::{account::types::AccountAddress, ClientOptions, Result, Wallet},
};

const ADDRESS_FILE_NAME: &str = "examples/wallet/offline_signing/addresses.json";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let offline_client = ClientOptions::new();

    // Setup Stronghold secret_manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build("examples/wallet/offline_signing/offline_signing.stronghold")?;
    // Only required the first time, can also be generated with `manager.generate_mnemonic()?`
    let mnemonic = std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap();

    // The mnemonic only needs to be stored the first time
    secret_manager.store_mnemonic(mnemonic).await?;

    // Create the wallet with the secret_manager and client options
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(secret_manager))
        .with_client_options(offline_client)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path("examples/wallet/offline_signing/offline_walletdb")
        .finish()
        .await?;

    // Create a new account
    let account = wallet
        .create_account()
        .with_alias("Alice".to_string())
        .with_bech32_hrp(SHIMMER_BECH32_HRP)
        .finish()
        .await?;

    println!("Generated a new account");

    let addresses = account.addresses().await?;

    write_addresses_to_file(ADDRESS_FILE_NAME, addresses)
}

fn write_addresses_to_file<P: AsRef<Path>>(path: P, addresses: Vec<AccountAddress>) -> Result<()> {
    let json = serde_json::to_string_pretty(&addresses)?;
    let mut file = BufWriter::new(File::create(path)?);

    println!("{json}");

    file.write_all(json.as_bytes())?;

    Ok(())
}
