// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will write out all wallet logging events to a dedicated log file.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example logger
//! ```

use iota_sdk::{
    client::{
        constants::IOTA_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, PublicKeyOptions, SecretManageExt},
    },
    crypto::keys::bip44::Bip44,
    types::block::address::Ed25519Address,
    wallet::{ClientOptions, Result, Wallet, WalletBuilder},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL", "MNEMONIC", "WALLET_DB_PATH"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Initialize a logger that writes to the specified file
    let logger_output_config = fern_logger::LoggerOutputConfigBuilder::new()
        .name("example.log")
        .target_exclusions(&["h2", "hyper", "rustls"])
        .level_filter(log::LevelFilter::Debug);
    let config = fern_logger::LoggerConfig::build()
        .with_output(logger_output_config)
        .finish();
    fern_logger::logger_init(config).unwrap();

    // Restore a wallet
    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;
    let secret_manager = std::sync::Arc::new(MnemonicSecretManager::try_from_mnemonic(
        std::env::var("MNEMONIC").unwrap(),
    )?);
    let wallet = WalletBuilder::new()
        .with_secret_manager(secret_manager.clone())
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_public_key_options(PublicKeyOptions::new(IOTA_COIN_TYPE))
        .with_signing_options(Bip44::new(IOTA_COIN_TYPE))
        .finish()
        .await?;

    println!("Generating address...");
    secret_manager
        .generate::<Ed25519Address>(&PublicKeyOptions::new(IOTA_COIN_TYPE))
        .await?;

    println!("Syncing wallet");
    wallet.sync(None).await?;

    println!("Example finished successfully");
    Ok(())
}
