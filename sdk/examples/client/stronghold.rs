// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create an address with a stronghold secret manager.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example stronghold
//! ```

use iota_sdk::{
    client::{
        constants::{IOTA_COIN_TYPE, IOTA_TESTNET_BECH32_HRP},
        secret::{stronghold::StrongholdSecretManager, PublicKeyOptions, SecretManageExt},
        Result,
    },
    crypto::keys::bip39::Mnemonic,
    types::block::address::{Ed25519Address, ToBech32Ext},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    #[allow(clippy::single_element_loop)]
    for var in ["MNEMONIC"] {
        std::env::var(var).unwrap_or_else(|_| panic!(".env variable '{var}' is undefined, see .env.example"));
    }

    let secret_manager = StrongholdSecretManager::builder()
        .password("some_hopefully_secure_password".to_owned())
        .build("test.stronghold")?;

    let mnemonic = Mnemonic::from(std::env::var("MNEMONIC").unwrap());

    // The mnemonic only needs to be stored the first time
    secret_manager.store_mnemonic(mnemonic).await?;

    // Generate address
    let address = secret_manager
        .generate::<Ed25519Address>(&PublicKeyOptions::new(IOTA_COIN_TYPE))
        .await?
        .to_bech32(IOTA_TESTNET_BECH32_HRP);

    println!("First public address: {address}");

    Ok(())
}
