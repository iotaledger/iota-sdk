// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashSet, str::FromStr};

use iota_sdk::{
    client::{
        api::transaction_builder::{Burn, TransactionBuilder, TransactionBuilderError},
        secret::types::InputSigningData,
    },
    types::block::{
        address::Address,
        output::{unlock_condition::AddressUnlockCondition, AccountId, BasicOutputBuilder},
        payload::signed_transaction::{TransactionCapabilities, TransactionCapabilityFlag},
        protocol::iota_mainnet_protocol_parameters,
        rand::output::{rand_output_id_with_slot_index, rand_output_metadata_with_id},
    },
};
use pretty_assertions::assert_eq;

use crate::client::{
    assert_remainder_or_return, build_inputs, build_outputs, unsorted_eq,
    Build::{Account, Basic},
    ACCOUNT_ID_1, ACCOUNT_ID_2, BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1, SLOT_COMMITMENT_ID, SLOT_INDEX,
};

#[test]
fn no_inputs() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = Vec::new();
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish();

    assert!(matches!(
        selected,
        Err(TransactionBuilderError::NoAvailableInputsProvided)
    ));
}

#[test]
fn no_outputs() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                native_token: None,
                sender: None,
                sdruc: None,
                timelock: None,
                expiration: None,
            },
            None,
        )],
        None,
    );
    let outputs = Vec::new();

    let selected = TransactionBuilder::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish();

    assert!(matches!(selected, Err(TransactionBuilderError::InvalidOutputCount(0))));
}

#[test]
fn no_outputs_but_required_input() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                native_token: None,
                sender: None,
                sdruc: None,
                timelock: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = Vec::new();

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_required_inputs(HashSet::from([*inputs[0].output_id()]))
    .finish()
    .unwrap();

    assert_eq!(selected.inputs_data, inputs);
    // Just a remainder
    assert_eq!(selected.transaction.outputs().len(), 1);
    assert_remainder_or_return(
        &selected.transaction.outputs()[0],
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
    );
}

#[test]
fn no_outputs_but_burn() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 2_000_000,
                account_id: account_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = Vec::new();

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_account(account_id_2))
    .finish()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyAccountOutputs])
    );
    assert_eq!(selected.inputs_data, inputs);
    assert_eq!(selected.transaction.outputs().len(), 1);
    assert_remainder_or_return(
        &selected.transaction.outputs()[0],
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
    );
}

#[test]
fn no_address_provided() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                native_token: None,
                sender: None,
                sdruc: None,
                timelock: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs,
        outputs,
        None,
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish();

    assert!(matches!(
        selected,
        Err(TransactionBuilderError::NoAvailableInputsProvided)
    ));
}

#[test]
fn no_matching_address_provided() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                native_token: None,
                sender: None,
                sdruc: None,
                timelock: None,
                expiration: None,
            },
            None,
        )],
        None,
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish();

    assert!(matches!(
        selected,
        Err(TransactionBuilderError::NoAvailableInputsProvided)
    ));
}

#[test]
fn two_addresses_one_missing() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 1_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 2_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish();

    assert!(matches!(
        selected,
        Err(TransactionBuilderError::InsufficientAmount {
            found: 1_000_000,
            required: 2_000_000,
        })
    ));
}

#[test]
fn two_addresses() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 1_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 2_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        ],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn consolidate_with_min_allotment() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = [
        BasicOutputBuilder::new_with_minimum_amount(protocol_parameters.storage_score_parameters())
            .with_mana(9860)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
        BasicOutputBuilder::new_with_minimum_amount(protocol_parameters.storage_score_parameters())
            .with_mana(9860)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
        BasicOutputBuilder::new_with_minimum_amount(protocol_parameters.storage_score_parameters())
            .with_mana(9860)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
        BasicOutputBuilder::new_with_minimum_amount(protocol_parameters.storage_score_parameters())
            .with_mana(9860)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
    ];
    let inputs = inputs
        .into_iter()
        .map(|input| InputSigningData {
            output: input,
            output_metadata: rand_output_metadata_with_id(rand_output_id_with_slot_index(SLOT_INDEX)),
            chain: None,
        })
        .collect::<Vec<_>>();

    let selected = TransactionBuilder::new(
        inputs.clone(),
        None,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_min_mana_allotment(account_id_1, 10)
    .with_required_inputs(inputs.iter().map(|i| *i.output_id()))
    .finish()
    .unwrap();

    assert_eq!(selected.transaction.outputs().len(), 1);
    assert_eq!(selected.transaction.allotments().len(), 1);
    assert_eq!(selected.transaction.allotments()[0].mana(), 39440);
    assert_eq!(selected.transaction.outputs().iter().map(|o| o.mana()).sum::<u64>(), 0);
}
