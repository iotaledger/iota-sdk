// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        api::{GetAddressesOptions, PreparedTransactionData},
        constants::SHIMMER_COIN_TYPE,
        secret::{SecretManage, SecretManager},
        Client,
    },
    types::block::{
        input::{Input, UtxoInput},
        payload::{
            signed_transaction::{Transaction, TransactionCapabilityFlag},
            SignedTransactionPayload,
        },
        protocol::iota_mainnet_protocol_parameters,
        slot::SlotIndex,
        unlock::{SignatureUnlock, Unlock},
    },
};
use pretty_assertions::assert_eq;

use crate::client::{build_inputs, build_outputs, Build::Basic};

#[tokio::test]
async fn single_ed25519_unlock() -> Result<(), Box<dyn std::error::Error>> {
    let secret_manager = SecretManager::try_from_mnemonic(Client::generate_mnemonic()?)?;

    let address_0 = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(0..1),
        )
        .await?[0]
        .clone()
        .into_inner();

    let protocol_parameters = iota_mainnet_protocol_parameters();
    let slot_index = SlotIndex::from(10);

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                mana: 0,
                address: address_0.clone(),
                native_token: None,
                sender: None,
                sdruc: None,
                timelock: None,
                expiration: None,
            },
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        )],
        Some(slot_index),
    );

    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        mana: 0,
        address: address_0,
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index + 1)
        .with_capabilities([TransactionCapabilityFlag::BurnMana])
        .finish_with_params(protocol_parameters)?;

    let prepared_transaction_data = PreparedTransactionData {
        transaction,
        inputs_data: inputs,
        remainders: Vec::new(),
        mana_rewards: Default::default(),
        issuer_id: None,
    };

    let unlocks = secret_manager
        .transaction_unlocks(&prepared_transaction_data, protocol_parameters)
        .await?;

    assert_eq!(unlocks.len(), 1);
    assert_eq!((*unlocks).first().unwrap().kind(), SignatureUnlock::KIND);

    SignedTransactionPayload::new(prepared_transaction_data.transaction.clone(), unlocks)?;

    prepared_transaction_data.verify_semantic(protocol_parameters)?;

    Ok(())
}

#[tokio::test]
async fn ed25519_reference_unlocks() -> Result<(), Box<dyn std::error::Error>> {
    let secret_manager = SecretManager::try_from_mnemonic(Client::generate_mnemonic()?)?;

    let address_0 = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(0..1),
        )
        .await?[0]
        .clone()
        .into_inner();

    let protocol_parameters = iota_mainnet_protocol_parameters();
    let slot_index = SlotIndex::from(10);

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: address_0.clone(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                Some(Bip44::new(SHIMMER_COIN_TYPE)),
            ),
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: address_0.clone(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                Some(Bip44::new(SHIMMER_COIN_TYPE)),
            ),
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: address_0.clone(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                Some(Bip44::new(SHIMMER_COIN_TYPE)),
            ),
        ],
        Some(slot_index),
    );

    let outputs = build_outputs([Basic {
        amount: 3_000_000,
        mana: 0,
        address: address_0,
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index + 1)
        .with_capabilities([TransactionCapabilityFlag::BurnMana])
        .finish_with_params(protocol_parameters)?;

    let prepared_transaction_data = PreparedTransactionData {
        transaction,
        inputs_data: inputs,
        remainders: Vec::new(),
        mana_rewards: Default::default(),
        issuer_id: None,
    };

    let unlocks = secret_manager
        .transaction_unlocks(&prepared_transaction_data, protocol_parameters)
        .await?;

    assert_eq!(unlocks.len(), 3);
    assert_eq!((*unlocks).first().unwrap().kind(), SignatureUnlock::KIND);
    match (*unlocks).get(1).unwrap() {
        Unlock::Reference(r) => {
            assert_eq!(r.index(), 0);
        }
        _ => panic!("Invalid unlock"),
    }
    match (*unlocks).get(2).unwrap() {
        Unlock::Reference(r) => {
            assert_eq!(r.index(), 0);
        }
        _ => panic!("Invalid unlock"),
    }

    SignedTransactionPayload::new(prepared_transaction_data.transaction.clone(), unlocks)?;

    prepared_transaction_data.verify_semantic(protocol_parameters)?;

    Ok(())
}

#[tokio::test]
async fn two_signature_unlocks() -> Result<(), Box<dyn std::error::Error>> {
    let secret_manager = SecretManager::try_from_mnemonic(Client::generate_mnemonic()?)?;

    let address_0 = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(0..1),
        )
        .await?[0]
        .clone()
        .into_inner();
    let address_1 = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(1..2),
        )
        .await?[0]
        .clone()
        .into_inner();

    let protocol_parameters = iota_mainnet_protocol_parameters();
    let slot_index = SlotIndex::from(10);

    let inputs = build_inputs(
        [
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: address_0.clone(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                Some(Bip44::new(SHIMMER_COIN_TYPE)),
            ),
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: address_1,
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                Some(Bip44::new(SHIMMER_COIN_TYPE).with_address_index(1)),
            ),
        ],
        Some(slot_index),
    );

    let outputs = build_outputs([Basic {
        amount: 2_000_000,
        mana: 0,
        address: address_0,
        native_token: None,
        sender: None,
        sdruc: None,
        timelock: None,
        expiration: None,
    }]);

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index + 1)
        .with_capabilities([TransactionCapabilityFlag::BurnMana])
        .finish_with_params(protocol_parameters)?;

    let prepared_transaction_data = PreparedTransactionData {
        transaction,
        inputs_data: inputs,
        remainders: Vec::new(),
        mana_rewards: Default::default(),
        issuer_id: None,
    };

    let unlocks = secret_manager
        .transaction_unlocks(&prepared_transaction_data, protocol_parameters)
        .await?;

    assert_eq!(unlocks.len(), 2);
    assert_eq!((*unlocks).first().unwrap().kind(), SignatureUnlock::KIND);
    assert_eq!((*unlocks).get(1).unwrap().kind(), SignatureUnlock::KIND);

    SignedTransactionPayload::new(prepared_transaction_data.transaction.clone(), unlocks)?;

    prepared_transaction_data.verify_semantic(protocol_parameters)?;

    Ok(())
}
