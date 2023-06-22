// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will write out all wallet logging events to a dedicated log file.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example logger
//! ```

use std::env::var;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    wallet::{ClientOptions, Result, Wallet},
};

// The number of addresses to generate
const NUM_ADDRESSES_TO_GENERATE: u32 = 5;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

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
    let client_options = ClientOptions::new().with_node(&var("NODE_URL").unwrap())?;
    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Get or create a new account
    let alias = "Alice";
    let account = if let Ok(account) = wallet.get_account(alias).await {
        account
    } else {
        println!("Creating account '{alias}'");
        wallet.create_account().with_alias(alias).finish().await?
    };

    println!("Generating {NUM_ADDRESSES_TO_GENERATE} addresses...");
    let _ = account
        .generate_ed25519_addresses(NUM_ADDRESSES_TO_GENERATE, None)
        .await?;

    println!("Syncing account");
    account.sync(None).await?;

    println!("Example finished successfully");
    Ok(())
}
