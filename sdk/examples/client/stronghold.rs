// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create an address with a stronghold secret manager.
//!
//! `cargo run --example stronghold --features=stronghold --release`

use iota_sdk::client::{
    api::GetAddressesOptions,
    constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
    secret::{stronghold::StrongholdSecretManager, SecretManager},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let stronghold_secret_manager = StrongholdSecretManager::builder()
        .password("some_hopefully_secure_password")
        .build("test.stronghold")?;

    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();
    let mnemonic = std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap();
    // The mnemonic only needs to be stored the first time
    stronghold_secret_manager.store_mnemonic(mnemonic).await?;

    // Generate addresses with custom account index and range
    let addresses = SecretManager::Stronghold(stronghold_secret_manager)
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_account_index(0)
                .with_range(0..1),
        )
        .await?;

    println!("First public address: {}", addresses[0]);

    Ok(())
}
