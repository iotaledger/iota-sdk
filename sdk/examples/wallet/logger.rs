// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example, we will write out all wallet logging events to a dedicated log file.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --all-features --example logger --release
//! ```

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    wallet::{ClientOptions, Result, Wallet},
};

// The account alias created in this example
const ACCOUNT_ALIAS: &str = "Alice";
// The wallet database folder created in this example
const WALLET_DB_PATH: &str = "./example.walletdb";
// The log file name
const LOG_FILE_NAME: &str = "example.log";
// The log level to use (error, warn, info, debug, trace)
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;
// The number of addresses to generate
const NUM_ADDRESSES_TO_GENERATE: u32 = 5;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Initialize a logger that writes to the specified file
    let logger_output_config = fern_logger::LoggerOutputConfigBuilder::new()
        .name(LOG_FILE_NAME)
        .target_exclusions(&["h2", "hyper", "rustls"])
        .level_filter(LOG_LEVEL);
    let config = fern_logger::LoggerConfig::build()
        .with_output(logger_output_config)
        .finish();
    fern_logger::logger_init(config).unwrap();

    // Restore a wallet
    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;
    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path(WALLET_DB_PATH)
        .finish()
        .await?;

    // Get or create a new account
    let account = if let Ok(account) = wallet.get_account(ACCOUNT_ALIAS).await {
        account
    } else {
        println!("Creating account '{ACCOUNT_ALIAS}'");
        wallet
            .create_account()
            .with_alias(ACCOUNT_ALIAS.to_string())
            .finish()
            .await?
    };

    print!("Generating {NUM_ADDRESSES_TO_GENERATE} addresses...");
    let _ = account.generate_addresses(NUM_ADDRESSES_TO_GENERATE, None).await?;
    println!("done");

    print!("Syncing account...");
    let _ = account.sync(None).await?;
    println!("done");

    Ok(())
}
