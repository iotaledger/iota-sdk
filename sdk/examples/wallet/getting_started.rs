// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will first create a new wallet and a mnemonic, and then, print the wallet's address.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example wallet_getting_started
//! ```

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
    },
    crypto::keys::bip44::Bip44,
    wallet::{ClientOptions, Wallet},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Setup Stronghold secret manager.
    // WARNING: Never hardcode passwords in production code.
    let secret_manager = StrongholdSecretManager::builder()
        .password("password".to_owned()) // A password to encrypt the stored data.
        .build("vault.stronghold")?; // The path to store the wallet snapshot.

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    // Set up and store the wallet.
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(secret_manager))
        .with_client_options(client_options)
        .with_storage_path("getting-started-db")
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .with_alias("Alice".to_string())
        .finish()
        .await?;

    // Generate a mnemonic and store its seed in the Stronghold vault.
    // INFO: It is best practice to back up the mnemonic somewhere secure.
    let mnemonic = wallet.generate_mnemonic()?;
    println!("Mnemonic: {}", mnemonic.as_ref());
    wallet.store_mnemonic(mnemonic).await?;

    let wallet_address = wallet.address().await;
    println!("{}", wallet_address);

    Ok(())
}
