// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::api::input_selection::{Error, InputSelection},
    types::block::{address::Address, protocol::iota_mainnet_protocol_parameters, slot::SlotIndex},
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
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        protocol_parameters,
    )
    .select();

    assert!(matches!(selected, Err(Error::NoAvailableInputsProvided)));
}

#[test]
fn timelock_equal_timestamp() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000,
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
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        200,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_one_timelock_expired() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
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
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_one_timelocked_one_missing() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
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
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn one_output_timelock_expired() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000,
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
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}
