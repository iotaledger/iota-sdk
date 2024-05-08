// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        api::{
            transaction_builder::{Burn, Requirement, TransactionBuilder, TransactionBuilderError, Transitions},
            GetAddressesOptions,
        },
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, types::InputSigningData, SecretManage, SecretManager},
        Client,
    },
    types::block::{
        address::{Address, ImplicitAccountCreationAddress},
        core::{
            basic::{MaxBurnedManaAmount, StrongParents},
            BlockHeader,
        },
        mana::ManaAllotment,
        output::{
            feature::{BlockIssuerFeature, BlockIssuerKeys, Ed25519PublicKeyHashBlockIssuerKey},
            unlock_condition::AddressUnlockCondition,
            AccountId, AccountOutputBuilder, BasicOutputBuilder, Output,
        },
        payload::{
            signed_transaction::{TransactionCapabilities, TransactionCapabilityFlag},
            Payload, SignedTransactionPayload,
        },
        protocol::{iota_mainnet_protocol_parameters, WorkScore},
        rand::output::{rand_output_id_with_slot_index, rand_output_metadata_with_id},
        slot::{SlotCommitmentId, SlotIndex},
        BlockBody, BlockId, UnsignedBlock,
    },
};
use pretty_assertions::{assert_eq, assert_ne};

use crate::client::{
    assert_remainder_or_return, build_inputs, build_outputs, unsorted_eq,
    Build::{Account, Basic},
    ACCOUNT_ID_0, ACCOUNT_ID_1, ACCOUNT_ID_2, BECH32_ADDRESS_ACCOUNT_1, BECH32_ADDRESS_ED25519_0,
    BECH32_ADDRESS_ED25519_1, BECH32_ADDRESS_NFT_1, SLOT_COMMITMENT_ID, SLOT_INDEX,
};

#[test]
fn input_account_eq_output_account() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 1_000_000,
                mana: 0,
                account_id: account_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
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
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

#[test]
fn transition_account_id_zero() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 1_000_000,
                mana: 0,
                account_id: account_id_0,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let account_id = AccountId::from(inputs[0].output_id());
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
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
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

// #[test]
// fn input_amount_lt_output_amount() {
//     let protocol_parameters = iota_mainnet_protocol_parameters().clone();
//     let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

//     let inputs = build_inputs([Account(
//         1_000_000,
//         account_id_2,
//         0,
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         None,
//         None,
//         None,
//         None,
//     )]);
//     let outputs = build_outputs([Basic(
//         2_000_000,
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
// SLOT_INDEX+1,        protocol_parameters,
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
//     let protocol_parameters = iota_mainnet_protocol_parameters().clone();
//     let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

//     let inputs = build_inputs([
//         Account(
//             2_000_000,
//             account_id_2,
//             0,
//             Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//             Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//             None,
//             None,
//             None,
//             None,
//         ),
//         Basic(1_000_000, Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), None, None, None, None, None,
// None),     ]);
//     let outputs = build_outputs([Basic(
//         3_000_001,
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
// SLOT_INDEX+1,        protocol_parameters,
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
//     let protocol_parameters = iota_mainnet_protocol_parameters().clone();
//     let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

//     let inputs = build_inputs([Account(
//         2_259_500,
//         account_id_2,
//         0,
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         None,
//         None,
//         None,
//         None,
//     )]);
//     let outputs = build_outputs([Basic(
//         2_000_000,
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
//         outputs,
//         [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
// SLOT_INDEX+1,        protocol_parameters,
//     )
//     .select()
//     .unwrap();

//     assert!(unsorted_eq(&selected.inputs_data, &inputs));
//     // basic output + account remainder
//     assert_eq!(selected.transaction.outputs().len(), 2);
// }

#[test]
fn create_account() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 2_000_000,
                mana: 0,
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
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
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
    // One output should be added for the remainder
    assert_eq!(selected.transaction.outputs().len(), 2);
    // Output contains the new minted account id
    assert!(selected.transaction.outputs().iter().any(|output| {
        if let Output::Account(account_output) = output {
            *account_output.account_id() == account_id_0
        } else {
            false
        }
    }));
}

#[test]
fn burn_account() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 2_000_000,
                mana: 0,
                account_id: account_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
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
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_account(account_id_2))
    .finish()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyAccountOutputs])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

// #[test]
// fn not_enough_storage_deposit_for_remainder() {
//     let protocol_parameters = iota_mainnet_protocol_parameters().clone();
//     let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

//     let inputs = build_inputs([Account(
//         1_000_001,
//         account_id_2,
//         0,
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         None,
//         None,
//         None,
//         None,
//     )]);
//     let outputs = build_outputs([Account(
//         1_000_000,
//         account_id_2,
//         0,
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         None,
//         None,
//         None,
//         None,
//     )]);

//     let selected = TransactionBuilder::new(
//         inputs,
//         outputs,
//         [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
// SLOT_INDEX+1,        protocol_parameters,
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
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
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
            None,
        )],
        None,
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
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
        Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Account(account_id))) if account_id == account_id_2
    ));
}

#[test]
fn missing_input_for_account_output_2() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 2_000_000,
                    mana: 0,
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
                    mana: 0,
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
        None,
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
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
        Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Account(account_id))) if account_id == account_id_2
    ));
}

#[test]
fn missing_input_for_account_output_but_created() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs(
        [(
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
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
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

    assert!(selected.is_ok());
}

#[test]
fn account_in_output_and_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
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
            (
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
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );
    let account_output = AccountOutputBuilder::from(inputs[0].output.as_account())
        .finish_output()
        .unwrap();
    let mut outputs = build_outputs([Basic {
        amount: 1_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);
    outputs.push(account_output);

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
fn missing_ed25519_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 1_000_000,
                mana: 0,
                account_id: account_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
            },
            None,
        )],
        None,
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
        issuer: None,
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
fn missing_ed25519_issuer_created() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs(
        [(
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
            None,
        )],
        None,
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
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
        Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn missing_ed25519_issuer_transition() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 1_000_000,
                mana: 0,
                account_id: account_id_1,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_1,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
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

    assert!(selected.is_ok());
}

#[test]
fn missing_account_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 1_000_000,
                mana: 0,
                account_id: account_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
            },
            None,
        )],
        None,
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
        issuer: None,
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
fn missing_account_issuer_created() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs(
        [(
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
            None,
        )],
        None,
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
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
        Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer == Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()
    ));
}

#[test]
fn missing_account_issuer_transition() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 1_000_000,
                mana: 0,
                account_id: account_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
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

    assert!(selected.is_ok());
}

#[test]
fn missing_nft_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 1_000_000,
                mana: 0,
                account_id: account_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
            },
            None,
        )],
        None,
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()),
        issuer: None,
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
fn missing_nft_issuer_created() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs(
        [(
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
            None,
        )],
        None,
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()),
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
        Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer == Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()
    ));
}

#[test]
fn missing_nft_issuer_transition() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 1_000_000,
                mana: 0,
                account_id: account_id_1,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()),
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_1,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()),
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

    assert!(selected.is_ok());
}

#[test]
fn increase_account_amount() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 2_000_000,
                    mana: 0,
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
                    mana: 0,
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
    let outputs = build_outputs([Account {
        amount: 3_000_000,
        mana: 0,
        account_id: account_id_1,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
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
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

#[test]
fn decrease_account_amount() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 2_000_000,
                    mana: 0,
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
                    mana: 0,
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
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_1,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
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
    assert_eq!(selected.inputs_data[0], inputs[0]);
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
fn prefer_basic_to_account() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
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
            (
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
    assert_eq!(selected.inputs_data[0], inputs[1]);
    assert_eq!(selected.transaction.outputs(), outputs);
}

#[test]
fn take_amount_from_account_to_fund_basic() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
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
            (
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
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic {
        amount: 1_200_000,
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
            assert!(output.is_account());
            assert_eq!(output.amount(), 800_000);
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
fn account_burn_should_validate_account_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
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
        amount: 2_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
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
    .with_burn(Burn::new().add_account(account_id_1))
    .finish()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyAccountOutputs])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    // One output should be added for the remainder.
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
            )
        }
    });
}

#[test]
fn account_burn_should_validate_account_address() {
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
    .with_burn(Burn::new().add_account(account_id_1))
    .finish()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyAccountOutputs])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    // One output should be added for the remainder.
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    selected.transaction.outputs().iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
            )
        }
    });
}

#[test]
fn transitioned_zero_account_id_no_longer_is_zero() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_0 = AccountId::from_str(ACCOUNT_ID_0).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 2_000_000,
                mana: 0,
                account_id: account_id_0,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
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
            assert!(output.is_account());
            assert_eq!(output.amount(), 1_000_000);
            assert_ne!(*output.as_account().account_id(), account_id_0);
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
fn two_accounts_required() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 2_000_000,
                    mana: 0,
                    account_id: account_id_1,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
            (
                Account {
                    amount: 2_000_000,
                    mana: 0,
                    account_id: account_id_2,
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
    assert!(
        selected
            .transaction
            .outputs()
            .iter()
            .any(|output| if let Output::Account(output) = output {
                output.account_id() == &account_id_1
            } else {
                false
            })
    );
    assert!(
        selected
            .transaction
            .outputs()
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
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 2_000_000,
                mana: 0,
                account_id: account_id_1,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
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
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()),
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
fn state_controller_sender_required_already_selected() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 2_000_000,
                mana: 0,
                account_id: account_id_1,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([
        Account {
            amount: 1_000_000,
            mana: 0,
            account_id: account_id_1,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            sender: None,
            issuer: None,
        },
        Basic {
            amount: 1_000_000,
            mana: 0,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            native_token: None,
            sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()),
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
    .with_required_inputs([*inputs[0].output_id()])
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

#[test]
fn state_transition_and_required() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 2_000_000,
                mana: 0,
                account_id: account_id_1,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Account {
        amount: 2_000_000,
        mana: 0,
        account_id: account_id_1,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
    }]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_required_inputs([*inputs[0].output_id()])
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

#[test]
fn remainder_address_in_state_controller() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [(
            Account {
                amount: 2_000_000,
                mana: 0,
                account_id: account_id_1,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id: account_id_1,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
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
            )
        }
    });
}

#[test]
fn min_allot_account_mana() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let mana_input_amount = 1_000_000;
    let required_allotment = 7864;

    let inputs = build_inputs(
        [(
            Account {
                amount: 2_000_000,
                mana: mana_input_amount,
                account_id: account_id_1,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
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
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()),
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
    .with_min_mana_allotment(account_id_1, 2)
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    assert_eq!(selected.transaction.allotments().len(), 1);
    assert_eq!(
        selected.transaction.allotments()[0],
        ManaAllotment::new(account_id_1, required_allotment).unwrap()
    );
    assert_eq!(
        selected.transaction.outputs()[1].as_account().mana(),
        mana_input_amount - required_allotment
    );
}

#[test]
fn min_allot_account_mana_additional() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let provided_allotment = 1000;
    let required_allotment = 7900;
    // The account does not have enough to cover the requirement
    let account_mana = required_allotment - 500;
    // But there is additional available mana elsewhere
    let additional_available_mana = 511;

    let inputs = [
        AccountOutputBuilder::new_with_amount(2_000_000, account_id_1)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .with_mana(account_mana)
            .finish_output()
            .unwrap(),
        BasicOutputBuilder::new_with_minimum_amount(protocol_parameters.storage_score_parameters())
            .with_mana(additional_available_mana)
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

    let outputs = [BasicOutputBuilder::new_with_amount(1_000_000)
        .add_unlock_condition(AddressUnlockCondition::new(
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        ))
        .finish_output()
        .unwrap()];

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_min_mana_allotment(account_id_1, 2)
    .with_mana_allotments(Some((account_id_1, provided_allotment)))
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));

    assert_eq!(selected.transaction.allotments().len(), 1);
    assert_eq!(
        selected.transaction.allotments()[0],
        ManaAllotment::new(account_id_1, required_allotment).unwrap()
    );
    assert_eq!(
        selected.transaction.outputs().iter().map(|o| o.mana()).sum::<u64>(),
        account_mana + additional_available_mana - required_allotment
    );
}

#[test]
fn min_allot_account_mana_cannot_select_additional() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2).unwrap();

    let provided_allotment = 1000;
    let required_allotment = 7900;
    // The account does not have enough to cover the requirement
    let account_mana = required_allotment - 100;
    // But there is additional available mana elsewhere
    let additional_available_mana = 111;

    let inputs = [
        AccountOutputBuilder::new_with_amount(2_000_000, account_id_1)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .with_mana(account_mana)
            .finish_output()
            .unwrap(),
        BasicOutputBuilder::new_with_minimum_amount(protocol_parameters.storage_score_parameters())
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .with_mana(additional_available_mana)
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

    let selected = TransactionBuilder::new(
        inputs.clone(),
        None,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_min_mana_allotment(account_id_1, 2)
    .with_mana_allotments(Some((account_id_2, provided_allotment)))
    .with_required_inputs([*inputs[0].output_id()])
    .disable_additional_input_selection()
    .finish()
    .unwrap_err();

    assert!(
        matches!(selected, TransactionBuilderError::AdditionalInputsRequired(_)),
        "expected AdditionalInputsRequired, found {selected:?}"
    );
}

#[test]
fn min_allot_account_mana_requirement_twice() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let required_allotment = 7900;

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 2_000_000,
                    mana: required_allotment,
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
                    mana: 100,
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

    let selected = TransactionBuilder::new(
        inputs.clone(),
        None,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_min_mana_allotment(account_id_1, 2)
    .with_required_inputs([*inputs[1].output_id()])
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 1);
    let account_output = selected.transaction.outputs()[0].as_account();
    assert_eq!(selected.transaction.allotments().len(), 1);
    assert_eq!(
        selected.transaction.allotments()[0],
        ManaAllotment::new(account_id_1, required_allotment).unwrap()
    );
    assert_eq!(account_output.account_id(), &account_id_1);
    assert_eq!(account_output.mana(), 100);
}

#[test]
fn min_allot_account_mana_requirement_covered() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let provided_allotment = 7900;

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 2_000_000,
                    mana: provided_allotment - 100,
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
                    mana: 100,
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

    // Must manually add account output with mana reduced for the manual allotment
    let account_output = AccountOutputBuilder::from(inputs[0].output.as_account())
        .with_mana(0)
        .finish_output()
        .unwrap();

    let mut outputs = build_outputs([Basic {
        amount: 1_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);
    outputs.push(account_output);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_min_mana_allotment(account_id_1, 2)
    .with_mana_allotments(Some((account_id_1, provided_allotment)))
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    assert_eq!(selected.transaction.allotments().len(), 1);
    assert_eq!(
        selected.transaction.allotments()[0],
        ManaAllotment::new(account_id_1, provided_allotment).unwrap()
    );
    assert_eq!(selected.transaction.outputs()[1].as_account().mana(), 0);
}

#[test]
fn min_allot_account_mana_requirement_covered_2() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let provided_allotment = 7900;

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 2_000_000,
                    mana: 100,
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
                    mana: provided_allotment - 100,
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

    // Must manually add account output with mana reduced for the manual allotment
    let account_output = AccountOutputBuilder::from(inputs[0].output.as_account())
        .with_mana(0)
        .finish_output()
        .unwrap();

    let mut outputs = build_outputs([Basic {
        amount: 1_000_000,
        mana: 0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);
    outputs.push(account_output);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_min_mana_allotment(account_id_1, 2)
    .with_mana_allotments(Some((account_id_1, provided_allotment)))
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs().contains(&outputs[0]));
    assert_eq!(selected.transaction.allotments().len(), 1);
    assert_eq!(
        selected.transaction.allotments()[0],
        ManaAllotment::new(account_id_1, provided_allotment).unwrap()
    );
    assert_eq!(selected.transaction.outputs()[1].as_account().mana(), 0);
}

#[test]
fn implicit_account_transition() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let ed25519_address = Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                mana: 10000,
                address: Address::ImplicitAccountCreation(ImplicitAccountCreationAddress::new(
                    **ed25519_address.as_ed25519(),
                )),
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
    let input_output_id = *inputs[0].output_id();
    let block_issuer_key = Ed25519PublicKeyHashBlockIssuerKey::new(**ed25519_address.as_ed25519());

    let selected = TransactionBuilder::new(
        inputs.clone(),
        None,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_required_inputs(vec![input_output_id])
    .with_min_mana_allotment(account_id_1, 2)
    .with_transitions(Transitions::new().add_implicit_account(input_output_id, block_issuer_key.into()))
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 1);
    assert!(selected.transaction.outputs()[0].is_account());
    assert_eq!(selected.transaction.allotments().len(), 1);
    assert_eq!(
        selected.transaction.allotments()[0],
        ManaAllotment::new(account_id_1, 9948).unwrap()
    );
    // One remainder Mana
    assert_eq!(selected.transaction.outputs()[0].mana(), 52);
}

#[test]
fn auto_transition_account_less_than_min() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let small_amount = 5;

    let inputs = build_inputs(
        [(
            Account {
                amount: small_amount,
                mana: 0,
                account_id: account_id_1,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
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
        protocol_parameters.clone(),
    )
    .with_required_inputs([*inputs[0].output_id()])
    .finish()
    .unwrap_err();

    let min_amount = AccountOutputBuilder::from(inputs[0].output.as_account())
        .with_minimum_amount(protocol_parameters.storage_score_parameters())
        .finish_output()
        .unwrap()
        .amount();

    assert_eq!(
        selected,
        TransactionBuilderError::InsufficientAmount {
            found: small_amount,
            required: min_amount
        },
    );
}

#[test]
fn auto_transition_account_less_than_min_additional() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let small_amount = 5;

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: small_amount,
                    mana: 0,
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
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                    sender: None,
                    native_token: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );

    let selected = TransactionBuilder::new(
        inputs.clone(),
        None,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters.clone(),
    )
    .with_required_inputs([*inputs[0].output_id()])
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 1);
    let account_output = selected.transaction.outputs()[0].as_account();
    assert_eq!(account_output.account_id(), &account_id_1);
    assert_eq!(
        account_output.amount(),
        inputs.iter().map(|i| i.output.amount()).sum::<u64>()
    );
}

#[test]
fn account_transition_with_required_context_inputs() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let ed25519_address = Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap();

    let inputs = [
        BasicOutputBuilder::new_with_amount(1_000_000)
            .with_mana(11000)
            .add_unlock_condition(AddressUnlockCondition::new(ed25519_address.clone()))
            .finish_output()
            .unwrap(),
        AccountOutputBuilder::new_with_amount(1_000_000, account_id_1)
            .with_mana(12000)
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .with_features([BlockIssuerFeature::new(
                u32::MAX,
                BlockIssuerKeys::from_vec(vec![
                    Ed25519PublicKeyHashBlockIssuerKey::new(**ed25519_address.as_ed25519()).into(),
                ])
                .unwrap(),
            )
            .unwrap()])
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

    let outputs = vec![
        BasicOutputBuilder::new_with_amount(1_000_000)
            .add_unlock_condition(AddressUnlockCondition::new(ed25519_address.clone()))
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
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs()[1].is_account());
    assert_eq!(selected.transaction.allotments().len(), 1);
    // Required context inputs are added when the account is transitioned
    assert_eq!(selected.transaction.context_inputs().len(), 2);
    assert!(selected.transaction.context_inputs().commitment().is_some());
    assert_eq!(
        selected.transaction.context_inputs().block_issuance_credits().count(),
        1
    );
}

#[test]
fn send_amount_from_block_issuer_account_with_generated_mana() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let ed25519_address = Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap();

    let inputs = [AccountOutputBuilder::new_with_amount(10_000_000, account_id_1)
        .with_mana(20000)
        .add_unlock_condition(AddressUnlockCondition::new(
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        ))
        .with_features([BlockIssuerFeature::new(
            u32::MAX,
            BlockIssuerKeys::from_vec(vec![
                Ed25519PublicKeyHashBlockIssuerKey::new(**ed25519_address.as_ed25519()).into(),
            ])
            .unwrap(),
        )
        .unwrap()])
        .finish_output()
        .unwrap()];
    let inputs = inputs
        .into_iter()
        .map(|input| InputSigningData {
            output: input,
            output_metadata: rand_output_metadata_with_id(rand_output_id_with_slot_index(SlotIndex(5))),
            chain: None,
        })
        .collect::<Vec<_>>();

    let outputs = vec![
        BasicOutputBuilder::new_with_amount(1_000_000)
            .add_unlock_condition(AddressUnlockCondition::new(ed25519_address.clone()))
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
    .with_remainder_address(Address::Account(account_id_1.into()))
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 2);
    assert!(selected.transaction.outputs()[1].is_account());
    assert_eq!(selected.transaction.allotments().len(), 1);
    // Required context inputs are added when the account is transitioned
    assert_eq!(selected.transaction.context_inputs().len(), 2);
    assert!(selected.transaction.context_inputs().commitment().is_some());
    assert_eq!(
        selected.transaction.context_inputs().block_issuance_credits().count(),
        1
    );
}

#[test]
fn custom_allot_account_bound_mana() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();
    let ed25519_address = Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap();

    let provided_allotment = 500_000;
    let account_mana = 2_000_000;

    let inputs = [AccountOutputBuilder::new_with_amount(2_000_000, account_id_1)
        .with_mana(account_mana)
        .add_unlock_condition(AddressUnlockCondition::new(
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        ))
        .add_feature(
            BlockIssuerFeature::new(
                u32::MAX,
                BlockIssuerKeys::from_vec(vec![
                    Ed25519PublicKeyHashBlockIssuerKey::new(**ed25519_address.as_ed25519()).into(),
                ])
                .unwrap(),
            )
            .unwrap(),
        )
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

    let outputs = [];

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_min_mana_allotment(account_id_1, 2)
    .with_mana_allotments(Some((account_id_1, provided_allotment)))
    .finish()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert_eq!(selected.transaction.outputs().len(), 1);

    assert_eq!(selected.transaction.allotments().len(), 1);
    assert_eq!(
        selected.transaction.allotments()[0],
        ManaAllotment::new(account_id_1, provided_allotment).unwrap()
    );
    assert_eq!(
        selected.transaction.outputs().iter().map(|o| o.mana()).sum::<u64>(),
        account_mana - provided_allotment
    );
}
