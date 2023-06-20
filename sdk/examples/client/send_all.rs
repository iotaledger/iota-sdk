// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will get the outputs of the first address of the seed and send everything.
//! Run the consolidation example first if there are more than 128 outputs.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example send_all
//! ```

use std::env;

use iota_sdk::{
    client::{
        api::GetAddressesOptions, node_api::indexer::query_parameters::QueryParameter, secret::SecretManager, Client,
        Result,
    },
    types::block::output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NativeTokensBuilder},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    // Configure your own mnemonic in ".env". Since the output amount cannot be zero, the mnemonic must contain non-zero
    // balance.
    dotenvy::dotenv().ok();

    // Create a client instance
    let client = Client::builder()
        .with_node(&env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let secret_manager_1 =
        SecretManager::try_from_mnemonic(env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;
    let secret_manager_2 =
        SecretManager::try_from_mnemonic(env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_2").unwrap())?;

    let token_supply = client.get_token_supply().await?;

    let address = secret_manager_1
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
        .await?[0];

    // Get output ids of outputs that can be controlled by this address without further unlock constraints
    let output_ids_response = client
        .basic_output_ids([
            QueryParameter::Address(address),
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

    let total_native_tokens = total_native_tokens.finish()?;

    println!("Total amount: {total_amount}");

    let address = secret_manager_2
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
        .await?[0];

    let mut basic_output_builder =
        BasicOutputBuilder::new_with_amount(total_amount).add_unlock_condition(AddressUnlockCondition::new(address));

    for native_token in total_native_tokens {
        basic_output_builder = basic_output_builder.add_native_token(native_token);
    }
    let new_output = basic_output_builder.finish_output(token_supply)?;

    let block = client
        .block()
        .with_secret_manager(&secret_manager_1)
        .with_outputs([new_output])?
        .finish()
        .await?;

    println!(
        "Block with all outputs sent: {}/block/{}",
        env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    let _ = client.retry_until_included(&block.id(), None, None).await?;

    Ok(())
}
