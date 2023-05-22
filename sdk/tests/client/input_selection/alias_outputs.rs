// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashSet, str::FromStr};

use iota_sdk::{
    client::{
        api::input_selection::{Burn, Error, InputSelection, Requirement},
        secret::types::InputSigningData,
    },
    types::block::{
        address::Address,
        output::{
            unlock_condition::{GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition},
            AliasId, AliasOutputBuilder, AliasTransition, Output, OutputMetadata,
        },
        protocol::protocol_parameters,
        rand::{block::rand_block_id, output::rand_output_id},
    },
};

use crate::client::{
    addresses, build_inputs, build_outputs, is_remainder_or_return, unsorted_eq,
    Build::{Alias, Basic},
    ALIAS_ID_0, ALIAS_ID_1, ALIAS_ID_2, BECH32_ADDRESS_ALIAS_1, BECH32_ADDRESS_ALIAS_2, BECH32_ADDRESS_ED25519_0,
    BECH32_ADDRESS_ED25519_1, BECH32_ADDRESS_ED25519_2, BECH32_ADDRESS_NFT_1, TOKEN_SUPPLY,
};

#[test]
fn input_alias_eq_output_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn transition_alias_id_zero() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_inputs(vec![Alias(
        1_000_000,
        alias_id_0,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let alias_id = AliasId::from(inputs[0].output_id());
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn input_amount_lt_output_amount() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
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
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientAmount {
            found: 1_000_000,
            // Amount we want to send + storage deposit for alias remainder
            required: 2_251_500,
        })
    ));
}

#[test]
fn input_amount_lt_output_amount_2() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            2_000_000,
            alias_id_2,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(
        3_000_001,
        BECH32_ADDRESS_ED25519_0,
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
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientAmount {
            found: 3_000_000,
            // Amount we want to send + storage deposit for alias remainder
            required: 3_251_501
        })
    ));
}

#[test]
fn basic_output_with_alias_input() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_251_500,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    // basic output + alias remainder
    assert_eq!(selected.outputs.len(), 2);
}

#[test]
fn create_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_0,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    // One output should be added for the remainder
    assert_eq!(selected.outputs.len(), 2);
    // Output contains the new minted alias id
    assert!(selected.outputs.iter().any(|output| {
        if let Output::Alias(alias_output) = output {
            *alias_output.alias_id() == alias_id_0
        } else {
            false
        }
    }));
}

#[test]
fn burn_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
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
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_alias(alias_id_2))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn not_enough_storage_deposit_for_remainder() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(
        1_000_001,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientAmount {
            found: 1_000_001,
            required: 1_213_000,
        })
    ));
}

#[test]
fn missing_input_for_alias_output() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id, AliasTransition::Governance))) if alias_id == alias_id_2
    ));
}

#[test]
fn missing_input_for_alias_output_2() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            2_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id, AliasTransition::Governance))) if alias_id == alias_id_2
    ));
}

#[test]
fn missing_input_for_alias_output_but_created() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_0,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(selected.is_ok());
}

#[test]
fn alias_in_output_and_sender() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            1_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let alias_output = AliasOutputBuilder::from(inputs[0].output.as_alias())
        .with_state_index(inputs[0].output.as_alias().state_index() + 1)
        .finish_output(TOKEN_SUPPLY)
        .unwrap();
    let mut outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ALIAS_1),
        None,
        None,
        None,
        None,
    )]);
    outputs.push(alias_output);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn missing_ed25519_sender() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ED25519_1),
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn missing_ed25519_issuer_created() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_0,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ED25519_1),
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn missing_ed25519_issuer_transition() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        1_000_000,
        alias_id_1,
        1,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ED25519_1),
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_1,
        2,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ED25519_1),
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(selected.is_ok());
}

#[test]
fn missing_alias_sender() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ALIAS_1),
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ALIAS_1).unwrap()
    ));
}

#[test]
fn missing_alias_issuer_created() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_0,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ALIAS_1),
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer == Address::try_from_bech32(BECH32_ADDRESS_ALIAS_1).unwrap()
    ));
}

#[test]
fn missing_alias_issuer_transition() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(
        1_000_000,
        alias_id_2,
        1,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ALIAS_1),
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_2,
        1,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ALIAS_1),
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(selected.is_ok());
}

#[test]
fn missing_nft_sender() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_NFT_1),
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()
    ));
}

#[test]
fn missing_nft_issuer_created() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_0,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_NFT_1),
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer == Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()
    ));
}

#[test]
fn missing_nft_issuer_transition() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        1_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_NFT_1),
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_NFT_1),
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(selected.is_ok());
}

#[test]
fn increase_alias_amount() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            2_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Alias(
        3_000_000,
        alias_id_1,
        1,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn decrease_alias_amount() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            2_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_1,
        1,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[0]);
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                BECH32_ADDRESS_ED25519_0,
                None
            ));
        }
    });
}

#[test]
fn prefer_basic_to_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            1_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
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
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn take_amount_from_alias_to_fund_basic() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            2_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_200_000,
        BECH32_ADDRESS_ED25519_0,
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
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_alias());
            assert_eq!(output.amount(), 1_800_000);
            assert_eq!(output.as_alias().native_tokens().len(), 0);
            assert_eq!(*output.as_alias().alias_id(), alias_id_1);
            assert_eq!(output.as_alias().unlock_conditions().len(), 2);
            assert_eq!(output.as_alias().features().len(), 0);
            assert_eq!(output.as_alias().immutable_features().len(), 0);
            assert_eq!(
                *output.as_alias().state_controller_address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
            );
            assert_eq!(
                *output.as_alias().governor_address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
            );
        }
    });
}

#[test]
fn alias_burn_should_not_validate_alias_sender() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(2_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Alias(
            1_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ALIAS_1),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_alias(alias_id_1))
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ALIAS_1).unwrap()
    ));
}

#[test]
fn alias_burn_should_not_validate_alias_address() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(2_000_000, BECH32_ADDRESS_ALIAS_1, None, None, None, None, None, None),
        Alias(
            1_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
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
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_alias(alias_id_1))
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id, AliasTransition::State))) if alias_id == alias_id_1
    ));
}

#[test]
fn alias_governance_transition_should_not_validate_alias_sender() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(2_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Alias(
            1_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let mut outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ALIAS_1),
        None,
        None,
        None,
        None,
    )]);
    outputs.push(inputs[1].output.clone());

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ALIAS_1).unwrap()
    ));
}

#[test]
fn alias_governance_transition_should_not_validate_alias_address() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(2_000_000, BECH32_ADDRESS_ALIAS_1, None, None, None, None, None, None),
        Alias(
            1_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let mut outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    outputs.push(inputs[1].output.clone());

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id, AliasTransition::State))) if alias_id == alias_id_1
    ));
}

#[test]
fn transitioned_zero_alias_id_no_longer_is_zero() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_0,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
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
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_alias());
            assert_eq!(output.amount(), 1_000_000);
            assert_eq!(output.as_alias().native_tokens().len(), 0);
            assert_ne!(*output.as_alias().alias_id(), alias_id_0);
            assert_eq!(output.as_alias().unlock_conditions().len(), 2);
            assert_eq!(output.as_alias().features().len(), 0);
            assert_eq!(output.as_alias().immutable_features().len(), 0);
            assert_eq!(
                *output.as_alias().state_controller_address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
            );
            assert_eq!(
                *output.as_alias().governor_address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
            );
        }
    });
}

#[test]
fn two_aliases_required() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            2_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Alias(
            2_000_000,
            alias_id_2,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        3_000_000,
        BECH32_ADDRESS_ED25519_0,
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
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 3);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(
        selected
            .outputs
            .iter()
            .any(|output| if let Output::Alias(output) = output {
                output.alias_id() == &alias_id_1
            } else {
                false
            })
    );
    assert!(
        selected
            .outputs
            .iter()
            .any(|output| if let Output::Alias(output) = output {
                output.alias_id() == &alias_id_2
            } else {
                false
            })
    )
}

#[test]
fn state_controller_sender_required() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ED25519_0),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(
        selected
            .outputs
            .iter()
            .any(|output| if let Output::Alias(output) = output {
                output.state_index() == inputs[0].output.as_alias().state_index() + 1
            } else {
                false
            })
    )
}

#[test]
fn state_controller_sender_required_already_selected() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![
        Alias(
            1_000_000,
            alias_id_1,
            1,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            Some(BECH32_ADDRESS_ED25519_0),
            None,
            None,
            None,
            None,
        ),
    ]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .required_inputs(HashSet::from_iter([*inputs[0].output_id()]))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn state_controller_sender_required_but_governance() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![
        Alias(
            1_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            Some(BECH32_ADDRESS_ED25519_0),
            None,
            None,
            None,
            None,
        ),
    ]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .required_inputs(HashSet::from_iter([*inputs[0].output_id()]))
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
    ));
}

#[test]
fn governor_sender_required() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            2_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ED25519_1),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(
        selected
            .outputs
            .iter()
            .any(|output| if let Output::Alias(output) = output {
                output.state_index() == inputs[0].output.as_alias().state_index()
            } else {
                false
            })
    )
}

#[test]
fn governor_sender_required_already_selected() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            1_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_1, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![
        Alias(
            1_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            Some(BECH32_ADDRESS_ED25519_1),
            None,
            None,
            None,
            None,
        ),
    ]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_1]),
        protocol_parameters,
    )
    .required_inputs(HashSet::from_iter([*inputs[0].output_id()]))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn governance_transition_and_required() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_1]),
        protocol_parameters,
    )
    .required_inputs(HashSet::from_iter([*inputs[0].output_id()]))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn state_transition_and_required() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        2_000_000,
        alias_id_1,
        1,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .required_inputs(HashSet::from_iter([*inputs[0].output_id()]))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn governor_sender_required_but_state() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![
        Alias(
            1_000_000,
            alias_id_1,
            1,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            Some(BECH32_ADDRESS_ED25519_1),
            None,
            None,
            None,
            None,
        ),
    ]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .required_inputs(HashSet::from_iter([*inputs[0].output_id()]))
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn both_state_controller_and_governor_sender() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            Some(BECH32_ADDRESS_ED25519_0),
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            Some(BECH32_ADDRESS_ED25519_1),
            None,
            None,
            None,
            None,
        ),
    ]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
    ));
}

#[test]
fn remainder_address_in_state_controller() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ALIAS_2,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_1,
        1,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ALIAS_2,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
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
                BECH32_ADDRESS_ED25519_0,
                None
            ));
        }
    });
}

#[test]
fn remainder_address_in_governor() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            1_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ALIAS_2,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ALIAS_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    // Add the basic output so it will be consumed
    .required_inputs(HashSet::from_iter(vec![*inputs[1].output_id()]))
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
                BECH32_ADDRESS_ED25519_0,
                None
            ));
        }
    });
}

#[test]
fn do_not_change_amount_of_governance_transition() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ED25519_1),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_1]),
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
fn state_transition_required_but_state_controller_not_provided() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
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
        addresses(vec![BECH32_ADDRESS_ED25519_1]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientAmount {
            found: 0,
            required: 1_000_000,
        })
    ));
}

#[test]
fn state_transition_but_state_controller_not_owned() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        2_000_000,
        alias_id_1,
        1,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_1]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Ed25519(address))) if address == Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
    ));
}

#[test]
fn governance_transition_but_governor_not_owned() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Ed25519(address))) if address == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn burn_alias_but_governor_not_owned() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
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
        addresses(vec![BECH32_ADDRESS_ED25519_2]),
        protocol_parameters,
    )
    .burn(Burn::new().add_alias(alias_id_1))
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Ed25519(address))) if address == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn sender_in_state_controller_but_not_owned() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ED25519_0),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_2]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
    ));
}

#[test]
fn sender_in_governor_but_not_owned() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ED25519_1),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_2]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn new_state_metadata() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let alias_output =
        AliasOutputBuilder::new_with_minimum_storage_deposit(*protocol_parameters.rent_structure(), alias_id_1)
            .with_state_metadata(vec![1, 2, 3])
            .add_unlock_condition(StateControllerAddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .add_unlock_condition(GovernorAddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output(protocol_parameters.token_supply())
            .unwrap();

    let inputs = vec![InputSigningData {
        output: alias_output.clone(),
        output_metadata: OutputMetadata::new(rand_block_id(), rand_output_id(), false, None, None, None, 0, 0, 0),
        chain: None,
    }];

    // New alias output, with updated state index
    let updated_alias_output = AliasOutputBuilder::from(alias_output.as_alias())
        .with_minimum_storage_deposit(*protocol_parameters.rent_structure())
        .with_state_metadata(vec![3, 4, 5])
        .with_state_index(alias_output.as_alias().state_index() + 1)
        .finish_output(protocol_parameters.token_supply())
        .unwrap();

    let outputs = vec![updated_alias_output];

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn new_state_metadata_but_same_state_index() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let alias_output =
        AliasOutputBuilder::new_with_minimum_storage_deposit(*protocol_parameters.rent_structure(), alias_id_1)
            .with_state_metadata(vec![1, 2, 3])
            .add_unlock_condition(StateControllerAddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .add_unlock_condition(GovernorAddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output(protocol_parameters.token_supply())
            .unwrap();

    let inputs = vec![InputSigningData {
        output: alias_output.clone(),
        output_metadata: OutputMetadata::new(rand_block_id(), rand_output_id(), false, None, None, None, 0, 0, 0),
        chain: None,
    }];

    // New alias output, without updated state index
    let updated_alias_output = AliasOutputBuilder::from(alias_output.as_alias())
        .with_minimum_storage_deposit(*protocol_parameters.rent_structure())
        .with_state_metadata(vec![3, 4, 5])
        .finish_output(protocol_parameters.token_supply())
        .unwrap();

    let outputs = vec![updated_alias_output];

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(
            alias_id,
            _alias_transition,
        ))) if alias_id == alias_id_1
    ));
}
