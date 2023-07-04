// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we generate an address which will be used later to find inputs.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example 0_address_generation
//! ```

use iota_sdk::{
    client::{api::GetAddressesOptions, constants::SHIMMER_TESTNET_BECH32_HRP, secret::SecretManager, Result},
    types::block::address::Bech32Address,
};

const ADDRESS_FILE_NAME: &str = "examples/client/offline_signing/address.json";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let secret_manager =
        SecretManager::try_from_mnemonic(std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    // Generates an address offline.
    let address = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
                // Currently only index 0 is supported for offline signing.
                .with_range(0..1),
        )
        .await?;

    write_address_to_file(ADDRESS_FILE_NAME, &address).await?;

    Ok(())
}

async fn write_address_to_file(path: impl AsRef<std::path::Path>, address: &[Bech32Address]) -> Result<()> {
    use tokio::io::AsyncWriteExt;

    let json = serde_json::to_string_pretty(&address)?;
    let mut file = tokio::io::BufWriter::new(tokio::fs::File::create(path).await.expect("failed to create file"));

    println!("{json}");

    file.write_all(json.as_bytes()).await.expect("failed to write to file");

    Ok(())
}
