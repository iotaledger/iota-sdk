// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{
        api::input_selection::{Burn, InputSelection},
        secret::types::InputSigningData,
    },
    types::block::{
        address::Address,
        output::{
            unlock_condition::AddressUnlockCondition, BasicOutputBuilder, DelegationId, DelegationOutputBuilder,
            OutputId,
        },
        protocol::protocol_parameters,
        rand::{
            address::rand_account_address, output::rand_output_metadata_with_id,
            transaction::rand_transaction_id_with_slot_index,
        },
        slot::SlotIndex,
    },
};
use pretty_assertions::assert_eq;

use crate::client::{BECH32_ADDRESS_ED25519_0, SLOT_INDEX};

#[test]
fn remainder_needed_for_mana() {
    let protocol_parameters = protocol_parameters();

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

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        protocol_parameters.clone(),
    )
    .with_burn(Burn::from(delegation_id))
    .add_mana_rewards(delegation_output_id, mana_rewards)
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 2);
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert_eq!(
        mana_rewards
            + selected
                .inputs
                .iter()
                .map(|i| i
                    .output
                    .available_mana(&protocol_parameters, SlotIndex(0), SLOT_INDEX)
                    .unwrap())
                .sum::<u64>(),
        selected.outputs.iter().map(|o| o.mana()).sum::<u64>()
    );
}
