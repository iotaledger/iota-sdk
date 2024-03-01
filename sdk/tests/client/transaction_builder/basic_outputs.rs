// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::{
    client::{
        api::transaction_builder::{Requirement, TransactionBuilder, TransactionBuilderError},
        secret::types::InputSigningData,
    },
    types::block::{
        address::{Address, AddressCapabilities, MultiAddress, RestrictedAddress, WeightedAddress},
        mana::ManaAllotment,
        output::{unlock_condition::AddressUnlockCondition, AccountId, BasicOutputBuilder, NftId},
        protocol::iota_mainnet_protocol_parameters,
        rand::output::{rand_output_id_with_slot_index, rand_output_metadata_with_id},
    },
};
use pretty_assertions::assert_eq;

use crate::client::{
    assert_remainder_or_return, build_inputs, build_outputs, unsorted_eq,
    Build::{Account, Basic, Nft},
    ACCOUNT_ID_0, ACCOUNT_ID_1, BECH32_ADDRESS_ACCOUNT_1, BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1,
    BECH32_ADDRESS_ED25519_2, BECH32_ADDRESS_NFT_1, BECH32_ADDRESS_REMAINDER, NFT_ID_0, NFT_ID_1, SLOT_COMMITMENT_ID,
    SLOT_INDEX,
};

#[test]
fn input_amount_equal_output_amount() {
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
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn input_amount_lower_than_output_amount() {
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
        amount: 2_000_000,
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
            found: 1_000_000,
            required: 2_000_000,
        })
    ));
}

#[test]
fn input_amount_lower_than_output_amount_2() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
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
            (
                Basic {
                    amount: 2_000_000,
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
        amount: 3_500_000,
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
            found: 3_000_000,
            required: 3_500_000,
        })
    ));
}

#[test]
fn input_amount_greater_than_output_amount() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000,
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

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
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
fn input_amount_greater_than_output_amount_with_remainder_address() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let remainder_address = Address::try_from_bech32(BECH32_ADDRESS_REMAINDER).unwrap();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000,
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
    .with_remainder_address(remainder_address)
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    // One output should be added for the remainder.
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                1_500_000,
                Address::try_from_bech32(BECH32_ADDRESS_REMAINDER).unwrap(),
                None,
            );
        }
    });
}

#[test]
fn two_same_inputs_one_needed() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
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
        inputs,
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    // One input has enough amount.
    assert_eq!(selected.inputs_data.len(), 1);
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
fn two_inputs_one_needed() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
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
            (
                Basic {
                    amount: 2_000_000,
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

    assert_eq!(selected.inputs_data, [inputs[0].clone()]);
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn two_inputs_one_needed_reversed() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
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
        amount: 1_000_000,
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

    assert_eq!(selected.inputs_data, [inputs[1].clone()]);
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn two_inputs_both_needed() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
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
            (
                Basic {
                    amount: 2_000_000,
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
fn two_inputs_remainder() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
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
            (
                Basic {
                    amount: 2_000_000,
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
        amount: 2_500_000,
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
    // One output should be added for the remainder.
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                500_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
            );
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

//     let selected = TransactionBuilder::new(
//         inputs,
//         outputs,
//         [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
//         SLOT_INDEX+1,protocol_parameters,
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
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let sender = Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
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
                    native_token: None,
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
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                    native_token: None,
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
                    native_token: None,
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
        amount: 2_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs,
        outputs,
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

    // Sender + another for amount
    assert_eq!(selected.inputs_data.len(), 2);
    assert!(
        selected
            .inputs_data
            .iter()
            .any(|input| *input.output.as_basic().address() == sender)
    );
    // Provided output + remainder
    assert_eq!(selected.transaction.outputs().len(), 2);
}

#[test]
fn missing_ed25519_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 5_000_000,
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
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
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
        Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn account_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
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
                    account_id: account_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
            (
                Basic {
                    amount: 2_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
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
        amount: 2_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
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

    // Sender + another for amount
    assert_eq!(selected.inputs_data.len(), 2);
    assert!(
        selected
            .inputs_data
            .iter()
            .any(|input| input.output.is_account() && *input.output.as_account().account_id() == account_id_1)
    );
    // Provided output + account
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
}

#[test]
fn account_sender_zero_id() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
                    account_id: account_id_0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );
    let account_id = AccountId::from(inputs[1].output_id());
    let outputs = build_outputs([Basic {
        amount: 2_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: Some(Address::from(account_id)),
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(
        selected
            .transaction
            .outputs()
            .iter()
            .any(|output| output.is_account() && *output.as_account().account_id() == account_id)
    );
}

#[test]
fn missing_account_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 5_000_000,
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
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
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
        Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()
    ));
}

#[test]
fn nft_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
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
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
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
                    amount: 2_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
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
        amount: 2_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()),
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

    // Sender + another for amount
    assert_eq!(selected.inputs_data.len(), 2);
    assert!(
        selected
            .inputs_data
            .iter()
            .any(|input| input.output.is_nft() && *input.output.as_nft().nft_id() == nft_id_1)
    );
    // Provided output + nft
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&inputs[2].output));
    assert!(selected.transaction.outputs().contains(&outputs[0]));
}

#[test]
fn nft_sender_zero_id() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
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
        ],
        Some(SLOT_INDEX),
    );
    let nft_id = NftId::from(inputs[1].output_id());
    let outputs = build_outputs([Basic {
        amount: 2_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: Some(Address::from(nft_id)),
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(
        selected
            .transaction
            .outputs()
            .iter()
            .any(|output| output.is_nft() && *output.as_nft().nft_id() == nft_id)
    );
}

#[test]
fn missing_nft_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 5_000_000,
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
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()),
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
        Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Sender(sender))) if sender == Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()
    ));
}

#[test]
fn simple_remainder() {
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

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                500_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
            );
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

//     let selected = TransactionBuilder::new(
//         inputs,
//         outputs,
//         [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
//         SLOT_INDEX+1,protocol_parameters,
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

//     let selected = TransactionBuilder::new(
//         inputs,
//         outputs,
//         [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
//         SLOT_INDEX+1,protocol_parameters,
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
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn insufficient_amount() {
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
        amount: 1_250_000,
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
            found: 1_000_000,
            required: 1_250_000,
        })
    ));
}

#[test]
fn two_inputs_remainder_2() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
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
            (
                Basic {
                    amount: 2_000_000,
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
    assert!(selected.inputs_data.contains(&inputs[0]));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                500_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
            );
        }
    });
}

#[test]
fn two_inputs_remainder_3() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [
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
            (
                Basic {
                    amount: 2_000_000,
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
        amount: 1_750_000,
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
                1_250_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
            );
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

//     let selected = TransactionBuilder::new(
//         inputs.clone(),
//         outputs.clone(),
//         [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
//         SLOT_INDEX+1,protocol_parameters,
//     )
//     .select()
//     .unwrap();

//     assert!(unsorted_eq(&selected.inputs_data, &inputs));
//     assert_eq!(selected.transaction.outputs().len(), 2);
//     assert!(selected.transaction.outputs().contains(&outputs[0]));
//     selected.transaction.outputs().iter().for_each(|output| {
//         if !outputs.contains(output) {
//             assert!(is_remainder_or_return(output, 800_000,
// Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), None));         }
//     });
// }

#[test]
fn sender_already_selected() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
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
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
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
    .with_required_inputs([*inputs[0].output_id()])
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn single_mandatory_input() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
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
    .with_required_inputs([*inputs[0].output_id()])
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn too_many_inputs() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    // 129 inputs that would be required for the amount, but that's above max inputs
    let inputs = build_inputs(
        std::iter::repeat_with(|| {
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
            )
        })
        .take(129),
        None,
    );
    let outputs = build_outputs([Basic {
        amount: 129_000_000,
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

    assert_eq!(
        selected.unwrap_err(),
        iota_sdk::client::api::transaction_builder::TransactionBuilderError::InvalidInputCount(129)
    )
}

#[test]
fn more_than_max_inputs_only_one_needed() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    // 1000 inputs where 129 would be needed for the required amount which is above the max inputs
    let mut inputs = build_inputs(
        std::iter::repeat_with(|| {
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
            )
        })
        .take(1000),
        None,
    );
    // Add the needed input
    let needed_input = build_inputs(
        [(
            Basic {
                amount: 129_000_000,
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
    inputs.push(needed_input[0].clone());

    let outputs = build_outputs([Basic {
        amount: 129_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
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

    assert!(unsorted_eq(&selected.inputs_data, &needed_input));
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn too_many_outputs() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000_000,
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
    let outputs = build_outputs(
        std::iter::repeat_with(|| Basic {
            amount: 1_000_000,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            native_token: None,
            sender: None,
            sdruc: None,
            timelock: None,
            expiration: None,
        })
        .take(129),
    );

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
        iota_sdk::client::api::transaction_builder::TransactionBuilderError::InvalidOutputCount(129)
    )
}

#[test]
fn too_many_outputs_with_remainder() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000_000,
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

    let outputs = build_outputs(
        std::iter::repeat_with(|| Basic {
            amount: 1_000_000,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            native_token: None,
            sender: None,
            sdruc: None,
            timelock: None,
            expiration: None,
        })
        .take(128),
    );

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
        // 129 because of required remainder
        iota_sdk::client::api::transaction_builder::TransactionBuilderError::InvalidOutputCount(129)
    )
}

#[test]
fn restricted_ed25519() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let address = Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap();
    let restricted = Address::from(RestrictedAddress::new(address.clone()).unwrap());

    let inputs = build_inputs(
        [
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
            (
                Basic {
                    amount: 1_000_000,
                    address: restricted,
                    native_token: None,
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

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert_eq!(selected.inputs_data.len(), 1);
    assert_eq!(selected.inputs_data, [inputs[2].clone()]);
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn restricted_nft() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();
    let nft_address = Address::from(nft_id_1);
    let restricted = Address::from(RestrictedAddress::new(nft_address.clone()).unwrap());

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    address: restricted,
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
            (
                Nft {
                    amount: 2_000_000,
                    nft_id: nft_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                    sdruc: None,
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
}

#[test]
fn restricted_account() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let account_address = Address::from(account_id_1);
    let restricted = Address::from(RestrictedAddress::new(account_address.clone()).unwrap());

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 3_000_000,
                    address: restricted,
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
                    amount: 2_000_000,
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
        amount: 3_000_000,
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
}

#[test]
fn restricted_ed25519_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let sender = Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap();
    let restricted_sender = Address::from(RestrictedAddress::new(sender.clone()).unwrap());

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    native_token: None,
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
                    native_token: None,
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
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                    native_token: None,
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
                    native_token: None,
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
        amount: 2_000_000,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: Some(restricted_sender),
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

    // Sender + another for amount
    assert_eq!(selected.inputs_data.len(), 2);
    assert!(
        selected
            .inputs_data
            .iter()
            .any(|input| *input.output.as_basic().address() == sender)
    );
    // Provided output + remainder
    assert_eq!(selected.transaction.outputs().len(), 2);
}

#[test]
fn multi_address_sender_already_fulfilled() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
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

    let inputs = build_inputs(
        [
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
            (
                Basic {
                    amount: 1_000_000,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                    native_token: None,
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
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
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
        sender: Some(multi),
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
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
        ],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_required_inputs([*inputs[0].output_id(), *inputs[1].output_id(), *inputs[2].output_id()])
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(&selected.transaction.outputs(), &outputs));
}

#[test]
fn ed25519_backed_available_address() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let ed25519 = Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap();
    let restricted_address = Address::from(
        RestrictedAddress::new(ed25519.clone())
            .unwrap()
            .with_allowed_capabilities(AddressCapabilities::all()),
    );

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 1_000_000,
                    address: restricted_address.clone(),
                    native_token: None,
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
                    address: ed25519.clone(),
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
        Basic {
            amount: 1_000_000,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            native_token: None,
            sender: None,
            sdruc: None,
            timelock: None,
            expiration: None,
        },
        Basic {
            amount: 1_000_000,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            native_token: None,
            sender: Some(restricted_address.clone()),
            sdruc: None,
            timelock: None,
            expiration: None,
        },
    ]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        // Restricted address is provided, but it can also unlock the ed25519 one
        [restricted_address],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    // Provided outputs
    assert_eq!(selected.transaction.outputs(), outputs);
}

#[test]
fn automatic_allotment_provided_in_and_output() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = [BasicOutputBuilder::new_with_amount(1_000_000)
        .add_unlock_condition(AddressUnlockCondition::new(
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        ))
        .with_mana(7577)
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

    let outputs = vec![
        BasicOutputBuilder::new_with_amount(1_000_000)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .with_mana(1)
            .finish_output()
            .unwrap(),
    ];

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_min_mana_allotment(account_id_1, 2)
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 1);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    assert_eq!(selected.transaction.allotments().len(), 1);
    let mana_cost = 7576;
    assert_eq!(
        selected.transaction.allotments()[0],
        ManaAllotment::new(account_id_1, mana_cost).unwrap()
    );
    assert_eq!(selected.transaction.outputs()[0].mana(), 1);
}
