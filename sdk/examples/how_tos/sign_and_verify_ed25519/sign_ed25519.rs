// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will sign with Ed25519.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example sign_ed25519
//! ```

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        hex_public_key_to_bech32_address,
        secret::{stronghold::StrongholdSecretManager, SecretManage, SecretManager},
    },
    crypto::keys::bip39::Mnemonic,
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
        .store_mnemonic(Mnemonic::from(std::env::var("MNEMONIC").unwrap()))
        .await?;

    let bip44_chain = Bip44::new()
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_account(ACCOUNT_INDEX)
        .with_change(INTERNAL_ADDRESS as _)
        .with_address_index(ADDRESS_INDEX);

    let message = FOUNDRY_METADATA.as_bytes();
    let signature = SecretManager::Stronghold(stronghold)
        .sign_ed25519(message, bip44_chain)
        .await?;
    println!(
        "Public key: {}\nSignature: {}",
        prefix_hex::encode(signature.public_key().as_ref()),
        prefix_hex::encode(signature.signature().to_bytes()),
    );

    // Hash the public key to get the address
    let bech32_address = hex_public_key_to_bech32_address(&prefix_hex::encode(signature.public_key().as_ref()), "rms")?;
    println!("Address: {bech32_address}");

    Ok(())
}
