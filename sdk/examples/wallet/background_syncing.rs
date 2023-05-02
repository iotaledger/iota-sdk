// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: Example description
//!
//! `cargo run --example background_syncing --release`

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    wallet::{ClientOptions, Result, Wallet},
};
use tokio::time::{sleep, Duration};

const ACCOUNT: &str = "logger";
const NUM_ADDRESSES_TO_GENERATE: u32 = 1;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Get account or create a new one
    let account_alias = ACCOUNT;
    let account = match wallet.get_account(account_alias).await {
        Ok(account) => account,
        _ => {
            // first we'll create an example account and store it
            wallet
                .create_account()
                .with_alias(account_alias.to_string())
                .finish()
                .await?
        }
    };

    account.generate_addresses(NUM_ADDRESSES_TO_GENERATE, None).await?;

    wallet.start_background_syncing(None, None).await?;
    sleep(Duration::from_secs(10)).await;
    wallet.stop_background_syncing().await?;
    wallet.start_background_syncing(None, None).await?;

    Ok(())
}
