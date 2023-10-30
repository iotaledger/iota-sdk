// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use iota_sdk::{
    client::api::input_selection::{Burn, Error, InputSelection, Requirement},
    types::block::{
        address::Address,
        output::{AccountId, ChainId, NftId, SimpleTokenScheme, TokenId},
        protocol::protocol_parameters,
    },
};
use pretty_assertions::assert_eq;

use crate::client::{
    addresses, build_inputs, build_outputs, is_remainder_or_return, unsorted_eq,
    Build::{Account, Basic, Foundry, Nft},
    ACCOUNT_ID_0, ACCOUNT_ID_1, ACCOUNT_ID_2, BECH32_ADDRESS_ED25519_0, NFT_ID_0, NFT_ID_1, NFT_ID_2, TOKEN_ID_1,
    TOKEN_ID_2,
};

#[test]
fn burn_account_present() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Account(1_000_000, account_id_1, BECH32_ADDRESS_ED25519_0, None, None, None),
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
    .with_burn(Burn::new().add_account(account_id_1))
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[0]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn burn_account_present_and_required() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Account(1_000_000, account_id_1, BECH32_ADDRESS_ED25519_0, None, None, None),
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
    .with_burn(Burn::new().add_account(account_id_1))
    .with_required_inputs([*inputs[0].output_id()])
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[0]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn burn_account_id_zero() {
    let protocol_parameters = protocol_parameters();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

    let inputs = build_inputs([
        Nft(
            1_000_000,
            nft_id_0,
            BECH32_ADDRESS_ED25519_0,
            None,
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
    let nft_id = NftId::from(inputs[0].output_id());

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .with_burn(Burn::new().add_nft(nft_id))
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[0]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn burn_account_absent() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .with_burn(Burn::new().add_account(account_id_1))
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Account(account_id))) if account_id == account_id_1
    ));
}

#[test]
fn burn_accounts_present() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs([
        Account(1_000_000, account_id_1, BECH32_ADDRESS_ED25519_0, None, None, None),
        Account(1_000_000, account_id_2, BECH32_ADDRESS_ED25519_0, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
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
    .with_burn(Burn::new().set_accounts(HashSet::from([account_id_1, account_id_2])))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn burn_account_in_outputs() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Account(1_000_000, account_id_1, BECH32_ADDRESS_ED25519_0, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs([
        Account(1_000_000, account_id_1, BECH32_ADDRESS_ED25519_0, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .with_burn(Burn::new().add_account(account_id_1))
    .select();

    assert!(matches!(
        selected,
        Err(Error::BurnAndTransition(ChainId::Account(account_id))) if account_id == account_id_1
    ));
}

#[test]
fn burn_nft_present() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs([
        Nft(
            1_000_000,
            nft_id_1,
            BECH32_ADDRESS_ED25519_0,
            None,
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
    .with_burn(Burn::new().add_nft(nft_id_1))
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[0]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn burn_nft_present_and_required() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs([
        Nft(
            1_000_000,
            nft_id_1,
            BECH32_ADDRESS_ED25519_0,
            None,
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
    .with_burn(Burn::new().add_nft(nft_id_1))
    .with_required_inputs([*inputs[0].output_id()])
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[0]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn burn_nft_id_zero() {
    let protocol_parameters = protocol_parameters();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs([
        Account(1_000_000, account_id_0, BECH32_ADDRESS_ED25519_0, None, None, None),
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
    let account_id = AccountId::from(inputs[0].output_id());

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .with_burn(Burn::new().add_account(account_id))
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[0]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn burn_nft_absent() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

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
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .with_burn(Burn::new().add_nft(nft_id_1))
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Nft(nft_id))) if nft_id == nft_id_1
    ));
}

#[test]
fn burn_nfts_present() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs([
        Nft(
            1_000_000,
            nft_id_1,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
            None,
        ),
        Nft(
            1_000_000,
            nft_id_2,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
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
    .with_burn(Burn::new().set_nfts(HashSet::from([nft_id_1, nft_id_2])))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn burn_nft_in_outputs() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs([
        Nft(
            1_000_000,
            nft_id_1,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs([
        Nft(
            1_000_000,
            nft_id_1,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .with_burn(Burn::new().add_nft(nft_id_1))
    .select();

    assert!(matches!(
        selected,
        Err(Error::BurnAndTransition(ChainId::Nft(nft_id))) if nft_id == nft_id_1
    ));
}

#[test]
fn burn_foundry_present() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Foundry(
            1_000_000,
            account_id_1,
            1,
            SimpleTokenScheme::new(0, 0, 10).unwrap(),
            None,
        ),
        Account(1_000_000, account_id_1, BECH32_ADDRESS_ED25519_0, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs([Basic(
        500_000,
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
    .with_burn(Burn::new().add_foundry(inputs[0].output.as_foundry().id()))
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 2);
    assert!(selected.inputs.contains(&inputs[0]));
    assert!(selected.inputs.contains(&inputs[1]));
    assert_eq!(selected.outputs.len(), 3);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            if output.is_basic() {
                assert!(is_remainder_or_return(
                    output,
                    1_500_000,
                    BECH32_ADDRESS_ED25519_0,
                    None,
                ));
            } else if output.is_account() {
                assert_eq!(output.amount(), 1_000_000);
                assert_eq!(*output.as_account().account_id(), account_id_1);
                assert_eq!(output.as_account().unlock_conditions().len(), 1);
                assert_eq!(output.as_account().features().len(), 0);
                assert_eq!(output.as_account().immutable_features().len(), 0);
                assert_eq!(
                    *output.as_account().address(),
                    Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
                );
            } else {
                panic!("unexpected output type")
            }
        }
    });
}

#[test]
fn burn_foundry_absent() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let foundry_id_1 = build_inputs([Foundry(
        1_000_000,
        account_id_1,
        1,
        SimpleTokenScheme::new(0, 0, 10).unwrap(),
        None,
    )])[0]
        .output
        .as_foundry()
        .id();

    let inputs = build_inputs([
        Account(1_000_000, account_id_1, BECH32_ADDRESS_ED25519_0, None, None, None),
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
        inputs,
        outputs,
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .with_burn(Burn::new().add_foundry(foundry_id_1))
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Foundry(foundry_id))) if foundry_id == foundry_id_1
    ));
}

#[test]
fn burn_foundries_present() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Foundry(
            1_000_000,
            account_id_1,
            1,
            SimpleTokenScheme::new(0, 0, 10).unwrap(),
            None,
        ),
        Foundry(
            1_000_000,
            account_id_1,
            2,
            SimpleTokenScheme::new(0, 0, 10).unwrap(),
            None,
        ),
        Account(1_000_000, account_id_1, BECH32_ADDRESS_ED25519_0, None, None, None),
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
        inputs.clone(),
        outputs.clone(),
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .with_burn(Burn::new().set_foundries(HashSet::from([
        inputs[0].output.as_foundry().id(),
        inputs[1].output.as_foundry().id(),
    ])))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_account());
            assert_eq!(output.amount(), 1_000_000);
            assert_eq!(*output.as_account().account_id(), account_id_1);
            assert_eq!(output.as_account().unlock_conditions().len(), 1);
            assert_eq!(output.as_account().features().len(), 0);
            assert_eq!(output.as_account().immutable_features().len(), 0);
            assert_eq!(
                *output.as_account().address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()
            );
        }
    });
}

#[test]
fn burn_foundry_in_outputs() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Foundry(
            1_000_000,
            account_id_1,
            1,
            SimpleTokenScheme::new(0, 0, 10).unwrap(),
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs([
        Foundry(
            1_000_000,
            account_id_1,
            1,
            SimpleTokenScheme::new(0, 0, 10).unwrap(),
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let foundry_id_1 = inputs[0].output.as_foundry().id();

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .with_burn(Burn::new().add_foundry(foundry_id_1))
    .select();

    assert!(matches!(
        selected,
        Err(Error::BurnAndTransition(ChainId::Foundry(foundry_id))) if foundry_id == foundry_id_1
    ));
}

#[test]
fn burn_native_tokens() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs([Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100), (TOKEN_ID_2, 100)]),
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        Vec::new(),
        addresses([BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .with_burn(Burn::new().set_native_tokens(HashMap::from([
        (TokenId::from_str(TOKEN_ID_1).unwrap(), 20),
        (TokenId::from_str(TOKEN_ID_2).unwrap(), 30),
    ])))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 1);
    assert!(is_remainder_or_return(
        &selected.outputs[0],
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 80), (TOKEN_ID_2, 70)])
    ));
}

#[test]
fn burn_foundry_and_its_account() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs([
        Foundry(
            1_000_000,
            account_id_1,
            1,
            SimpleTokenScheme::new(0, 0, 10).unwrap(),
            None,
        ),
        Account(1_000_000, account_id_1, BECH32_ADDRESS_ED25519_0, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs([Basic(
        500_000,
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
    .with_burn(
        Burn::new()
            .add_foundry(inputs[0].output.as_foundry().id())
            .add_account(account_id_1),
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 2);
    assert!(selected.inputs.contains(&inputs[0]));
    assert!(selected.inputs.contains(&inputs[1]));
    // One output should be added for the remainder.
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_500_000,
                BECH32_ADDRESS_ED25519_0,
                None,
            ));
        }
    });
}
