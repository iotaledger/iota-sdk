// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::{
    client::api::input_selection::{Error, InputSelection, Requirement},
    types::block::{
        address::{Address, MultiAddress, RestrictedAddress, WeightedAddress},
        output::{AccountId, NftId},
        protocol::protocol_parameters,
    },
};
use pretty_assertions::assert_eq;

use crate::client::{
    build_inputs, build_outputs, is_remainder_or_return, unsorted_eq,
    Build::{Account, Basic, Nft},
    ACCOUNT_ID_0, ACCOUNT_ID_1, BECH32_ADDRESS_ACCOUNT_1, BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1,
    BECH32_ADDRESS_ED25519_2, BECH32_ADDRESS_NFT_1, BECH32_ADDRESS_REMAINDER, NFT_ID_0, NFT_ID_1,
};

#[test]
fn input_amount_equal_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
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
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn input_amount_lower_than_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
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
fn input_amount_lower_than_output_amount_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([
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
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs([Basic(
        3_500_000,
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
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientAmount {
            found: 3_000_000,
            required: 3_500_000,
        })
    ));
}

#[test]
fn input_amount_greater_than_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
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
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    // One output should be added for the remainder.
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_500_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
            ));
        }
    });
}

#[test]
fn input_amount_greater_than_output_amount_with_remainder_address() {
    let protocol_parameters = protocol_parameters();
    let remainder_address = Address::try_from_bech32(BECH32_ADDRESS_REMAINDER).unwrap();

    let inputs = build_inputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
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
        protocol_parameters,
    )
    .with_remainder_address(remainder_address)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    // One output should be added for the remainder.
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_500_000,
                Address::try_from_bech32(BECH32_ADDRESS_REMAINDER).unwrap(),
                None,
            ));
        }
    });
}

#[test]
fn two_same_inputs_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
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
        inputs,
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        protocol_parameters,
    )
    .select()
    .unwrap();

    // One input has enough amount.
    assert_eq!(selected.inputs.len(), 1);
    // One output should be added for the remainder.
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_500_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
            ));
        }
    });
}

#[test]
fn two_inputs_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([
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
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
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
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs, [inputs[0].clone()]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_inputs_one_needed_reversed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([
        Basic(
            2_000_000,
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
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
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
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs, [inputs[1].clone()]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_inputs_both_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([
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
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs([Basic(
        3_000_000,
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
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_inputs_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([
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
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs([Basic(
        2_500_000,
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
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    // One output should be added for the remainder.
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                500_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None
            ));
        }
    });
}

// #[test]
// fn not_enough_storage_deposit_for_remainder() {
//     let protocol_parameters = protocol_parameters();

//     let inputs = build_inputs([Basic(
//         1_000_001,
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         None,
//         None,
//         None,
//         None,
//         None,
//         None,
//     )]);
//     let outputs = build_outputs([Basic(
//         1_000_000,
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         None,
//         None,
//         None,
//         None,
//         None,
//         None,
//     )]);

//     let selected = InputSelection::new(
//         inputs,
//         outputs,
//         [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
//         protocol_parameters,
//     )
//     .select();

//     assert!(matches!(
//         selected,
//         Err(Error::InsufficientAmount {
//             found: 1_000_001,
//             required: 1_217_000,
//         })
//     ));
// }

#[test]
fn ed25519_sender() {
    let protocol_parameters = protocol_parameters();
    let sender = Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap();

    let inputs = build_inputs([
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            2_000_000,
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
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        [
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        ],
        protocol_parameters,
    )
    .select()
    .unwrap();

    // Sender + another for amount
    assert_eq!(selected.inputs.len(), 2);
    assert!(
        selected
            .inputs
            .iter()
            .any(|input| *input.output.as_basic().address() == sender)
    );
    // Provided output + remainder
    assert_eq!(selected.outputs.len(), 2);
}

#[test]
fn missing_ed25519_sender() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        5_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn account_sender() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);

    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        protocol_parameters,
    )
    .select()
    .unwrap();

    // Sender + another for amount
    assert_eq!(selected.inputs.len(), 2);
    assert!(
        selected
            .inputs
            .iter()
            .any(|input| input.output.is_account() && *input.output.as_account().account_id() == account_id_1)
    );
    // Provided output + account
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
}

#[test]
fn account_sender_zero_id() {
    let protocol_parameters = protocol_parameters();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs([
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Account(
            1_000_000,
            account_id_0,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
        ),
    ]);
    let account_id = AccountId::from(inputs[1].output_id());
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(Address::from(account_id)),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(
        selected
            .outputs
            .iter()
            .any(|output| output.is_account() && *output.as_account().account_id() == account_id)
    );
}

#[test]
fn missing_account_sender() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        5_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()
    ));
}

#[test]
fn nft_sender() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs([
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Nft(
            1_000_000,
            nft_id_1,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        protocol_parameters,
    )
    .select()
    .unwrap();

    // Sender + another for amount
    assert_eq!(selected.inputs.len(), 2);
    assert!(
        selected
            .inputs
            .iter()
            .any(|input| input.output.is_nft() && *input.output.as_nft().nft_id() == nft_id_1)
    );
    // Provided output + nft
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&inputs[2].output));
    assert!(selected.outputs.contains(&outputs[0]));
}

#[test]
fn nft_sender_zero_id() {
    let protocol_parameters = protocol_parameters();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

    let inputs = build_inputs([
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Nft(
            1_000_000,
            nft_id_0,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let nft_id = NftId::from(inputs[1].output_id());
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(Address::from(nft_id)),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(
        selected
            .outputs
            .iter()
            .any(|output| output.is_nft() && *output.as_nft().nft_id() == nft_id)
    );
}

#[test]
fn missing_nft_sender() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        5_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()
    ));
}

#[test]
fn simple_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
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
                500_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None
            ));
        }
    });
}

// #[test]
// fn remainder_lower_than_rent() {
//     let protocol_parameters = protocol_parameters();

//     let inputs = build_inputs([Basic(
//         1_000_000,
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         None,
//         None,
//         None,
//         None,
//         None,
//         None,
//     )]);
//     let outputs = build_outputs([Basic(
//         800_000,
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         None,
//         None,
//         None,
//         None,
//         None,
//         None,
//     )]);

//     let selected = InputSelection::new(
//         inputs,
//         outputs,
//         [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
//         protocol_parameters,
//     )
//     .select();

//     println!("{selected:?}");

//     assert!(matches!(
//         selected,
//         Err(Error::InsufficientAmount {
//             found: 1_000_000,
//             required: 1_017_000,
//         })
//     ));
// }

// #[test]
// fn remainder_lower_than_rent_2() {
//     let protocol_parameters = protocol_parameters();

//     let inputs = build_inputs([
//         Basic(1_000_000, Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), None, None, None, None, None,
// None),         Basic(2_000_000, Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), None, None, None, None,
// None, None),     ]);
//     let outputs = build_outputs([Basic(
//         2_800_000,
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         None,
//         None,
//         None,
//         None,
//         None,
//         None,
//     )]);

//     let selected = InputSelection::new(
//         inputs,
//         outputs,
//         [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
//         protocol_parameters,
//     )
//     .select();

//     assert!(matches!(
//         selected,
//         Err(Error::InsufficientAmount {
//             found: 3_000_000,
//             required: 3_017_000,
//         })
//     ));
// }

#[test]
fn one_provided_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
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
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn insufficient_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Basic(
        1_250_000,
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
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientAmount {
            found: 1_000_000,
            required: 1_250_000,
        })
    ));
}

#[test]
fn two_inputs_remainder_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([
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
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
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
                500_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None
            ));
        }
    });
}

#[test]
fn two_inputs_remainder_3() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([
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
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs([Basic(
        1_750_000,
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
                1_250_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None
            ));
        }
    });
}

// TODO: re-enabled when rent is figured out
// #[test]
// fn another_input_required_to_cover_remainder_rent() {
//     let protocol_parameters = protocol_parameters();

//     let inputs = build_inputs([
//         Basic(500_000, Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), None, None, None, None, None,
// None),         Basic(600_000, Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), None, None, None, None,
// None, None),         Basic(700_000, Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), None, None, None,
// None, None, None),     ]);
//     let outputs = build_outputs([Basic(
//         1_000_000,
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
//         [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
//         protocol_parameters,
//     )
//     .select()
//     .unwrap();

//     assert!(unsorted_eq(&selected.inputs, &inputs));
//     assert_eq!(selected.outputs.len(), 2);
//     assert!(selected.outputs.contains(&outputs[0]));
//     selected.outputs.iter().for_each(|output| {
//         if !outputs.contains(output) {
//             assert!(is_remainder_or_return(output, 800_000,
// Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), None));         }
//     });
// }

#[test]
fn sender_already_selected() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
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
        protocol_parameters,
    )
    .with_required_inputs([*inputs[0].output_id()])
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn single_mandatory_input() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
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
        [
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        ],
        protocol_parameters,
    )
    .with_required_inputs([*inputs[0].output_id()])
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn too_many_inputs() {
    let protocol_parameters = protocol_parameters();

    // 129 inputs that would be required for the amount, but that's above max inputs
    let inputs = build_inputs(
        std::iter::repeat_with(|| {
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            )
        })
        .take(129),
    );

    let outputs = build_outputs([Basic(
        129_000_000,
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
        protocol_parameters,
    )
    .select();

    assert_eq!(
        selected.unwrap_err(),
        iota_sdk::client::api::input_selection::Error::InvalidInputCount(129)
    )
}

#[test]
fn more_than_max_inputs_only_one_needed() {
    let protocol_parameters = protocol_parameters();

    // 1000 inputs where 129 would be needed for the required amount which is above the max inputs
    let mut inputs = build_inputs(
        std::iter::repeat_with(|| {
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            )
        })
        .take(1000),
    );
    // Add the needed input
    let needed_input = build_inputs([Basic(
        129_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    inputs.push(needed_input[0].clone());

    let outputs = build_outputs([Basic(
        129_000_000,
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
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &needed_input));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn too_many_outputs() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        2_000_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let outputs = build_outputs(
        std::iter::repeat_with(|| {
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            )
        })
        .take(129),
    );

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        protocol_parameters,
    )
    .select();

    assert_eq!(
        selected.unwrap_err(),
        iota_sdk::client::api::input_selection::Error::InvalidOutputCount(129)
    )
}

#[test]
fn too_many_outputs_with_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        2_000_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let outputs = build_outputs(
        std::iter::repeat_with(|| {
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            )
        })
        .take(128),
    );

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        protocol_parameters,
    )
    .select();

    assert_eq!(
        selected.unwrap_err(),
        // 129 because of required remainder
        iota_sdk::client::api::input_selection::Error::InvalidOutputCount(129)
    )
}

#[test]
fn restricted_ed25519() {
    let protocol_parameters = protocol_parameters();
    let address = Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap();
    let restricted = Address::from(RestrictedAddress::new(address.clone()).unwrap());

    let inputs = build_inputs([
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
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, restricted, None, None, None, None, None, None),
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
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
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
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()],
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs, [inputs[2].clone()]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn restricted_nft() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();
    let nft_address = Address::from(nft_id_1);
    let restricted = Address::from(RestrictedAddress::new(nft_address.clone()).unwrap());

    let inputs = build_inputs([
        Basic(2_000_000, restricted, None, None, None, None, None, None),
        Nft(
            2_000_000,
            nft_id_1,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs([Basic(
        3_000_000,
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
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
}

#[test]
fn restricted_account() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let account_address = Address::from(account_id_1);
    let restricted = Address::from(RestrictedAddress::new(account_address.clone()).unwrap());

    let inputs = build_inputs([
        Basic(3_000_000, restricted, None, None, None, None, None, None),
        Account(
            2_000_000,
            account_id_1,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
        ),
    ]);

    let outputs = build_outputs([Basic(
        3_000_000,
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
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
}

#[test]
fn restricted_ed25519_sender() {
    let protocol_parameters = protocol_parameters();
    let sender = Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap();
    let restricted_sender = Address::from(RestrictedAddress::new(sender.clone()).unwrap());

    let inputs = build_inputs([
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            2_000_000,
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
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(restricted_sender),
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
        protocol_parameters,
    )
    .select()
    .unwrap();

    // Sender + another for amount
    assert_eq!(selected.inputs.len(), 2);
    assert!(
        selected
            .inputs
            .iter()
            .any(|input| *input.output.as_basic().address() == sender)
    );
    // Provided output + remainder
    assert_eq!(selected.outputs.len(), 2);
}

#[test]
fn multi_address_sender_already_fulfilled() {
    let protocol_parameters = protocol_parameters();
    let sender_0 = Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap();
    let sender_1 = Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap();
    let sender_2 = Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap();
    let multi = Address::from(
        MultiAddress::new(
            [
                WeightedAddress::new(sender_0, 1).unwrap(),
                WeightedAddress::new(sender_1, 1).unwrap(),
                WeightedAddress::new(sender_2, 1).unwrap(),
            ],
            3,
        )
        .unwrap(),
    );

    let inputs = build_inputs([
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
        Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs([Basic(
        3_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(multi),
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
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
        ],
        protocol_parameters,
    )
    .with_required_inputs([*inputs[0].output_id(), *inputs[1].output_id(), *inputs[2].output_id()])
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}
