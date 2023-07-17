// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.

use iota_sdk::{
    client::{api::GetAddressesOptions, node_api::indexer::query_parameters::QueryParameter, Result},
    types::block::{
        address::ToBech32Ext,
        input::{Input, UtxoInput},
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, OutputId},
        payload::{transaction::TransactionEssence, Payload},
    },
};

use crate::client::common::create_client_and_secret_manager_with_funds;

#[ignore]
#[tokio::test]
async fn send_basic_output() -> Result<()> {
    let (client, secret_manager) = create_client_and_secret_manager_with_funds(None).await?;

    let token_supply = client.get_token_supply().await?;

    let second_address = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(1..2))
        .await?[0];

    let output = BasicOutputBuilder::new_with_amount(1_000_000)
        .add_unlock_condition(AddressUnlockCondition::new(second_address))
        .finish_output(token_supply)?;

    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs([output.clone()])?
        .finish()
        .await?;

    let output_id = if let Payload::Transaction(tx_payload) = block.payload().unwrap() {
        let TransactionEssence::Regular(essence) = tx_payload.essence();
        // only one input from the faucet
        assert_eq!(essence.inputs().len(), 1);
        // provided output + remainder output
        assert_eq!(essence.outputs().len(), 2);
        // first output == provided output
        assert_eq!(essence.outputs()[0], output);

        OutputId::new(tx_payload.id(), 0)?
    } else {
        panic!("missing transaction payload")
    };

    client.retry_until_included(&block.id(), None, None).await?;

    let bech32_hrp = client.get_bech32_hrp().await?;

    // output can be fetched from the second address
    let output_ids_response = client
        .basic_output_ids([
            QueryParameter::Address(second_address.to_bech32(bech32_hrp)),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;

    assert_eq!(output_ids_response.items, [output_id]);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn custom_input() -> Result<()> {
    let (client, secret_manager) = create_client_and_secret_manager_with_funds(None).await?;

    let token_supply = client.get_token_supply().await?;

    let address = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
        .await?[0];

    let bech32_hrp = client.get_bech32_hrp().await?;

    // First create multiple outputs on our address, so there is even the possibility to select another input
    let output = BasicOutputBuilder::new_with_amount(1_000_000)
        .add_unlock_condition(AddressUnlockCondition::new(address))
        .finish_output(token_supply)?;
    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs(vec![output; 10])?
        .finish()
        .await?;
    client.retry_until_included(&block.id(), None, None).await?;

    let output_ids_response = client
        .basic_output_ids([
            QueryParameter::Address(address.to_bech32(bech32_hrp)),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;

    // The first output would be picked when using automatic input selection, so use the second one instead
    let input_id = output_ids_response.items[1];

    let input_amount = client.get_output(&input_id).await?.output().amount();

    let output = BasicOutputBuilder::new_with_amount(input_amount)
        .add_unlock_condition(AddressUnlockCondition::new(address))
        .finish_output(token_supply)?;

    let utxo_input = UtxoInput::from(input_id);
    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs([output.clone()])?
        .with_input(utxo_input)?
        .finish()
        .await?;

    client.retry_until_included(&block.id(), None, None).await?;

    if let Payload::Transaction(tx_payload) = block.payload().unwrap() {
        let TransactionEssence::Regular(essence) = tx_payload.essence();
        // only the provided input is used
        assert_eq!(essence.inputs().len(), 1);
        assert_eq!(essence.inputs()[0], Input::Utxo(utxo_input));
    } else {
        panic!("missing transaction payload")
    };

    Ok(())
}
