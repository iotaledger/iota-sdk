// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::{
    client::api::input_selection::{Burn, Error, InputSelection, Requirement},
    types::block::{
        address::{Address, AliasAddress},
        output::{
            dto::NftOutputDto, AliasId, FoundryId, NativeToken, NftId, NftOutput, Output, Rent, SimpleTokenScheme,
            TokenId,
        },
        protocol::protocol_parameters,
        rand::output::{
            feature::{rand_issuer_feature, rand_sender_feature},
            rand_nft_output,
            unlock_condition::rand_address_unlock_condition,
        },
    },
};
use packable::PackableExt;

use crate::client::{
    addresses, build_inputs, build_outputs, is_remainder_or_return, unsorted_eq,
    Build::{Basic, Nft},
    ALIAS_ID_2, BECH32_ADDRESS_ALIAS_1, BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1, BECH32_ADDRESS_NFT_1,
    NFT_ID_0, NFT_ID_1, NFT_ID_2,
};

#[test]
fn input_nft_eq_output_nft() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(vec![Nft(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_2,
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
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn transition_nft_id_zero() {
    let protocol_parameters = protocol_parameters();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

    let inputs = build_inputs(vec![Nft(
        1_000_000,
        nft_id_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let nft_id = NftId::from(inputs[0].output_id());
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id,
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
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn input_amount_lt_output_amount() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(vec![Nft(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
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
            // Amount we want to send + storage deposit for nft remainder
            required: 2_229_500,
        })
    ));
}

#[test]
fn basic_output_with_nft_input() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(vec![Nft(
        2_229_500,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
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
    // basic output + nft remainder
    assert_eq!(selected.outputs.len(), 2);
}

#[test]
fn mint_nft() {
    let protocol_parameters = protocol_parameters();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

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
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_0,
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
    // One output should be added for the remainder
    assert_eq!(selected.outputs.len(), 2);
    // Output contains the new minted nft id
    assert!(selected.outputs.iter().any(|output| {
        if let Output::Nft(nft_output) = output {
            *nft_output.nft_id() == nft_id_0
        } else {
            false
        }
    }));
}

#[test]
fn burn_nft() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(vec![Nft(
        2_000_000,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
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
    .burn(Burn::new().add_nft(nft_id_2))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn not_enough_storage_deposit_for_remainder() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(vec![Nft(
        1_000_001,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_2,
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
            found: 1_000_001,
            required: 1_213_000,
        })
    ));
}

#[test]
fn missing_input_for_nft_output() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

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
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_2,
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
        Err(Error::UnfulfillableRequirement(Requirement::Nft(nft_id))) if nft_id == nft_id_2
    ));
}

#[test]
fn missing_input_for_nft_output_but_created() {
    let protocol_parameters = protocol_parameters();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

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
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_0,
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

    assert!(selected.is_ok());
}

#[test]
fn nft_in_output_and_sender() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Nft(
            1_000_000,
            nft_id_1,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![
        Nft(
            1_000_000,
            nft_id_1,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            Some(BECH32_ADDRESS_NFT_1),
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
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.iter().any(|output| {
        if let Output::Nft(nft_output) = output {
            *nft_output.nft_id() == nft_id_1
        } else {
            false
        }
    }));
    assert!(selected.outputs.iter().any(|output| output.is_basic()));
}

#[test]
fn missing_ed25519_sender() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(vec![Nft(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_2,
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
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

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
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ED25519_1),
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
        Err(Error::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn missing_ed25519_issuer_transition() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(vec![Nft(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ED25519_1),
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
fn missing_alias_sender() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(vec![Nft(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_2,
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
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ALIAS_1).unwrap()
    ));
}

#[test]
fn missing_alias_issuer_created() {
    let protocol_parameters = protocol_parameters();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

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
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ALIAS_1),
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
        Err(Error::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer == Address::try_from_bech32(BECH32_ADDRESS_ALIAS_1).unwrap()
    ));
}

#[test]
fn missing_alias_issuer_transition() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(vec![Nft(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ALIAS_1),
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
fn missing_nft_sender() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(vec![Nft(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_NFT_1),
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
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()
    ));
}

#[test]
fn missing_nft_issuer_created() {
    let protocol_parameters = protocol_parameters();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

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
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_NFT_1),
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
        Err(Error::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer == Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()
    ));
}

#[test]
fn missing_nft_issuer_transition() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(vec![Nft(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_NFT_1),
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
fn increase_nft_amount() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Nft(
            2_000_000,
            nft_id_1,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Nft(
        3_000_000,
        nft_id_1,
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
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn decrease_nft_amount() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Nft(
            2_000_000,
            nft_id_1,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Nft(
        1_000_000,
        nft_id_1,
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
fn prefer_basic_to_nft() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Nft(
            2_000_000,
            nft_id_1,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
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
fn take_amount_from_nft_to_fund_basic() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Nft(
            2_000_000,
            nft_id_1,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
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
            assert!(output.is_nft());
            assert_eq!(output.amount(), 1_800_000);
            assert_eq!(output.as_nft().native_tokens().len(), 0);
            assert_eq!(output.as_nft().unlock_conditions().len(), 1);
            assert_eq!(output.as_nft().features().len(), 0);
            assert_eq!(
                *output.as_nft().address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
            );
        }
    });
}

#[test]
fn nft_burn_should_validate_nft_sender() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(2_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Nft(
            1_000_000,
            nft_id_1,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
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
        Some(BECH32_ADDRESS_NFT_1),
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
    .burn(Burn::new().add_nft(nft_id_1))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn nft_burn_should_validate_nft_address() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(2_000_000, BECH32_ADDRESS_NFT_1, None, None, None, None, None, None),
        Nft(
            1_000_000,
            nft_id_1,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
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
    .burn(Burn::new().add_nft(nft_id_1))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn transitioned_zero_nft_id_no_longer_is_zero() {
    let protocol_parameters = protocol_parameters();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

    let inputs = build_inputs(vec![Nft(
        2_000_000,
        nft_id_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
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
            assert!(output.is_nft());
            assert_eq!(output.amount(), 1_000_000);
            assert_ne!(*output.as_nft().nft_id(), nft_id_0);
            assert_eq!(output.as_nft().native_tokens().len(), 0);
            assert_eq!(output.as_nft().unlock_conditions().len(), 1);
            assert_eq!(output.as_nft().features().len(), 0);
            assert_eq!(
                *output.as_nft().address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
            );
        }
    });
}

#[test]
fn nft_builder_replace() {
    let protocol_parameters = protocol_parameters();
    let alias_id = AliasId::from_str(ALIAS_ID_2).unwrap();
    let foundry_id = FoundryId::build(&AliasAddress::from(alias_id), 0, SimpleTokenScheme::KIND);
    let address_1 = rand_address_unlock_condition();
    let address_2 = rand_address_unlock_condition();
    let sender_1 = rand_sender_feature();
    let sender_2 = rand_sender_feature();
    let issuer_1 = rand_issuer_feature();
    let issuer_2 = rand_issuer_feature();

    let mut builder =
        NftOutput::build_with_minimum_storage_deposit(*protocol_parameters.rent_structure(), NftId::null())
            .add_native_token(NativeToken::new(TokenId::from(foundry_id), 1000.into()).unwrap())
            .add_unlock_condition(address_1)
            .add_feature(sender_1)
            .replace_feature(sender_2)
            .replace_immutable_feature(issuer_1)
            .add_immutable_feature(issuer_2);

    let output = builder.clone().finish_unverified().unwrap();
    assert_eq!(
        output.amount(),
        Output::Nft(output.clone()).rent_cost(protocol_parameters.rent_structure())
    );
    assert_eq!(output.unlock_conditions().address(), Some(&address_1));
    assert_eq!(output.features().sender(), Some(&sender_2));
    assert_eq!(output.immutable_features().issuer(), Some(&issuer_1));

    builder = builder
        .clear_unlock_conditions()
        .clear_features()
        .clear_immutable_features()
        .replace_unlock_condition(address_2);
    let output = builder.finish_unverified().unwrap();
    assert_eq!(output.unlock_conditions().address(), Some(&address_2));
    assert!(output.features().is_empty());
    assert!(output.immutable_features().is_empty());
}

#[test]
fn nft_output_pack_unpack() {
    let protocol_parameters = protocol_parameters();
    let output = rand_nft_output(protocol_parameters.token_supply());
    let bytes = output.pack_to_vec();
    let output_unpacked = NftOutput::unpack_verified(bytes, &protocol_parameters).unwrap();
    assert_eq!(output, output_unpacked);
}

#[test]
fn nft_output_to_from_dto() {
    let protocol_parameters = protocol_parameters();
    let output = rand_nft_output(protocol_parameters.token_supply());
    let dto = NftOutputDto::from(&output);
    let output_unver = NftOutput::try_from_dto_unverified(&dto).unwrap();
    assert_eq!(output, output_unver);
    let output_ver = NftOutput::try_from_dto(&dto, protocol_parameters.token_supply()).unwrap();
    assert_eq!(output, output_ver);
}
