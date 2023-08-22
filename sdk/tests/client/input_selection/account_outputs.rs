// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::{
    client::{
        api::input_selection::{Burn, Error, InputSelection, Requirement},
        secret::types::InputSigningData,
    },
    types::block::{
        address::Address,
        output::{
            unlock_condition::{GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition},
            AccountId, AccountOutputBuilder, AccountTransition, Output,
        },
        protocol::protocol_parameters,
        rand::output::rand_output_metadata,
    },
};

use crate::client::{
    addresses, build_inputs, build_outputs, is_remainder_or_return, unsorted_eq,
    Build::{Account, Basic},
    ACCOUNT_ID_0, ACCOUNT_ID_1, ACCOUNT_ID_2, BECH32_ADDRESS_ACCOUNT_1, BECH32_ADDRESS_ACCOUNT_2,
    BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1, BECH32_ADDRESS_ED25519_2, BECH32_ADDRESS_NFT_1, TOKEN_SUPPLY,
};

#[test]
fn input_account_eq_output_account() {
    let protocol_parameters = protocol_parameters();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs([Account(
        1_000_000,
        account_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_2,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn transition_account_id_zero() {
    let protocol_parameters = protocol_parameters();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs([Account(
        1_000_000,
        account_id_0,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let account_id = AccountId::from(inputs[0].output_id());
    let outputs = build_outputs([Account(
        1_000_000,
        account_id,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

// #[test]
// fn input_amount_lt_output_amount() {
//     let protocol_parameters = protocol_parameters();
//     let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

//     let inputs = build_inputs([Account(
//         1_000_000,
//         account_id_2,
//         0,
//         BECH32_ADDRESS_ED25519_0,
//         BECH32_ADDRESS_ED25519_0,
//         None,
//         None,
//         None,
//         None,
//     )]);
//     let outputs = build_outputs([Basic(
//         2_000_000,
//         BECH32_ADDRESS_ED25519_0,
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
//         addresses([BECH32_ADDRESS_ED25519_0]),
//         protocol_parameters,
//     )
//     .select();

//     assert!(matches!(
//         selected,
//         Err(Error::InsufficientAmount {
//             found: 1_000_000,
//             // Amount we want to send + storage deposit for account remainder
//             required: 2_255_500,
//         })
//     ));
// }

// #[test]
// fn input_amount_lt_output_amount_2() {
//     let protocol_parameters = protocol_parameters();
//     let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

//     let inputs = build_inputs([
//         Account(
//             2_000_000,
//             account_id_2,
//             0,
//             BECH32_ADDRESS_ED25519_0,
//             BECH32_ADDRESS_ED25519_0,
//             None,
//             None,
//             None,
//             None,
//         ),
//         Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
//     ]);
//     let outputs = build_outputs([Basic(
//         3_000_001,
//         BECH32_ADDRESS_ED25519_0,
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
//         addresses([BECH32_ADDRESS_ED25519_0]),
//         protocol_parameters,
//     )
//     .select();

//     assert!(matches!(
//         selected,
//         Err(Error::InsufficientAmount {
//             found: 3_000_000,
//             // Amount we want to send + storage deposit for account remainder
//             required: 3_255_501
//         })
//     ));
// }

// #[test]
// fn basic_output_with_account_input() {
//     let protocol_parameters = protocol_parameters();
//     let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

//     let inputs = build_inputs([Account(
//         2_259_500,
//         account_id_2,
//         0,
//         BECH32_ADDRESS_ED25519_0,
//         BECH32_ADDRESS_ED25519_0,
//         None,
//         None,
//         None,
//         None,
//     )]);
//     let outputs = build_outputs([Basic(
//         2_000_000,
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
//         outputs,
//         addresses([BECH32_ADDRESS_ED25519_0]),
//         protocol_parameters,
//     )
//     .select()
//     .unwrap();

//     assert!(unsorted_eq(&selected.inputs, &inputs));
//     // basic output + account remainder
//     assert_eq!(selected.outputs.len(), 2);
// }

#[test]
fn create_account() {
    let protocol_parameters = protocol_parameters();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs([Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_0,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    // One output should be added for the remainder
    assert_eq!(selected.outputs.len(), 2);
    // Output contains the new minted account id
    assert!(selected.outputs.iter().any(|output| {
        if let Output::Account(account_output) = output {
            *account_output.account_id() == account_id_0
        } else {
            false
        }
    }));
}

#[test]
fn burn_account() {
    let protocol_parameters = protocol_parameters();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_account(account_id_2))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

// #[test]
// fn not_enough_storage_deposit_for_remainder() {
//     let protocol_parameters = protocol_parameters();
//     let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

//     let inputs = build_inputs([Account(
//         1_000_001,
//         account_id_2,
//         0,
//         BECH32_ADDRESS_ED25519_0,
//         BECH32_ADDRESS_ED25519_0,
//         None,
//         None,
//         None,
//         None,
//     )]);
//     let outputs = build_outputs([Account(
//         1_000_000,
//         account_id_2,
//         0,
//         BECH32_ADDRESS_ED25519_0,
//         BECH32_ADDRESS_ED25519_0,
//         None,
//         None,
//         None,
//         None,
//     )]);

//     let selected = InputSelection::new(
//         inputs,
//         outputs,
//         addresses([BECH32_ADDRESS_ED25519_0]),
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
fn missing_input_for_account_output() {
    let protocol_parameters = protocol_parameters();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs([Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_2,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Account(account_id, AccountTransition::Governance))) if account_id == account_id_2
    ));
}

#[test]
fn missing_input_for_account_output_2() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs([
        Account(
            2_000_000,
            account_id_1,
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
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_2,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Account(account_id, AccountTransition::Governance))) if account_id == account_id_2
    ));
}

#[test]
fn missing_input_for_account_output_but_created() {
    let protocol_parameters = protocol_parameters();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs([Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_0,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(selected.is_ok());
}

#[test]
fn account_in_output_and_sender() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Account(
            1_000_000,
            account_id_1,
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
    let account_output = AccountOutputBuilder::from(inputs[0].output.as_account())
        .with_state_index(inputs[0].output.as_account().state_index() + 1)
        .finish_output(TOKEN_SUPPLY)
        .unwrap();
    let mut outputs = build_outputs([Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ACCOUNT_1),
        None,
        None,
        None,
        None,
    )]);
    outputs.push(account_output);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses([BECH32_ADDRESS_ED25519_0]),
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
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs([Account(
        1_000_000,
        account_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_2,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
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
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs([Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_0,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
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
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        1_000_000,
        account_id_1,
        1,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ED25519_1),
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_1,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(selected.is_ok());
}

#[test]
fn missing_account_sender() {
    let protocol_parameters = protocol_parameters();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs([Account(
        1_000_000,
        account_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ACCOUNT_1),
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()
    ));
}

#[test]
fn missing_account_issuer_created() {
    let protocol_parameters = protocol_parameters();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs([Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_0,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ACCOUNT_1),
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer == Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()
    ));
}

#[test]
fn missing_account_issuer_transition() {
    let protocol_parameters = protocol_parameters();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs([Account(
        1_000_000,
        account_id_2,
        1,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ACCOUNT_1),
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_2,
        1,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_ACCOUNT_1),
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(selected.is_ok());
}

#[test]
fn missing_nft_sender() {
    let protocol_parameters = protocol_parameters();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs([Account(
        1_000_000,
        account_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_2,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
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
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs([Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_0,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
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
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        1_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some(BECH32_ADDRESS_NFT_1),
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_1,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(selected.is_ok());
}

#[test]
fn increase_account_amount() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Account(
            2_000_000,
            account_id_1,
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
    let outputs = build_outputs([Account(
        3_000_000,
        account_id_1,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn decrease_account_amount() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Account(
            2_000_000,
            account_id_1,
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
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_1,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
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
fn prefer_basic_to_account() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Account(
            1_000_000,
            account_id_1,
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
    let outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn take_amount_from_account_to_fund_basic() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Account(
            2_000_000,
            account_id_1,
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
    let outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_account());
            assert_eq!(output.amount(), 1_800_000);
            assert_eq!(output.as_account().native_tokens().len(), 0);
            assert_eq!(*output.as_account().account_id(), account_id_1);
            assert_eq!(output.as_account().unlock_conditions().len(), 2);
            assert_eq!(output.as_account().features().len(), 0);
            assert_eq!(output.as_account().immutable_features().len(), 0);
            assert_eq!(
                *output.as_account().state_controller_address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
            );
            assert_eq!(
                *output.as_account().governor_address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
            );
        }
    });
}

#[test]
fn account_burn_should_not_validate_account_sender() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Basic(2_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Account(
            1_000_000,
            account_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs([Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ACCOUNT_1),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_account(account_id_1))
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()
    ));
}

#[test]
fn account_burn_should_not_validate_account_address() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Basic(2_000_000, BECH32_ADDRESS_ACCOUNT_1, None, None, None, None, None, None),
        Account(
            1_000_000,
            account_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_account(account_id_1))
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Account(account_id, AccountTransition::State))) if account_id == account_id_1
    ));
}

#[test]
fn account_governance_transition_should_not_validate_account_sender() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Basic(2_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Account(
            1_000_000,
            account_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let mut outputs = build_outputs([Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ACCOUNT_1),
        None,
        None,
        None,
        None,
    )]);
    outputs.push(inputs[1].output.clone());

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()
    ));
}

#[test]
fn account_governance_transition_should_not_validate_account_address() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Basic(2_000_000, BECH32_ADDRESS_ACCOUNT_1, None, None, None, None, None, None),
        Account(
            1_000_000,
            account_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let mut outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Account(account_id, AccountTransition::State))) if account_id == account_id_1
    ));
}

#[test]
fn transitioned_zero_account_id_no_longer_is_zero() {
    let protocol_parameters = protocol_parameters();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_0,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_account());
            assert_eq!(output.amount(), 1_000_000);
            assert_eq!(output.as_account().native_tokens().len(), 0);
            assert_ne!(*output.as_account().account_id(), account_id_0);
            assert_eq!(output.as_account().unlock_conditions().len(), 2);
            assert_eq!(output.as_account().features().len(), 0);
            assert_eq!(output.as_account().immutable_features().len(), 0);
            assert_eq!(
                *output.as_account().state_controller_address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
            );
            assert_eq!(
                *output.as_account().governor_address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
            );
        }
    });
}

#[test]
fn two_accounts_required() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs([
        Account(
            2_000_000,
            account_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Account(
            2_000_000,
            account_id_2,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_0]),
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
            .any(|output| if let Output::Account(output) = output {
                output.account_id() == &account_id_1
            } else {
                false
            })
    );
    assert!(
        selected
            .outputs
            .iter()
            .any(|output| if let Output::Account(output) = output {
                output.account_id() == &account_id_2
            } else {
                false
            })
    )
}

#[test]
fn state_controller_sender_required() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_0]),
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
            .any(|output| if let Output::Account(output) = output {
                output.state_index() == inputs[0].output.as_account().state_index() + 1
            } else {
                false
            })
    )
}

#[test]
fn state_controller_sender_required_already_selected() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([
        Account(
            1_000_000,
            account_id_1,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .required_inputs([*inputs[0].output_id()])
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn state_controller_sender_required_but_governance() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([
        Account(
            1_000_000,
            account_id_1,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .required_inputs([*inputs[0].output_id()])
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
    ));
}

#[test]
fn governor_sender_required() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Account(
            2_000_000,
            account_id_1,
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
    let outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1]),
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
            .any(|output| if let Output::Account(output) = output {
                output.state_index() == inputs[0].output.as_account().state_index()
            } else {
                false
            })
    )
}

#[test]
fn governor_sender_required_already_selected() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Account(
            1_000_000,
            account_id_1,
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
    let outputs = build_outputs([
        Account(
            1_000_000,
            account_id_1,
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
        addresses([BECH32_ADDRESS_ED25519_1]),
        protocol_parameters,
    )
    .required_inputs([*inputs[0].output_id()])
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn governance_transition_and_required() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        2_000_000,
        account_id_1,
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
        addresses([BECH32_ADDRESS_ED25519_1]),
        protocol_parameters,
    )
    .required_inputs([*inputs[0].output_id()])
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn state_transition_and_required() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        2_000_000,
        account_id_1,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .required_inputs([*inputs[0].output_id()])
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn governor_sender_required_but_state() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([
        Account(
            1_000_000,
            account_id_1,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .required_inputs([*inputs[0].output_id()])
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn both_state_controller_and_governor_sender() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([
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
        addresses([BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1]),
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
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ACCOUNT_2,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_1,
        1,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ACCOUNT_2,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses([BECH32_ADDRESS_ED25519_0]),
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
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Account(
            1_000_000,
            account_id_1,
            0,
            BECH32_ADDRESS_ACCOUNT_2,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs([Account(
        1_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ACCOUNT_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    // Add the basic output so it will be consumed
    .required_inputs([*inputs[1].output_id()])
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
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_1]),
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
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_1]),
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
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        2_000_000,
        account_id_1,
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
        addresses([BECH32_ADDRESS_ED25519_1]),
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
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Account(
        2_000_000,
        account_id_1,
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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Ed25519(address))) if address == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn burn_account_but_governor_not_owned() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_2]),
        protocol_parameters,
    )
    .burn(Burn::new().add_account(account_id_1))
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Ed25519(address))) if address == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn sender_in_state_controller_but_not_owned() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_2]),
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
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([Account(
        2_000_000,
        account_id_1,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs([Basic(
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
        addresses([BECH32_ADDRESS_ED25519_2]),
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
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let account_output =
        AccountOutputBuilder::new_with_minimum_storage_deposit(*protocol_parameters.rent_structure(), account_id_1)
            .with_state_metadata([1, 2, 3])
            .add_unlock_condition(StateControllerAddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .add_unlock_condition(GovernorAddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output(protocol_parameters.token_supply())
            .unwrap();

    let inputs = [InputSigningData {
        output: account_output.clone(),
        output_metadata: rand_output_metadata(),
        chain: None,
    }];

    // New account output, with updated state index
    let updated_account_output = AccountOutputBuilder::from(account_output.as_account())
        .with_minimum_storage_deposit(*protocol_parameters.rent_structure())
        .with_state_metadata([3, 4, 5])
        .with_state_index(account_output.as_account().state_index() + 1)
        .finish_output(protocol_parameters.token_supply())
        .unwrap();

    let outputs = [updated_account_output];

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses([BECH32_ADDRESS_ED25519_0]),
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
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let account_output =
        AccountOutputBuilder::new_with_minimum_storage_deposit(*protocol_parameters.rent_structure(), account_id_1)
            .with_state_metadata([1, 2, 3])
            .add_unlock_condition(StateControllerAddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .add_unlock_condition(GovernorAddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output(protocol_parameters.token_supply())
            .unwrap();

    let inputs = [InputSigningData {
        output: account_output.clone(),
        output_metadata: rand_output_metadata(),
        chain: None,
    }];

    // New account output, without updated state index
    let updated_account_output = AccountOutputBuilder::from(account_output.as_account())
        .with_minimum_storage_deposit(*protocol_parameters.rent_structure())
        .with_state_metadata([3, 4, 5])
        .finish_output(protocol_parameters.token_supply())
        .unwrap();

    let outputs = [updated_account_output];

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Account(
            account_id,
            _account_transition,
        ))) if account_id == account_id_1
    ));
}
