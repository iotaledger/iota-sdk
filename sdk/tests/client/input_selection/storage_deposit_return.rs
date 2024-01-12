// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::{
    client::api::input_selection::{Error, InputSelection},
    types::block::{address::Address, output::AccountId, protocol::protocol_parameters},
};
use pretty_assertions::assert_eq;

use crate::client::{
    build_inputs, build_outputs, is_remainder_or_return, unsorted_eq,
    Build::{Account, Basic},
    ACCOUNT_ID_1, BECH32_ADDRESS_ACCOUNT_1, BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1,
    BECH32_ADDRESS_ED25519_2, SLOT_INDEX,
};

#[test]
fn sdruc_output_not_provided_no_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
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

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX + 1,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None
            ));
        }
    });
}

#[test]
fn sdruc_output_provided_no_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
            None,
            None,
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([
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
    ]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX + 1,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn sdruc_output_provided_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
            None,
            None,
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
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
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX + 1,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None
            ));
        }
    });
}

#[test]
fn two_sdrucs_to_the_same_address_both_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                None,
                None,
                None,
            ),
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                None,
                None,
                None,
            ),
        ],
        None,
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
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX + 1,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None
            ));
        }
    });
}

#[test]
fn two_sdrucs_to_the_same_address_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                None,
                None,
                None,
            ),
        ],
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
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX + 1,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert!(selected.inputs.contains(&inputs[0]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None
            ));
        }
    });
}

#[test]
fn two_sdrucs_to_different_addresses_both_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                None,
                None,
                None,
            ),
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(), 1_000_000)),
                None,
                None,
                None,
            ),
        ],
        None,
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
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX + 1,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 3);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(selected.outputs.iter().any(|output| {
        is_remainder_or_return(
            output,
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
            None,
        )
    }));
    assert!(selected.outputs.iter().any(|output| {
        is_remainder_or_return(
            output,
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
            None,
        )
    }));
}

#[test]
fn two_sdrucs_to_different_addresses_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(), 1_000_000)),
                None,
                None,
                None,
            ),
        ],
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
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX + 1,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert!(selected.inputs.contains(&inputs[0]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None
            ));
        }
    });
}

#[test]
fn insufficient_amount_because_of_sdruc() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
            None,
            None,
            None,
        )],
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
        SLOT_INDEX + 1,
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientAmount {
            found: 2_000_000,
            required: 3_000_000,
        })
    ));
}

#[test]
fn useless_sdruc_required_for_sender_feature() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(), 1_000_000)),
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
        None,
    );
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()),
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
        SLOT_INDEX + 1,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
                None
            ));
        }
    });
}

#[test]
fn sdruc_required_non_ed25519_in_address_unlock() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap(),
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(), 1_000_000)),
                None,
                None,
                None,
            ),
            Account(
                1_000_000,
                account_id_1,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
            ),
        ],
        None,
    );
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
        None,
        Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX + 1,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 3);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) && !output.is_account() {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
                None
            ));
        }
    });
}

#[test]
fn useless_sdruc_non_ed25519_in_address_unlock() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap(),
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(), 1_000_000)),
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            Account(
                1_000_000,
                account_id_1,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
            ),
        ],
        None,
    );
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
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
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX + 1,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 2);
    assert!(selected.inputs.contains(&inputs[1]));
    assert!(selected.inputs.contains(&inputs[2]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_account());
        }
    });
}
