// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to get the outputs of an address that have no additional unlock conditions, and sum the
//! amounts and native tokens.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example 02_address_balance
//! ```

use iota_sdk::{
    client::{
        constants::IOTA_COIN_TYPE,
        node_api::indexer::query_parameters::BasicOutputQueryParameters,
        secret::{mnemonic::MnemonicSecretManager, PublicKeyOptions, SecretManageExt},
        Client, Result,
    },
    types::block::{
        address::{Ed25519Address, ToBech32Ext},
        output::NativeTokensBuilder,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL", "MNEMONIC"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Create a node client.
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let secret_manager = MnemonicSecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    let hrp = client.get_bech32_hrp().await?;

    // Generate the first address
    let first_address = secret_manager
        .generate::<Ed25519Address>(&PublicKeyOptions::new(IOTA_COIN_TYPE))
        .await?
        .to_bech32(hrp);

    // Get output ids of outputs that can be controlled by this address without further unlock constraints
    let output_ids_response = client
        .basic_output_ids(BasicOutputQueryParameters::only_address_unlock_condition(
            first_address.clone(),
        ))
        .await?;

    // Get the outputs by their id
    let outputs = client.get_outputs(&output_ids_response.items).await?;

    // Calculate the total amount and native tokens
    let mut total_amount = 0;
    let mut total_native_tokens = NativeTokensBuilder::new();
    for output in outputs {
        if let Some(native_token) = output.output.native_token() {
            total_native_tokens.add_native_token(*native_token)?;
        }
        total_amount += output.output.amount();
    }

    println!(
        "Outputs controlled by {} have: {:?}i and native tokens:\n{:#?}",
        first_address,
        total_amount,
        total_native_tokens.finish_vec()?
    );
    Ok(())
}
