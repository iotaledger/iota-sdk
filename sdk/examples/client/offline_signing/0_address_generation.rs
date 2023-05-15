// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we generate an address which will be used later to find inputs.
//!
//! `cargo run --example 0_address_generation --release`

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use iota_sdk::client::{
    api::GetAddressesOptions, constants::SHIMMER_TESTNET_BECH32_HRP, secret::SecretManager, Result,
};

const ADDRESS_FILE_NAME: &str = "examples/client/offline_signing/address.json";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let secret_manager =
        SecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    // Generates an address offline.
    let address = secret_manager
        .get_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
                .with_range(0..1),
        )
        .await?;

    write_address_to_file(ADDRESS_FILE_NAME, &address)
}

fn write_address_to_file<P: AsRef<Path>>(path: P, address: &[String]) -> Result<()> {
    let json = serde_json::to_string_pretty(&address)?;
    let mut file = BufWriter::new(File::create(path).unwrap());

    println!("{json}");

    file.write_all(json.as_bytes()).unwrap();

    Ok(())
}
