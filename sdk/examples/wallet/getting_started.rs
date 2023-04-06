// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
    },
    wallet::{ClientOptions, Result, Wallet},
};

const NODE_URL: &str = "https://api.testnet.shimmer.network";
const STRONGHOLD_SNAPSHOT_PATH: &str = "vault.stronghold";

#[tokio::main]
async fn main() -> Result<()> {
    // Change to a secure password.
    let password = "some-secure-password";

    // Setup Stronghold secret manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(password)
        .build(PathBuf::from(STRONGHOLD_SNAPSHOT_PATH))?;

    let client_options = ClientOptions::new().with_node(NODE_URL)?;

    // Set up and store the wallet.
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Generate a mnemonic and store it in the Stronghold vault.
    let mnemonic = wallet.generate_mnemonic()?;
    wallet.store_mnemonic(mnemonic.clone()).await?;

    // Create an account and get the first address.
    let account = wallet.create_account().with_alias("Alice".to_string()).finish().await?;
    let address = &account.addresses().await?[0];

    // Print the account data.
    println!("Mnemonic:\n{}\n", mnemonic);
    println!("Address:\n{}\n", address.address().to_bech32());

    Ok(())
}
