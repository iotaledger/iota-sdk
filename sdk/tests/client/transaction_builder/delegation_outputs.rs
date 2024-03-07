// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{
        api::transaction_builder::{Burn, TransactionBuilder},
        secret::types::InputSigningData,
    },
    types::block::{
        address::Address,
        output::{
            unlock_condition::AddressUnlockCondition, BasicOutputBuilder, DelegationId, DelegationOutputBuilder,
            OutputId,
        },
        protocol::iota_mainnet_protocol_parameters,
        rand::{
            address::rand_account_address, output::rand_output_metadata_with_id,
            transaction::rand_transaction_id_with_slot_index,
        },
    },
};
use pretty_assertions::assert_eq;

use crate::client::{BECH32_ADDRESS_ED25519_0, SLOT_COMMITMENT_ID, SLOT_INDEX};

#[test]
fn remainder_needed_for_mana() {
    let protocol_parameters = iota_mainnet_protocol_parameters();

    let delegation_input =
        DelegationOutputBuilder::new_with_amount(1_000_000, DelegationId::null(), rand_account_address())
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap();

    let inputs = [
        delegation_input,
        BasicOutputBuilder::new_with_amount(1_000_000)
            .with_mana(100)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
    ];

    let inputs = inputs
        .into_iter()
        .map(|output| {
            let transaction_id = rand_transaction_id_with_slot_index(SLOT_INDEX);

            InputSigningData {
                output,
                output_metadata: rand_output_metadata_with_id(OutputId::new(transaction_id, 0)),
                chain: None,
            }
        })
        .collect::<Vec<InputSigningData>>();
    let delegation_output_id = *inputs[0].output_id();
    let delegation_id = DelegationId::from(&delegation_output_id);

    let outputs = vec![
        BasicOutputBuilder::new_with_amount(1_000_000)
            .with_mana(200)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
    ];

    let mana_rewards = 100;

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters.clone(),
    )
    .with_burn(Burn::from(delegation_id))
    .add_mana_rewards(delegation_output_id, mana_rewards)
    .finish()
    .unwrap();

    let inputs = selected
        .inputs_data
        .iter()
        .map(|input| (input.output_id(), &input.output))
        .collect::<Vec<_>>();

    // validating without rewards
    iota_sdk::types::block::semantic::SemanticValidationContext::new(
        &selected.transaction,
        &inputs,
        None,
        None,
        protocol_parameters,
    )
    .validate()
    .unwrap();

    assert_eq!(selected.inputs_data.len(), 2);
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    assert_eq!(
        mana_rewards
            + selected
                .inputs_data
                .iter()
                .map(|i| i
                    .output
                    .available_mana(
                        protocol_parameters,
                        i.output_id().transaction_id().slot_index(),
                        SLOT_INDEX
                    )
                    .unwrap())
                .sum::<u64>(),
        selected.transaction.outputs().iter().map(|o| o.mana()).sum::<u64>()
    );
}
