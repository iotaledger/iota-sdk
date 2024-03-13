// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::{
    client::api::transaction_builder::{TransactionBuilder, TransactionBuilderError},
    types::block::{address::Address, output::AccountId, protocol::iota_mainnet_protocol_parameters},
};
use pretty_assertions::assert_eq;

use crate::client::{
    assert_remainder_or_return, build_inputs, build_outputs, unsorted_eq,
    Build::{Account, Basic},
    ACCOUNT_ID_1, BECH32_ADDRESS_ACCOUNT_1, BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1,
    BECH32_ADDRESS_ED25519_2, SLOT_COMMITMENT_ID, SLOT_INDEX,
};

#[test]
fn sdruc_output_not_provided_no_remainder() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000,
                mana: 0,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                native_token: None,
                sender: None,
                sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                timelock: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None,
            );
        }
    });
}

#[test]
fn sdruc_output_provided_no_remainder() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000,
                mana: 0,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                native_token: None,
                sender: None,
                sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                timelock: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([
        Basic {
            amount: 1_000_000,
            mana: 0,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            native_token: None,
            sender: None,
            sdruc: None,
            timelock: None,
            expiration: None,
        },
        Basic {
            amount: 1_000_000,
            mana: 0,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
            native_token: None,
            sender: None,
            sdruc: None,
            timelock: None,
            expiration: None,
        },
    ]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

#[test]
fn sdruc_output_provided_remainder() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000,
                mana: 0,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                native_token: None,
                sender: None,
                sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                timelock: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
            );
        }
    });
}

#[test]
fn two_sdrucs_to_the_same_address_both_needed() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 2_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None,
            );
        }
    });
}

#[test]
fn two_sdrucs_to_the_same_address_one_needed() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
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
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert_eq!(selected.inputs_data.len(), 1);
    assert!(selected.inputs_data.contains(&inputs[0]));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None,
            );
        }
    });
}

#[test]
fn two_sdrucs_to_different_addresses_both_needed() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(), 1_000_000)),
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 2_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 3);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    assert_remainder_or_return(
        selected
            .transaction
            .outputs()
            .iter()
            .find(|o| o.as_basic().address() == &Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap())
            .unwrap(),
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        None,
    );
    assert_remainder_or_return(
        selected
            .transaction
            .outputs()
            .iter()
            .find(|o| o.as_basic().address() == &Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap())
            .unwrap(),
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
        None,
    );
}

#[test]
fn two_sdrucs_to_different_addresses_one_needed() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(), 1_000_000)),
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
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert_eq!(selected.inputs_data.len(), 1);
    assert!(selected.inputs_data.contains(&inputs[0]));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None,
            );
        }
    });
}

#[test]
fn insufficient_amount_because_of_sdruc() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000,
                mana: 0,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                native_token: None,
                sender: None,
                sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
                timelock: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 2_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish();

    assert!(matches!(
        selected,
        Err(TransactionBuilderError::InsufficientAmount {
            found: 2_000_000,
            required: 3_000_000,
        })
    ));
}

#[test]
fn useless_sdruc_required_for_sender_feature() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(), 1_000_000)),
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
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
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()),
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        ],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
                None,
            );
        }
    });
}

#[test]
fn sdruc_required_non_ed25519_in_address_unlock() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(), 1_000_000)),
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
            (
                Account {
                    amount: 1_000_000,
                    mana: 0,
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
        amount: 1_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
        native_token: None,
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 3);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) && !output.is_account() {
            assert_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
                None,
            );
        }
    });
}

#[test]
fn useless_sdruc_non_ed25519_in_address_unlock() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(), 1_000_000)),
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
            (
                Account {
                    amount: 1_000_000,
                    mana: 0,
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
        amount: 1_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert_eq!(selected.inputs_data.len(), 2);
    assert!(selected.inputs_data.contains(&inputs[1]));
    assert!(selected.inputs_data.contains(&inputs[2]));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_account());
        }
    });
}
