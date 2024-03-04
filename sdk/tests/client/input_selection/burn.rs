// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use iota_sdk::{
    client::{
        api::input_selection::{Burn, Error, InputSelection, Requirement},
        secret::types::InputSigningData,
    },
    types::block::{
        address::Address,
        output::{
            unlock_condition::AddressUnlockCondition, AccountId, AccountOutputBuilder, BasicOutputBuilder, ChainId,
            NftId, SimpleTokenScheme, TokenId,
        },
        payload::signed_transaction::{TransactionCapabilities, TransactionCapabilityFlag},
        protocol::iota_mainnet_protocol_parameters,
        rand::output::{rand_output_id_with_slot_index, rand_output_metadata_with_id},
        slot::SlotIndex,
    },
};
use pretty_assertions::assert_eq;

use crate::client::{
    assert_remainder_or_return, build_inputs, build_outputs,
    input_selection::native_tokens::nt_remainder_min_storage_deposit,
    unsorted_eq,
    Build::{Account, Basic, Foundry, Nft},
    ACCOUNT_ID_0, ACCOUNT_ID_1, ACCOUNT_ID_2, BECH32_ADDRESS_ED25519_0, NFT_ID_0, NFT_ID_1, NFT_ID_2,
    SLOT_COMMITMENT_ID, SLOT_INDEX, TOKEN_ID_1, TOKEN_ID_2,
};

#[test]
fn burn_account_present() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 1_000_000,
                    account_id: account_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_account(account_id_1))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyAccountOutputs])
    );
    assert_eq!(selected.inputs_data.len(), 1);
    assert_eq!(selected.inputs_data[0], inputs[0]);
    assert_eq!(selected.transaction.outputs(), outputs);
}

#[test]
fn burn_account_present_and_required() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 1_000_000,
                    account_id: account_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_account(account_id_1))
    .with_required_inputs([*inputs[0].output_id()])
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyAccountOutputs])
    );
    assert_eq!(selected.inputs_data.len(), 1);
    assert_eq!(selected.inputs_data[0], inputs[0]);
    assert_eq!(selected.transaction.outputs(), outputs);
}

#[test]
fn burn_account_id_zero() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

    let inputs = build_inputs(
        [
            (
                Nft {
                    amount: 1_000_000,
                    nft_id: nft_id_0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                    sdruc: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);
    let nft_id = NftId::from(inputs[0].output_id());

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_nft(nft_id))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyNftOutputs])
    );
    assert_eq!(selected.inputs_data.len(), 1);
    assert_eq!(selected.inputs_data[0], inputs[0]);
    assert_eq!(selected.transaction.outputs(), outputs);
}

#[test]
fn burn_account_absent() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                native_token: None,
                sender: None,
                sdruc: None,
                timelock: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_account(account_id_1))
    .select()
    .unwrap_err();

    assert_eq!(
        selected,
        Error::UnfulfillableRequirement(Requirement::Account(account_id_1))
    );
}

#[test]
fn burn_accounts_present() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 1_000_000,
                    account_id: account_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
            (
                Account {
                    amount: 1_000_000,
                    account_id: account_id_2,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 3_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().set_accounts(HashSet::from([account_id_1, account_id_2])))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyAccountOutputs])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn burn_account_in_outputs() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 1_000_000,
                    account_id: account_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([
        Account {
            amount: 1_000_000,
            account_id: account_id_1,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            sender: None,
            issuer: None,
        },
        Basic {
            amount: 1_000_000,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            native_token: None,
            sender: None,
            sdruc: None,
            timelock: None,
            expiration: None,
        },
    ]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_account(account_id_1))
    .select()
    .unwrap_err();

    assert_eq!(selected, Error::BurnAndTransition(ChainId::Account(account_id_1)));
}

#[test]
fn burn_nft_present() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Nft {
                    amount: 1_000_000,
                    nft_id: nft_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                    sdruc: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_nft(nft_id_1))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyNftOutputs])
    );
    assert_eq!(selected.inputs_data.len(), 1);
    assert_eq!(selected.inputs_data[0], inputs[0]);
    assert_eq!(selected.transaction.outputs(), outputs);
}

#[test]
fn burn_nft_present_and_required() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Nft {
                    amount: 1_000_000,
                    nft_id: nft_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                    sdruc: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_nft(nft_id_1))
    .with_required_inputs([*inputs[0].output_id()])
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyNftOutputs])
    );
    assert_eq!(selected.inputs_data.len(), 1);
    assert_eq!(selected.inputs_data[0], inputs[0]);
    assert_eq!(selected.transaction.outputs(), outputs);
}

#[test]
fn burn_nft_id_zero() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 1_000_000,
                    account_id: account_id_0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);
    let account_id = AccountId::from(inputs[0].output_id());

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_account(account_id))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyAccountOutputs])
    );
    assert_eq!(selected.inputs_data.len(), 1);
    assert_eq!(selected.inputs_data[0], inputs[0]);
    assert_eq!(selected.transaction.outputs(), outputs);
}

#[test]
fn burn_nft_absent() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                native_token: None,
                sender: None,
                sdruc: None,
                timelock: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_nft(nft_id_1))
    .select()
    .unwrap_err();

    assert_eq!(selected, Error::UnfulfillableRequirement(Requirement::Nft(nft_id_1)));
}

#[test]
fn burn_nfts_present() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(
        [
            (
                Nft {
                    amount: 1_000_000,
                    nft_id: nft_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                    sdruc: None,
                    expiration: None,
                },
                None,
            ),
            (
                Nft {
                    amount: 1_000_000,
                    nft_id: nft_id_2,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                    sdruc: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 3_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().set_nfts(HashSet::from([nft_id_1, nft_id_2])))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyNftOutputs])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn burn_nft_in_outputs() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Nft {
                    amount: 1_000_000,
                    nft_id: nft_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                    sdruc: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([
        Nft {
            amount: 1_000_000,
            nft_id: nft_id_1,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            sender: None,
            issuer: None,
            sdruc: None,
            expiration: None,
        },
        Basic {
            amount: 1_000_000,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            native_token: None,
            sender: None,
            sdruc: None,
            timelock: None,
            expiration: None,
        },
    ]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_nft(nft_id_1))
    .select()
    .unwrap_err();

    assert_eq!(selected, Error::BurnAndTransition(ChainId::Nft(nft_id_1)));
}

#[test]
fn burn_foundry_present() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Foundry {
                    amount: 1_000_000,
                    account_id: account_id_1,
                    serial_number: 1,
                    token_scheme: SimpleTokenScheme::new(0, 0, 10).unwrap(),
                    native_token: None,
                },
                None,
            ),
            (
                Account {
                    amount: 1_000_000,
                    account_id: account_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 500_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_foundry(inputs[0].output.as_foundry().id()))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyFoundryOutputs])
    );
    assert_eq!(selected.inputs_data.len(), 2);
    assert!(selected.inputs_data.contains(&inputs[0]));
    assert!(selected.inputs_data.contains(&inputs[1]));
    assert_eq!(selected.transaction.outputs().len(), 3);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            if output.is_basic() {
                assert_remainder_or_return(
                    output,
                    1_500_000,
                    Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    None,
                );
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
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let foundry_id_1 = build_inputs(
        [(
            Foundry {
                amount: 1_000_000,
                account_id: account_id_1,
                serial_number: 1,
                token_scheme: SimpleTokenScheme::new(0, 0, 10).unwrap(),
                native_token: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    )[0]
    .output
    .as_foundry()
    .id();

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 1_000_000,
                    account_id: account_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_foundry(foundry_id_1))
    .select()
    .unwrap_err();

    assert_eq!(
        selected,
        Error::UnfulfillableRequirement(Requirement::Foundry(foundry_id_1))
    );
}

#[test]
fn burn_foundries_present() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Foundry {
                    amount: 1_000_000,
                    account_id: account_id_1,
                    serial_number: 1,
                    token_scheme: SimpleTokenScheme::new(0, 0, 10).unwrap(),
                    native_token: None,
                },
                None,
            ),
            (
                Foundry {
                    amount: 1_000_000,
                    account_id: account_id_1,
                    serial_number: 2,
                    token_scheme: SimpleTokenScheme::new(0, 0, 10).unwrap(),
                    native_token: None,
                },
                None,
            ),
            (
                Account {
                    amount: 1_000_000,
                    account_id: account_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 2_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().set_foundries(HashSet::from([
        inputs[0].output.as_foundry().id(),
        inputs[1].output.as_foundry().id(),
    ])))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyFoundryOutputs])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
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
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Foundry {
                    amount: 1_000_000,
                    account_id: account_id_1,
                    serial_number: 1,
                    token_scheme: SimpleTokenScheme::new(0, 0, 10).unwrap(),
                    native_token: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([
        Foundry {
            amount: 1_000_000,
            account_id: account_id_1,
            serial_number: 1,
            token_scheme: SimpleTokenScheme::new(0, 0, 10).unwrap(),
            native_token: None,
        },
        Basic {
            amount: 1_000_000,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            native_token: None,
            sender: None,
            sdruc: None,
            timelock: None,
            expiration: None,
        },
    ]);
    let foundry_id_1 = inputs[0].output.as_foundry().id();

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_foundry(foundry_id_1))
    .select()
    .unwrap_err();

    assert_eq!(selected, Error::BurnAndTransition(ChainId::Foundry(foundry_id_1)));
}

#[test]
fn burn_native_tokens() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 1_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: Some((TOKEN_ID_1, 100)),
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: Some((TOKEN_ID_2, 100)),
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );

    let nt_remainder_output_amount = nt_remainder_min_storage_deposit(&protocol_parameters);

    let selected = InputSelection::new(
        inputs.clone(),
        None,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().set_native_tokens(HashMap::from([
        (TokenId::from_str(TOKEN_ID_1).unwrap(), 20),
        (TokenId::from_str(TOKEN_ID_2).unwrap(), 30),
    ])))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::BurnNativeTokens])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);

    assert_remainder_or_return(
        &selected.transaction.outputs()[0],
        nt_remainder_output_amount,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 80)),
    );
    assert_remainder_or_return(
        &selected.transaction.outputs()[1],
        2_000_000 - nt_remainder_output_amount,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_2, 70)),
    );
}

#[test]
fn burn_foundry_and_its_account() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Foundry {
                    amount: 1_000_000,
                    account_id: account_id_1,
                    serial_number: 1,
                    token_scheme: SimpleTokenScheme::new(0, 0, 10).unwrap(),
                    native_token: None,
                },
                None,
            ),
            (
                Account {
                    amount: 1_000_000,
                    account_id: account_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
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
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 500_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(
        Burn::new()
            .add_foundry(inputs[0].output.as_foundry().id())
            .add_account(account_id_1),
    )
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([
            TransactionCapabilityFlag::DestroyAccountOutputs,
            TransactionCapabilityFlag::DestroyFoundryOutputs
        ])
    );
    assert_eq!(selected.inputs_data.len(), 2);
    assert!(selected.inputs_data.contains(&inputs[0]));
    assert!(selected.inputs_data.contains(&inputs[1]));
    // One output should be added for the remainder.
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                1_500_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
            );
        }
    });
}

#[test]
fn burn_mana() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = [BasicOutputBuilder::new_with_amount(1_000_000)
        .with_mana(1000)
        .add_unlock_condition(AddressUnlockCondition::new(
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        ))
        .finish_output()
        .unwrap()];
    let inputs = inputs
        .into_iter()
        .map(|input| InputSigningData {
            output: input,
            output_metadata: rand_output_metadata_with_id(rand_output_id_with_slot_index(SLOT_INDEX)),
            chain: None,
        })
        .collect::<Vec<_>>();

    let outputs = [BasicOutputBuilder::new_with_amount(1_000_000)
        .add_unlock_condition(AddressUnlockCondition::new(
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        ))
        .with_mana(500)
        .finish_output()
        .unwrap()];

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_required_inputs([*inputs[0].output_id()])
    .with_burn(Burn::new().set_mana(true))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::BurnMana])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs(), &outputs);
}

#[test]
fn burn_mana_need_additional() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = [
        BasicOutputBuilder::new_with_amount(100_000)
            .with_mana(1000)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
        BasicOutputBuilder::new_with_amount(1_000_000)
            .with_mana(200)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
    ];
    let inputs = inputs
        .into_iter()
        .map(|input| InputSigningData {
            output: input,
            output_metadata: rand_output_metadata_with_id(rand_output_id_with_slot_index(SLOT_INDEX)),
            chain: None,
        })
        .collect::<Vec<_>>();

    let outputs = [BasicOutputBuilder::new_with_amount(1_100_000)
        .add_unlock_condition(AddressUnlockCondition::new(
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        ))
        .with_mana(500)
        .finish_output()
        .unwrap()];

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_required_inputs([*inputs[0].output_id()])
    .with_burn(Burn::new().set_mana(true))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::BurnMana])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 1);
    assert_eq!(selected.transaction.outputs()[0].mana(), 700);
}

#[test]
fn burn_mana_need_additional_account() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = [
        BasicOutputBuilder::new_with_amount(100_000)
            .with_mana(1000)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
        AccountOutputBuilder::new_with_amount(1_200_000, account_id_1)
            .with_mana(200)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
    ];
    let inputs = inputs
        .into_iter()
        .map(|input| InputSigningData {
            output: input,
            output_metadata: rand_output_metadata_with_id(rand_output_id_with_slot_index(SLOT_INDEX)),
            chain: None,
        })
        .collect::<Vec<_>>();

    let outputs = [BasicOutputBuilder::new_with_amount(1_100_000)
        .add_unlock_condition(AddressUnlockCondition::new(
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        ))
        .with_mana(500)
        .finish_output()
        .unwrap()];

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_required_inputs([*inputs[0].output_id()])
    .with_burn(Burn::new().set_mana(true))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::BurnMana])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert_eq!(selected.transaction.outputs()[0].mana(), 500);
    assert_eq!(selected.transaction.outputs()[1].mana(), 200);
}

#[test]
fn burn_generated_mana() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = [
        BasicOutputBuilder::new_with_amount(1_000_000)
            .with_mana(1000)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
        BasicOutputBuilder::new_with_amount(1_000_000)
            .with_mana(200)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
    ];
    let inputs = inputs
        .into_iter()
        .map(|input| InputSigningData {
            output: input,
            output_metadata: rand_output_metadata_with_id(rand_output_id_with_slot_index(SlotIndex(5))),
            chain: None,
        })
        .collect::<Vec<_>>();

    let outputs = [BasicOutputBuilder::new_with_amount(2_000_000)
        .add_unlock_condition(AddressUnlockCondition::new(
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        ))
        .with_mana(1200)
        .finish_output()
        .unwrap()];

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().set_generated_mana(true))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::BurnMana])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn burn_generated_mana_remainder() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = [
        BasicOutputBuilder::new_with_amount(1_000_000)
            .with_mana(1000)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
        BasicOutputBuilder::new_with_amount(1_000_000)
            .with_mana(200)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
    ];
    let inputs = inputs
        .into_iter()
        .map(|input| InputSigningData {
            output: input,
            output_metadata: rand_output_metadata_with_id(rand_output_id_with_slot_index(SlotIndex(5))),
            chain: None,
        })
        .collect::<Vec<_>>();

    let selected = InputSelection::new(
        inputs.clone(),
        None,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().set_generated_mana(true))
    .with_required_inputs(inputs.iter().map(|i| *i.output_id()))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::BurnMana])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 1);
    assert_eq!(selected.transaction.outputs()[0].mana(), 1200);
}

#[test]
fn burn_generated_mana_account() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = [
        BasicOutputBuilder::new_with_amount(1_000_000)
            .with_mana(1000)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
        AccountOutputBuilder::new_with_amount(1_000_000, account_id_1)
            .with_mana(200)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap(),
    ];
    let inputs = inputs
        .into_iter()
        .map(|input| InputSigningData {
            output: input,
            output_metadata: rand_output_metadata_with_id(rand_output_id_with_slot_index(SlotIndex(5))),
            chain: None,
        })
        .collect::<Vec<_>>();

    let selected = InputSelection::new(
        inputs.clone(),
        None,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().set_generated_mana(true))
    .with_required_inputs(inputs.iter().map(|i| *i.output_id()))
    .select()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::BurnMana])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert_eq!(
        selected.transaction.outputs().iter().map(|o| o.mana()).sum::<u64>(),
        1200
    );
}
