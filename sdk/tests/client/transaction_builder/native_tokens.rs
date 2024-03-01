// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::{
    client::api::transaction_builder::{Burn, TransactionBuilder, TransactionBuilderError},
    types::block::{
        address::Address,
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NativeToken, TokenId},
        payload::signed_transaction::{TransactionCapabilities, TransactionCapabilityFlag},
        protocol::{iota_mainnet_protocol_parameters, ProtocolParameters},
    },
};
use pretty_assertions::assert_eq;
use primitive_types::U256;

use crate::client::{
    assert_remainder_or_return, build_inputs, build_outputs, unsorted_eq, Build::Basic, BECH32_ADDRESS_ED25519_0,
    SLOT_COMMITMENT_ID, SLOT_INDEX, TOKEN_ID_1, TOKEN_ID_2,
};

#[test]
fn two_native_tokens_one_needed() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 1_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: Some((TOKEN_ID_1, 150)),
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
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: Some((TOKEN_ID_1, 150)),
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

    assert_eq!(selected.inputs_data[0], inputs[0]);
    assert_eq!(selected.transaction.outputs().len(), 1);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
}

#[test]
fn two_native_tokens_both_needed_plus_remainder() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
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
                    amount: 2_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: Some((TOKEN_ID_2, 150)),
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
        Basic {
            amount: 1_000_000,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            native_token: Some((TOKEN_ID_1, 100)),
            sender: None,
            sdruc: None,
            timelock: None,
            expiration: None,
        },
        Basic {
            amount: 1_000_000,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            native_token: Some((TOKEN_ID_2, 100)),
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
    assert_eq!(selected.transaction.outputs().len(), 3);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_2, 50)),
            );
        }
    });
}

#[test]
fn three_inputs_two_needed_plus_remainder() {
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
                    native_token: Some((TOKEN_ID_1, 100)),
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
        native_token: Some((TOKEN_ID_1, 120)),
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs,
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert_eq!(selected.inputs_data.len(), 2);
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 80)),
            );
        }
    });
}

#[test]
fn three_inputs_two_needed_no_remainder() {
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
                    native_token: Some((TOKEN_ID_1, 100)),
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
        amount: 2_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: Some((TOKEN_ID_1, 200)),
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs,
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert_eq!(selected.inputs_data.len(), 2);
    assert_eq!(selected.transaction.outputs(), outputs);
}

#[test]
fn insufficient_native_tokens_one_input() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
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
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: Some((TOKEN_ID_1, 150)),
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
        Err(TransactionBuilderError::InsufficientNativeTokenAmount {
            token_id,
            found,
            required,
        }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(100) && required == U256::from(150)));
}

#[test]
fn insufficient_native_tokens_three_inputs() {
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
                    native_token: Some((TOKEN_ID_1, 100)),
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
        native_token: Some((TOKEN_ID_1, 301)),
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
        Err(TransactionBuilderError::InsufficientNativeTokenAmount {
            token_id,
            found,
            required,
        }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(300) && required == U256::from(301)));
}

#[test]
fn burn_and_send_at_the_same_time() {
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
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: Some((TOKEN_ID_1, 50)),
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
    .with_burn(
        Burn::new()
            .add_native_token(TokenId::from_str(TOKEN_ID_1).unwrap(), 10)
            .add_native_token(TokenId::from_str(TOKEN_ID_2).unwrap(), 100),
    )
    .finish()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::BurnNativeTokens])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                Some((TOKEN_ID_1, 40)),
            );
        }
    });
}

#[test]
fn burn_one_input_no_output() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
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
        )],
        Some(SLOT_INDEX),
    );

    let selected = TransactionBuilder::new(
        inputs.clone(),
        None,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_native_token(TokenId::from_str(TOKEN_ID_1).unwrap(), 50))
    .finish()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::BurnNativeTokens])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 1);
    assert_remainder_or_return(
        &selected.transaction.outputs()[0],
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 50)),
    );
}

#[test]
fn multiple_native_tokens() {
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
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: Some((TOKEN_ID_1, 100)),
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
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn insufficient_native_tokens() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

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
        native_token: Some((TOKEN_ID_1, 150)),
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
        Err(TransactionBuilderError::InsufficientNativeTokenAmount {
            token_id,
            found,
            required,
        }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(0) && required == U256::from(150)));
}

#[test]
fn insufficient_native_tokens_2() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
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
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: Some((TOKEN_ID_1, 150)),
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
        Err(TransactionBuilderError::InsufficientNativeTokenAmount {
            token_id,
            found,
            required,
        }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(100) && required == U256::from(150)));
}

#[test]
fn insufficient_amount_for_remainder() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
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
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: Some((TOKEN_ID_1, 50)),
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let nt_remainder_min_storage_deposit = nt_remainder_min_storage_deposit(&protocol_parameters);

    let selected = TransactionBuilder::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish();

    assert_eq!(
        selected.unwrap_err(),
        TransactionBuilderError::InsufficientAmount {
            found: 1_000_000,
            required: 1_000_000 + nt_remainder_min_storage_deposit,
        }
    );
}

#[test]
fn single_output_native_token_no_remainder() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
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
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: Some((TOKEN_ID_1, 100)),
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
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn single_output_native_token_remainder_1() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
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
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 500_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: Some((TOKEN_ID_1, 50)),
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
    assert_remainder_or_return(
        &selected.transaction.outputs()[0],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 50)),
    );
}

#[test]
fn single_output_native_token_remainder_2() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
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
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 500_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: Some((TOKEN_ID_1, 100)),
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
    assert_remainder_or_return(
        &selected.transaction.outputs()[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
    );
}

#[test]
fn two_basic_outputs_1() {
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
                    native_token: Some((TOKEN_ID_1, 200)),
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
    assert_remainder_or_return(
        &selected.transaction.outputs()[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 100)),
    );
}

#[test]
fn two_basic_outputs_2() {
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
                    native_token: Some((TOKEN_ID_1, 200)),
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
        native_token: Some((TOKEN_ID_1, 50)),
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
    assert_remainder_or_return(
        &selected.transaction.outputs()[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 50)),
    );
}

#[test]
fn two_basic_outputs_3() {
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
                    native_token: Some((TOKEN_ID_1, 200)),
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
        native_token: Some((TOKEN_ID_1, 75)),
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
    assert_remainder_or_return(
        &selected.transaction.outputs()[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 25)),
    );
}

#[test]
fn two_basic_outputs_4() {
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
                    native_token: Some((TOKEN_ID_1, 200)),
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
        native_token: Some((TOKEN_ID_1, 100)),
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
    assert_remainder_or_return(
        &selected.transaction.outputs()[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
    );
}

#[test]
fn two_basic_outputs_5() {
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
                    native_token: Some((TOKEN_ID_1, 200)),
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
        native_token: Some((TOKEN_ID_1, 100)),
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
    assert_remainder_or_return(
        &selected.transaction.outputs()[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
    );
}

#[test]
fn two_basic_outputs_6() {
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
                    native_token: Some((TOKEN_ID_1, 200)),
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
        native_token: Some((TOKEN_ID_1, 250)),
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
    assert_remainder_or_return(
        &selected.transaction.outputs()[1],
        1_500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        Some((TOKEN_ID_1, 50)),
    );
}

#[test]
fn two_basic_outputs_7() {
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
                    native_token: Some((TOKEN_ID_1, 200)),
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
        native_token: Some((TOKEN_ID_1, 300)),
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
    assert_remainder_or_return(
        &selected.transaction.outputs()[1],
        1_500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
    );
}

#[test]
fn two_basic_outputs_8() {
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
                    native_token: Some((TOKEN_ID_1, 200)),
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
        native_token: Some((TOKEN_ID_1, 350)),
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
            Err(TransactionBuilderError::InsufficientNativeTokenAmount {
                token_id,
                found,
                required,
            }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(300) && required == U256::from(350)));
}

#[test]
fn two_basic_outputs_native_tokens_not_needed() {
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
    assert!(selected.inputs_data.contains(&inputs[1]));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    assert_remainder_or_return(
        &selected.transaction.outputs()[1],
        500_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
    );
}

#[test]
fn multiple_remainders() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 5_000_000,
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
                    amount: 5_000_000,
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
                    amount: 5_000_000,
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
                    amount: 5_000_000,
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
    let outputs = build_outputs([Basic {
        amount: 15_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let nt_remainder_min_storage_deposit = nt_remainder_min_storage_deposit(&protocol_parameters);

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

    assert_eq!(selected.inputs_data.len(), 4);
    assert_eq!(selected.transaction.outputs().len(), 3);
    assert!(selected.transaction.outputs().contains(&outputs[0]));

    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            if output.native_token().unwrap().token_id().to_string() == TOKEN_ID_1 {
                assert_remainder_or_return(
                    output,
                    nt_remainder_min_storage_deposit,
                    Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    Some((TOKEN_ID_1, 300)),
                );
            } else {
                assert_remainder_or_return(
                    output,
                    5_000_000 - nt_remainder_min_storage_deposit,
                    Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    Some((TOKEN_ID_2, 100)),
                );
            }
        }
    });
}

pub fn nt_remainder_min_storage_deposit(protocol_parameters: &ProtocolParameters) -> u64 {
    BasicOutputBuilder::new_with_minimum_amount(protocol_parameters.storage_score_parameters())
        .add_unlock_condition(AddressUnlockCondition::from(
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        ))
        .with_native_token(NativeToken::new(TokenId::from_str(TOKEN_ID_1).unwrap(), 1).unwrap())
        .finish_output()
        .unwrap()
        .amount()
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

//     let selected = TransactionBuilder::new(
//         inputs.clone(),
//         outputs.clone(),
//         addresses([BECH32_ADDRESS_ED25519_0]),
//         protocol_parameters,
//     )
//     .select()
//     .unwrap();

//     assert_eq!(selected.inputs_data.len(), 1);
//     assert!(selected.inputs_data.contains(&inputs[1]));
//     assert_eq!(selected.transaction.outputs().len(), 2);
//     assert!(selected.transaction.outputs().contains(&outputs[0]));
//     assert!(is_remainder_or_return(
//         &selected.transaction.outputs()[1],
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
