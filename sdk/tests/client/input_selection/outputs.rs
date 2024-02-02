// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashSet, str::FromStr};

use iota_sdk::{
    client::api::input_selection::{Burn, Error, InputSelection},
    types::block::{address::Address, output::AccountId, protocol::protocol_parameters},
};
use pretty_assertions::assert_eq;

use crate::client::{
    build_inputs, build_outputs, is_remainder_or_return, unsorted_eq,
    Build::{Account, Basic},
    ACCOUNT_ID_2, BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1, SLOT_INDEX,
};

#[test]
fn no_inputs() {
    let protocol_parameters = protocol_parameters();

    let inputs = Vec::new();
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        protocol_parameters,
    )
    .select();

    assert!(matches!(selected, Err(Error::NoAvailableInputsProvided)));
}

#[test]
fn no_outputs() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        )],
        None,
    );
    let outputs = Vec::new();

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        protocol_parameters,
    )
    .select();

    assert!(matches!(selected, Err(Error::InvalidOutputCount(0))));
}

#[test]
fn no_outputs_but_required_input() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = Vec::new();

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        protocol_parameters,
    )
    .with_required_inputs(HashSet::from([*inputs[0].output_id()]))
    .select()
    .unwrap();

    assert_eq!(selected.inputs, inputs);
    // Just a remainder
    assert_eq!(selected.outputs.len(), 1);
    assert!(is_remainder_or_return(
        &selected.outputs[0],
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None
    ));
}

#[test]
fn no_outputs_but_burn() {
    let protocol_parameters = protocol_parameters();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs(
        [Account(
            2_000_000,
            account_id_2,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = Vec::new();

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_account(account_id_2))
    .select()
    .unwrap();

    assert_eq!(selected.inputs, inputs);
    assert_eq!(selected.outputs.len(), 1);
    assert!(is_remainder_or_return(
        &selected.outputs[0],
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None
    ));
}

#[test]
fn no_address_provided() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(inputs, outputs, [], SLOT_INDEX, protocol_parameters).select();

    assert!(matches!(selected, Err(Error::NoAvailableInputsProvided)));
}

#[test]
fn no_matching_address_provided() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        )],
        None,
    );
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()],
        SLOT_INDEX,
        protocol_parameters,
    )
    .select();

    assert!(matches!(selected, Err(Error::NoAvailableInputsProvided)));
}

#[test]
fn two_addresses_one_missing() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientAmount {
            found: 1_000_000,
            required: 2_000_000,
        })
    ));
}

#[test]
fn two_addresses() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        ],
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}
