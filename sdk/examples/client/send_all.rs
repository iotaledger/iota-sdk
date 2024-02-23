// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will get the outputs of the first address of the seed and send everything.
//! Run the consolidation example first if there are more than 128 outputs.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example send_all
//! ```

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        api::{input_selection::InputSelection, GetAddressesOptions},
        constants::IOTA_COIN_TYPE,
        node_api::indexer::query_parameters::BasicOutputQueryParameters,
        secret::{types::InputSigningData, SecretManage, SecretManager, SignBlock},
        Client,
    },
    types::block::{
        output::{unlock_condition::AddressUnlockCondition, AccountId, BasicOutputBuilder, NativeTokensBuilder},
        payload::{Payload, SignedTransactionPayload},
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    // Configure your own mnemonic in ".env". Since the output amount cannot be zero, the mnemonic
    // `NON_SECURE_USE_DEVELOPMENT_MNEMONIC_1` must contain non-zero balance.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL", "MNEMONIC", "MNEMONIC_2", "EXPLORER_URL", "ISSUER_ID"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Create a node client.
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let secret_manager_1 = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;
    let secret_manager_2 = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC_2").unwrap())?;
    let issuer_id = std::env::var("ISSUER_ID").unwrap().parse::<AccountId>().unwrap();

    let address = secret_manager_1
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
        .await?[0]
        .clone();

    // Get output ids of outputs that can be controlled by this address without further unlock constraints
    let output_ids_response = client
        .basic_output_ids(BasicOutputQueryParameters::only_address_unlock_condition(address))
        .await?;

    // Get the outputs by their id
    let outputs_responses = client.get_outputs_with_metadata(&output_ids_response.items).await?;

    // Calculate the total amount and native tokens
    let mut total_amount = 0;
    let mut total_native_tokens = NativeTokensBuilder::new();

    let mut inputs = Vec::new();

    for res in outputs_responses {
        if let Some(native_token) = res.output.native_token() {
            total_native_tokens.add_native_token(native_token.clone())?;
        }
        total_amount += res.output.amount();
        inputs.push(InputSigningData {
            output: res.output,
            output_metadata: res.metadata,
            chain: None,
        });
    }

    let total_native_tokens = total_native_tokens.finish()?;

    println!("Total amount: {total_amount}");

    let address = secret_manager_2
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
        .await?[0]
        .clone();

    let mut basic_output_builder =
        BasicOutputBuilder::new_with_amount(total_amount).add_unlock_condition(AddressUnlockCondition::new(address));

    for native_token in total_native_tokens {
        basic_output_builder = basic_output_builder.with_native_token(native_token);
    }
    let new_output = basic_output_builder.finish_output()?;

    let protocol_parameters = client.get_protocol_parameters().await?;

    let prepared_transaction = InputSelection::new(
        inputs,
        [new_output],
        None,
        client.get_slot_index().await?,
        client.get_issuance().await?.latest_commitment.id(),
        protocol_parameters.clone(),
    )
    .select()?;
    let unlocks = secret_manager_1
        .transaction_unlocks(&prepared_transaction, &protocol_parameters)
        .await?;

    let block = client
        .build_basic_block(
            issuer_id,
            Payload::from(SignedTransactionPayload::new(
                prepared_transaction.transaction,
                unlocks,
            )?),
        )
        .await?
        .sign_ed25519(&secret_manager_1, Bip44::new(IOTA_COIN_TYPE))
        .await?;

    client.post_block(&block).await?;

    println!(
        "Block with all outputs sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id(&protocol_parameters)
    );

    Ok(())
}
