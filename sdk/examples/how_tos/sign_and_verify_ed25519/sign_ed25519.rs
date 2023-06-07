// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will sign with Ed25519.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example sign_ed25519 --release --features=stronghold`

use iota_sdk::{
    client::{
        constants::{HD_WALLET_TYPE, SHIMMER_COIN_TYPE},
        hex_public_key_to_bech32_address,
        secret::{stronghold::StrongholdSecretManager, SecretManage, SecretManager},
    },
    crypto::keys::slip10::Chain,
    wallet::Result,
};

const FOUNDRY_METADATA: &str = r#"{"standard":"IRC30","name":"NativeToken","description":"A native token","symbol":"NT","decimals":6,"logoUrl":"https://my.website/nativeToken.png"}"#;
const ACCOUNT_INDEX: u32 = 0;
const INTERNAL_ADDRESS: bool = false;
const ADDRESS_INDEX: u32 = 0;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Setup Stronghold secret_manager
    let stronghold = StrongholdSecretManager::builder()
        .password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build("sign_ed25519.stronghold")?;

    stronghold
        .store_mnemonic(std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())
        .await?;

    let bip32_chain = Chain::from_u32_hardened([
        HD_WALLET_TYPE,
        SHIMMER_COIN_TYPE,
        ACCOUNT_INDEX,
        INTERNAL_ADDRESS as u32,
        ADDRESS_INDEX,
    ]);

    let message = FOUNDRY_METADATA.as_bytes();
    let signature = SecretManager::Stronghold(stronghold)
        .sign_ed25519(message, &bip32_chain)
        .await?;
    println!(
        "Public key: {}\nSignature: {}",
        prefix_hex::encode(signature.public_key()),
        prefix_hex::encode(signature.signature()),
    );

    // Hash the public key to get the address
    let bech32_address = hex_public_key_to_bech32_address(&prefix_hex::encode(signature.public_key()), "rms")?;
    println!("Address: {bech32_address}");

    Ok(())
}
