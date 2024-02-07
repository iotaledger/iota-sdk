// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::{
    client::api::input_selection::{Burn, Error, InputSelection},
    types::block::{address::Address, output::TokenId, protocol::protocol_parameters},
};
use pretty_assertions::assert_eq;
use primitive_types::U256;

use crate::client::{
    build_inputs, build_outputs, is_remainder_or_return, unsorted_eq, Build::Basic, BECH32_ADDRESS_ED25519_0,
    SLOT_INDEX, TOKEN_ID_1, TOKEN_ID_2,
};

#[test]
fn two_native_tokens_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 150)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_2, 100)),
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
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 150)),
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs[0], inputs[0]);
    assert_eq!(selected.outputs.len(), 1);
    assert!(selected.outputs.contains(&outputs[0]));
}

#[test]
fn two_native_tokens_both_needed_plus_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_2, 150)),
                None,
                None,
                None,
                None,
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([
        Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Some((TOKEN_ID_1, 100)),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Some((TOKEN_ID_2, 100)),
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 3);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_2, 50))
            ));
        }
    });
}

#[test]
fn three_inputs_two_needed_plus_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
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
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 120)),
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 2);
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 80))
            ));
        }
    });
}

#[test]
fn three_inputs_two_needed_no_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
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
        Some((TOKEN_ID_1, 200)),
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 2);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn insufficient_native_tokens_one_input() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Some((TOKEN_ID_1, 100)),
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
        Some((TOKEN_ID_1, 150)),
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
        Err(Error::InsufficientNativeTokenAmount {
            token_id,
            found,
            required,
        }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(100) && required == U256::from(150)));
}

#[test]
fn insufficient_native_tokens_three_inputs() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
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
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 301)),
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
        Err(Error::InsufficientNativeTokenAmount {
            token_id,
            found,
            required,
        }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(300) && required == U256::from(301)));
}

#[test]
fn burn_and_send_at_the_same_time() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_2, 100)),
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
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 50)),
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .with_burn(
        Burn::new()
            .add_native_token(TokenId::from_str(TOKEN_ID_1).unwrap(), 10)
            .add_native_token(TokenId::from_str(TOKEN_ID_2).unwrap(), 100),
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
                Some((TOKEN_ID_1, 40))
            ));
        }
    });
}

#[test]
fn burn_one_input_no_output() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Some((TOKEN_ID_1, 100)),
            None,
            None,
            None,
            None,
            None,
        )],
        Some(SLOT_INDEX),
    );

    let selected = InputSelection::new(
        inputs.clone(),
        Vec::new(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_native_token(TokenId::from_str(TOKEN_ID_1).unwrap(), 50))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 1);
    assert!(is_remainder_or_return(
        &selected.outputs[0],
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 50))
    ));
}

#[test]
fn multiple_native_tokens() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_2, 100)),
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
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 100)),
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert!(selected.inputs.contains(&inputs[0]));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn insufficient_native_tokens() {
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
        Some((TOKEN_ID_1, 150)),
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
        Err(Error::InsufficientNativeTokenAmount {
            token_id,
            found,
            required,
        }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(0) && required == U256::from(150)));
}

#[test]
fn insufficient_native_tokens_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Some((TOKEN_ID_1, 100)),
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
        Some((TOKEN_ID_1, 150)),
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
        Err(Error::InsufficientNativeTokenAmount {
            token_id,
            found,
            required,
        }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(100) && required == U256::from(150)));
}

#[test]
fn insufficient_amount_for_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Some((TOKEN_ID_1, 100)),
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
        Some((TOKEN_ID_1, 50)),
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
            required: 1_106_000,
        })
    ));
}

#[test]
fn single_output_native_token_no_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Some((TOKEN_ID_1, 100)),
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
        Some((TOKEN_ID_1, 100)),
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn single_output_native_token_remainder_1() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Some((TOKEN_ID_1, 100)),
            None,
            None,
            None,
            None,
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic(
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 50)),
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[0],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 50))
    ));
}

#[test]
fn single_output_native_token_remainder_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Some((TOKEN_ID_1, 100)),
            None,
            None,
            None,
            None,
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic(
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 100)),
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None
    ));
}

#[test]
fn two_basic_outputs_1() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 200)),
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
        500_000,
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert!(selected.inputs.contains(&inputs[0]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 100)),
    ));
}

#[test]
fn two_basic_outputs_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 200)),
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
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 50)),
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert!(selected.inputs.contains(&inputs[0]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 50)),
    ));
}

#[test]
fn two_basic_outputs_3() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 200)),
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
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 75)),
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert!(selected.inputs.contains(&inputs[0]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 25)),
    ));
}

#[test]
fn two_basic_outputs_4() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 200)),
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
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 100)),
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert!(selected.inputs.contains(&inputs[0]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
    ));
}

#[test]
fn two_basic_outputs_5() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 200)),
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
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 100)),
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert!(selected.inputs.contains(&inputs[0]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
    ));
}

#[test]
fn two_basic_outputs_6() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 200)),
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
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 250)),
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        1_500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 50)),
    ));
}

#[test]
fn two_basic_outputs_7() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 200)),
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
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 300)),
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        1_500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
    ));
}

#[test]
fn two_basic_outputs_8() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 200)),
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
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 350)),
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
            Err(Error::InsufficientNativeTokenAmount {
                token_id,
                found,
                required,
            }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(300) && required == U256::from(350)));
}

#[test]
fn two_basic_outputs_native_tokens_not_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
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
        ],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic(
        500_000,
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert!(selected.inputs.contains(&inputs[1]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
    ));
}

#[test]
fn multiple_remainders() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                5_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                5_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                5_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 100)),
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                5_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_2, 100)),
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
        15_000_000,
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
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 4);
    assert_eq!(selected.outputs.len(), 3);
    assert!(selected.outputs.contains(&outputs[0]));
    let nt_remainder_min_storage_deposit = 106000;
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(
                is_remainder_or_return(
                    output,
                    nt_remainder_min_storage_deposit,
                    Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    Some((TOKEN_ID_1, 300))
                ) || is_remainder_or_return(
                    output,
                    5_000_000 - nt_remainder_min_storage_deposit,
                    Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    Some((TOKEN_ID_2, 100))
                )
            );
        }
    });
}

// #[test]
// fn higher_nts_count_but_below_max_native_tokens() {
//     let protocol_parameters = protocol_parameters();

//     let mut input_native_tokens_0 = Vec::new();
//     for _ in 0..10 {
//         input_native_tokens_0.push((TokenId::from(rand_bytes_array()).to_string(), 10));
//     }
//     let mut input_native_tokens_1 = Vec::new();
//     for _ in 0..64 {
//         input_native_tokens_1.push((TokenId::from(rand_bytes_array()).to_string(), 10));
//     }

//     let inputs = build_inputs([
//         Basic(
//             3_000_000,
//             BECH32_ADDRESS_ED25519_0,
//             Some(input_native_tokens_0.iter().map(|(t, a)| (t.as_str(), *a)).collect()),
//             None,
//             None,
//             None,
//             None,
//             None,
//         ),
//         Basic(
//             10_000_000,
//             BECH32_ADDRESS_ED25519_0,
//             Some(input_native_tokens_1.iter().map(|(t, a)| (t.as_str(), *a)).collect()),
//             None,
//             None,
//             None,
//             None,
//             None,
//         ),
//     ]);
//     let outputs = build_outputs([Basic(
//         5_000_000,
//         BECH32_ADDRESS_ED25519_0,
//         None,
//         None,
//         None,
//         None,
//         None,
//         None,
//     )]);

//     let selected = InputSelection::new(
//         inputs.clone(),
//         outputs.clone(),
//         addresses([BECH32_ADDRESS_ED25519_0]),
//         protocol_parameters,
//     )
//     .select()
//     .unwrap();

//     assert_eq!(selected.inputs.len(), 1);
//     assert!(selected.inputs.contains(&inputs[1]));
//     assert_eq!(selected.outputs.len(), 2);
//     assert!(selected.outputs.contains(&outputs[0]));
//     assert!(is_remainder_or_return(
//         &selected.outputs[1],
//         5_000_000,
//         BECH32_ADDRESS_ED25519_0,
//         Some(input_native_tokens_1.iter().map(|(t, a)| (t.as_str(), *a)).collect()),
//     ));
// }

// T27: :wavy_dash:
// inputs: [basic{ amount: 1_000_000, native_tokens: [{‘a’: 100}] }, basic{ amount: 1_000_000, native_tokens: [{‘a’:
// 200}] }] }] outputs: [basic{ amount: 500_000, native_tokens: [{‘a’: 150}] }]
// expected selected: [basic{ amount: 1_000_000, native_tokens: [{‘a’: 200}] }]
// expected remainder: Some(basic{ amount: 500_000, native_tokens: [{‘a’: 50}] })

// T28: :wavy_dash:
// inputs: [basic{ amount: 1_000_000, native_tokens: [{‘a’: 100}] }, basic{ amount: 1_000_000, native_tokens: [{‘a’:
// 200}] }] }] outputs: [basic{ amount: 500_000, native_tokens: [{‘a’: 200}] }]
// expected selected: [basic{ amount: 1_000_000, native_tokens: [{‘a’: 200}] }]
// expected remainder: Some(basic{ amount: 500_000 })
