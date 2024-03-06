// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will get the outputs of the first address of the seed and send everything.
//! Run the consolidation example first if there are more than 128 outputs.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example send_all
//! ```

use std::collections::BTreeSet;

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        api::{options::TransactionOptions, GetAddressesOptions},
        constants::IOTA_COIN_TYPE,
        node_api::indexer::query_parameters::BasicOutputQueryParameters,
        secret::{SecretManage, SecretManager, SignBlock},
        Client,
    },
    types::block::{
        address::{Address, Ed25519Address, ToBech32Ext},
        output::{unlock_condition::AddressUnlockCondition, AccountId, BasicOutputBuilder},
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

    let chain = Bip44::new(IOTA_COIN_TYPE);

    let from_address = Address::from(Ed25519Address::from_public_key_bytes(
        secret_manager_1
            .generate_ed25519_public_keys(
                chain.coin_type,
                chain.account,
                chain.address_index..chain.address_index + 1,
                None,
            )
            .await?[0]
            .to_bytes(),
    ))
    .to_bech32(client.get_bech32_hrp().await?);

    // Get output ids of outputs that can be controlled by this address without further unlock constraints
    let output_ids_response = client
        .basic_output_ids(BasicOutputQueryParameters::only_address_unlock_condition(
            from_address.clone(),
        ))
        .await?;

    // Get the outputs by their id
    let outputs_responses = client.get_outputs_with_metadata(&output_ids_response.items).await?;

    let protocol_parameters = client.get_protocol_parameters().await?;

    // Calculate the total amount
    let mut total_amount = 0;

    let mut inputs = BTreeSet::new();
    let mut outputs = Vec::new();

    for res in outputs_responses {
        total_amount += res.output.amount();
        if let Some(native_token) = res.output.native_token() {
            // We don't want to send the native tokens, so return them and subtract out the storage amount.
            let native_token_return =
                BasicOutputBuilder::new_with_minimum_amount(protocol_parameters.storage_score_parameters())
                    .add_unlock_condition(AddressUnlockCondition::new(from_address.clone()))
                    .with_native_token(native_token.clone())
                    .finish_output()?;
            total_amount -= native_token_return.amount();
            outputs.push(native_token_return);
        }
        inputs.insert(*res.metadata().output_id());
    }

    println!("Total amount: {total_amount}");

    let to_address = secret_manager_2
        .generate_ed25519_addresses_as_bech32(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
        .await?[0]
        .clone();

    // Add the output with the total amount we're sending
    outputs.push(
        BasicOutputBuilder::new_with_amount(total_amount)
            .add_unlock_condition(AddressUnlockCondition::new(to_address.clone()))
            .finish_output()?,
    );

    let prepared_transaction = client
        .build_transaction(
            [(from_address.into_inner(), chain)],
            outputs,
            TransactionOptions {
                required_inputs: inputs,
                allow_additional_input_selection: false,
                ..Default::default()
            },
        )
        .await?;
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
