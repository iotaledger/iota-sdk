// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::api::transaction_builder::{TransactionBuilder, TransactionBuilderError},
    types::block::{
        address::Address,
        protocol::iota_mainnet_protocol_parameters,
        slot::{SlotCommitmentHash, SlotIndex},
    },
};
use pretty_assertions::assert_eq;

use crate::client::{
    build_inputs, build_outputs, unsorted_eq, Build::Basic, BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1,
};

#[test]
fn one_output_timelock_not_expired() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000,
                mana: 0,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                native_token: None,
                sender: None,
                sdruc: None,
                timelock: Some(200),
                expiration: None,
            },
            None,
        )],
        None,
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
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
        100,
        SlotCommitmentHash::null().into_slot_commitment_id(99),
        protocol_parameters,
    )
    .finish();

    assert!(matches!(
        selected,
        Err(TransactionBuilderError::NoAvailableInputsProvided)
    ));
}

#[test]
fn timelock_equal_timestamp() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000,
                mana: 0,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                native_token: None,
                sender: None,
                sdruc: None,
                timelock: Some(200),
                expiration: None,
            },
            None,
        )],
        Some(SlotIndex::from(200)),
    );
    let outputs = build_outputs([Basic {
        amount: 2_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        200,
        SlotCommitmentHash::null().into_slot_commitment_id(199),
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

#[test]
fn two_outputs_one_timelock_expired() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: Some(200),
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: Some(50),
                    expiration: None,
                },
                None,
            ),
        ],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Basic {
        amount: 2_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        SlotCommitmentHash::null().into_slot_commitment_id(99),
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert_eq!(selected.inputs_data.len(), 1);
    assert_eq!(selected.inputs_data[0], inputs[1]);
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

#[test]
fn two_outputs_one_timelocked_one_missing() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: Some(200),
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
        ],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Basic {
        amount: 2_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        SlotCommitmentHash::null().into_slot_commitment_id(99),
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert_eq!(selected.inputs_data.len(), 1);
    assert_eq!(selected.inputs_data[0], inputs[1]);
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

#[test]
fn one_output_timelock_expired() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000,
                mana: 0,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                native_token: None,
                sender: None,
                sdruc: None,
                timelock: Some(50),
                expiration: None,
            },
            None,
        )],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Basic {
        amount: 2_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        SlotCommitmentHash::null().into_slot_commitment_id(99),
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}
