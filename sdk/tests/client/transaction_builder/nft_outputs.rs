// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::{
    client::{
        api::transaction_builder::{Burn, Requirement, TransactionBuilder, TransactionBuilderError},
        secret::types::InputSigningData,
    },
    types::block::{
        address::Address,
        output::{feature::MetadataFeature, unlock_condition::AddressUnlockCondition, NftId, NftOutputBuilder, Output},
        payload::signed_transaction::{TransactionCapabilities, TransactionCapabilityFlag},
        protocol::iota_mainnet_protocol_parameters,
        rand::output::{rand_output_id_with_slot_index, rand_output_metadata_with_id},
        semantic::TransactionFailureReason,
    },
};
use pretty_assertions::{assert_eq, assert_ne};

use crate::client::{
    assert_remainder_or_return, build_inputs, build_outputs, unsorted_eq,
    Build::{Basic, Nft},
    BECH32_ADDRESS_ACCOUNT_1, BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1, BECH32_ADDRESS_NFT_1, NFT_ID_0,
    NFT_ID_1, NFT_ID_2, SLOT_COMMITMENT_ID, SLOT_INDEX,
};

#[test]
fn input_nft_eq_output_nft() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Nft {
                amount: 1_000_000,
                mana: 0,
                nft_id: nft_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
                sdruc: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
        sdruc: None,
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
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

#[test]
fn transition_nft_id_zero() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

    let inputs = build_inputs(
        [(
            Nft {
                amount: 1_000_000,
                mana: 0,
                nft_id: nft_id_0,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
                sdruc: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let nft_id = NftId::from(inputs[0].output_id());
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
        sdruc: None,
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
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

// #[test]
// fn input_amount_lt_output_amount() {
//     let protocol_parameters = protocol_parameters();
//     let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

//     let inputs = build_inputs([Nft(
//         1_000_000,
//         nft_id_2,
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         None,
//         None,
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
//         protocol_parameters,
//     )
//     .select();

//     println!("{selected:?}");

//     assert!(matches!(
//         selected,
//         Err(Error::InsufficientAmount {
//             found: 1_000_000,
//             // Amount we want to send + storage deposit for nft remainder
//             required: 2_233_500,
//         })
//     ));
// }

// #[test]
// fn basic_output_with_nft_input() {
//     let protocol_parameters = protocol_parameters();
//     let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

//     let inputs = build_inputs([Nft(
//         2_237_500,
//         nft_id_2,
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         None,
//         None,
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
//         protocol_parameters,
//     )
//     .select()
//     .unwrap();

//     assert!(unsorted_eq(&selected.inputs_data, &inputs));
//     // basic output + nft remainder
//     assert_eq!(selected.transaction.outputs().len(), 2);
// }

#[test]
fn mint_nft() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

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
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
        sdruc: None,
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
    // One output should be added for the remainder
    assert_eq!(selected.transaction.outputs().len(), 2);
    // Output contains the new minted nft id
    assert!(selected.transaction.outputs().iter().any(|output| {
        if let Output::Nft(nft_output) = output {
            *nft_output.nft_id() == nft_id_0
        } else {
            false
        }
    }));
}

#[test]
fn burn_nft() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Nft {
                amount: 2_000_000,
                mana: 0,
                nft_id: nft_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
                sdruc: None,
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
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .with_burn(Burn::new().add_nft(nft_id_2))
    .finish()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyNftOutputs])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

// #[test]
// fn not_enough_storage_deposit_for_remainder() {
//     let protocol_parameters = protocol_parameters();
//     let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

//     let inputs = build_inputs([Nft(
//         1_000_001,
//         nft_id_2,
//         Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
//         None,
//         None,
//         None,
//         None,
//         None,
//         None,
//     )]);
//     let outputs = build_outputs([Nft(
//         1_000_000,
//         nft_id_2,
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
fn missing_input_for_nft_output() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

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
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
        sdruc: None,
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
        Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Nft(nft_id))) if nft_id == nft_id_2
    ));
}

#[test]
fn missing_input_for_nft_output_but_created() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

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
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
        sdruc: None,
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

    assert!(selected.is_ok());
}

#[test]
fn nft_in_output_and_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Nft {
                    amount: 1_000_000,
                    mana: 0,
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
    let outputs = build_outputs([
        Nft {
            amount: 1_000_000,
            mana: 0,
            nft_id: nft_id_1,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            sender: None,
            issuer: None,
            sdruc: None,
            expiration: None,
        },
        Basic {
            amount: 1_000_000,
            mana: 0,
            address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            native_token: None,
            sender: Some(Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()),
            sdruc: None,
            timelock: None,
            expiration: None,
        },
    ]);

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
    assert!(selected.transaction.outputs().iter().any(|output| {
        if let Output::Nft(nft_output) = output {
            *nft_output.nft_id() == nft_id_1
        } else {
            false
        }
    }));
    assert!(selected.transaction.outputs().iter().any(|output| output.is_basic()));
}

#[test]
fn missing_ed25519_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Nft {
                amount: 1_000_000,
                mana: 0,
                nft_id: nft_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
                sdruc: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
        issuer: None,
        sdruc: None,
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
fn missing_ed25519_issuer_created() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

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
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
        sdruc: None,
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
        Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer == Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()
    ));
}

#[test]
fn missing_ed25519_issuer_transition() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [(
            Nft {
                amount: 1_000_000,
                mana: 0,
                nft_id: nft_id_1,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
                sdruc: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_1,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
        sdruc: None,
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

    assert!(selected.is_ok());
}

#[test]
fn missing_account_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Nft {
                amount: 1_000_000,
                mana: 0,
                nft_id: nft_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
                sdruc: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
        issuer: None,
        sdruc: None,
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
fn missing_account_issuer_created() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

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
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
        sdruc: None,
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
        Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer == Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()
    ));
}

#[test]
fn missing_account_issuer_transition() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Nft {
                amount: 1_000_000,
                mana: 0,
                nft_id: nft_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
                sdruc: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap()),
        sdruc: None,
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

    assert!(selected.is_ok());
}

#[test]
fn missing_nft_sender() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Nft {
                amount: 1_000_000,
                mana: 0,
                nft_id: nft_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
                sdruc: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: Some(Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()),
        issuer: None,
        sdruc: None,
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
fn missing_nft_issuer_created() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

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
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_0,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()),
        sdruc: None,
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
        Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer == Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()
    ));
}

#[test]
fn missing_nft_issuer_transition() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_inputs(
        [(
            Nft {
                amount: 1_000_000,
                mana: 0,
                nft_id: nft_id_2,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()),
                sdruc: None,
                expiration: None,
            },
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_2,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: Some(Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap()),
        sdruc: None,
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

    assert!(selected.is_ok());
}

#[test]
fn increase_nft_amount() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Nft {
                    amount: 2_000_000,
                    mana: 0,
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
    let outputs = build_outputs([Nft {
        amount: 3_000_000,
        mana: 0,
        nft_id: nft_id_1,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
        sdruc: None,
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
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

#[test]
fn decrease_nft_amount() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Nft {
                    amount: 2_000_000,
                    mana: 0,
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
    let outputs = build_outputs([Nft {
        amount: 1_000_000,
        mana: 0,
        nft_id: nft_id_1,
        address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        sender: None,
        issuer: None,
        sdruc: None,
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
fn prefer_basic_to_nft() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Nft {
                    amount: 2_000_000,
                    mana: 0,
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
fn take_amount_from_nft_to_fund_basic() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Nft {
                    amount: 2_000_000,
                    mana: 0,
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
            assert!(output.is_nft());
            assert_eq!(output.amount(), 1_800_000);
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
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

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
                Nft {
                    amount: 1_000_000,
                    mana: 0,
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
        mana: 0,
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
    .with_burn(Burn::new().add_nft(nft_id_1))
    .finish()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyNftOutputs])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

#[test]
fn nft_burn_should_validate_nft_address() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: Address::try_from_bech32(BECH32_ADDRESS_NFT_1).unwrap(),
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
                    mana: 0,
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
    .with_burn(Burn::new().add_nft(nft_id_1))
    .finish()
    .unwrap();

    assert_eq!(
        selected.transaction.capabilities(),
        &TransactionCapabilities::from([TransactionCapabilityFlag::DestroyNftOutputs])
    );
    assert!(unsorted_eq(&selected.inputs_data, &inputs));
    assert!(unsorted_eq(selected.transaction.outputs(), &outputs));
}

#[test]
fn transitioned_zero_nft_id_no_longer_is_zero() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

    let inputs = build_inputs(
        [(
            Nft {
                amount: 2_000_000,
                mana: 0,
                nft_id: nft_id_0,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
                sdruc: None,
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
            assert!(output.is_nft());
            assert_eq!(output.amount(), 1_000_000);
            assert_ne!(*output.as_nft().nft_id(), nft_id_0);
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
fn changed_immutable_metadata() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    #[cfg(feature = "irc_27")]
    let metadata = iota_sdk::types::block::output::feature::Irc27Metadata::new(
        "image/jpeg",
        "https://mywebsite.com/my-nft-files-1.jpeg".parse().unwrap(),
        "file 1",
    )
    .with_issuer_name("Alice");
    #[cfg(not(feature = "irc_27"))]
    let metadata = vec![("42".to_owned(), vec![42])];

    let nft_output =
        NftOutputBuilder::new_with_minimum_amount(protocol_parameters.storage_score_parameters(), nft_id_1)
            .with_immutable_features(MetadataFeature::try_from(metadata))
            .add_unlock_condition(AddressUnlockCondition::new(
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            ))
            .finish_output()
            .unwrap();

    let inputs = [InputSigningData {
        output: nft_output.clone(),
        output_metadata: rand_output_metadata_with_id(rand_output_id_with_slot_index(SLOT_INDEX)),
        chain: None,
    }];

    #[cfg(feature = "irc_27")]
    let metadata = iota_sdk::types::block::output::feature::Irc27Metadata::new(
        "image/jpeg",
        "https://mywebsite.com/my-nft-files-2.jpeg".parse().unwrap(),
        "file 2",
    )
    .with_issuer_name("Alice");
    #[cfg(not(feature = "irc_27"))]
    let metadata = vec![("43".to_owned(), vec![43])];

    // New nft output with changed immutable metadata feature
    let updated_nft_output = NftOutputBuilder::from(nft_output.as_nft())
        .with_minimum_amount(protocol_parameters.storage_score_parameters())
        .with_immutable_features(MetadataFeature::try_from(metadata))
        .finish_output()
        .unwrap();

    let outputs = [updated_nft_output];

    let selected = TransactionBuilder::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        SLOT_COMMITMENT_ID,
        protocol_parameters,
    )
    .finish()
    .unwrap_err();

    assert_eq!(
        selected,
        TransactionBuilderError::Semantic(TransactionFailureReason::ChainOutputImmutableFeaturesChanged)
    );
}

#[test]
fn auto_transition_nft_less_than_min() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let small_amount = 5;

    let inputs = build_inputs(
        [(
            Nft {
                amount: small_amount,
                mana: 0,
                nft_id: nft_id_1,
                address: Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                sender: None,
                issuer: None,
                sdruc: None,
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
        protocol_parameters.clone(),
    )
    .with_required_inputs([*inputs[0].output_id()])
    .finish()
    .unwrap_err();

    let min_amount = NftOutputBuilder::from(inputs[0].output.as_nft())
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
fn auto_transition_nft_less_than_min_additional() {
    let protocol_parameters = iota_mainnet_protocol_parameters().clone();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let small_amount = 5;

    let inputs = build_inputs(
        [
            (
                Nft {
                    amount: small_amount,
                    mana: 0,
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
    assert_eq!(selected.transaction.outputs().len(), 2);
    let min_amount = NftOutputBuilder::from(inputs[0].output.as_nft())
        .with_minimum_amount(protocol_parameters.storage_score_parameters())
        .finish_output()
        .unwrap()
        .amount();
    let nft_output = selected
        .transaction
        .outputs()
        .iter()
        .filter_map(Output::as_nft_opt)
        .find(|o| o.nft_id() == &nft_id_1)
        .unwrap();
    assert_eq!(nft_output.amount(), min_amount);
}
