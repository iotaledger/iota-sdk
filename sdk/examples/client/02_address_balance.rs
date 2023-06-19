// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to get the outputs of an address that have no additional unlock conditions, and sum the
//! amounts and native tokens.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example 02_address_balance
//! ```

use std::env;

use iota_sdk::{
    client::{
        api::GetAddressesOptions, node_api::indexer::query_parameters::QueryParameter, secret::SecretManager, Client,
        Result,
    },
    types::block::output::NativeTokensBuilder,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create a client instance.
    let client = Client::builder()
        .with_node(&env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let secret_manager =
        SecretManager::try_from_mnemonic(env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    // Generate the first address
    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::from_client(&client)
                .await?
                .with_account_index(0)
                .with_range(0..1),
        )
        .await?;

    // Get output ids of outputs that can be controlled by this address without further unlock constraints
    let output_ids_response = client
        .basic_output_ids([
            QueryParameter::Address(addresses[0]),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;

    // Get the outputs by their id
    let outputs_responses = client.get_outputs(&output_ids_response.items).await?;

    // Calculate the total amount and native tokens
    let mut total_amount = 0;
    let mut total_native_tokens = NativeTokensBuilder::new();
    for output_response in outputs_responses {
        let output = output_response.output();
        if let Some(native_tokens) = output.native_tokens() {
            total_native_tokens.add_native_tokens(native_tokens.clone())?;
        }
        total_amount += output.amount();
    }

    println!(
        "Outputs controlled by {} have: {:?}i and native tokens:\n{:#?}",
        addresses[0],
        total_amount,
        total_native_tokens.finish_vec()?
    );
    Ok(())
}
