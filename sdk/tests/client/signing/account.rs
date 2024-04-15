// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        api::{GetAddressesOptions, PreparedTransactionData},
        constants::SHIMMER_COIN_TYPE,
        secret::{SecretManage, SecretManager},
        Client,
    },
    types::block::{
        address::{AccountAddress, Address},
        input::{Input, UtxoInput},
        output::AccountId,
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

use crate::client::{
    build_inputs, build_outputs,
    Build::{Account, Basic},
    ACCOUNT_ID_1,
};

#[tokio::test]
async fn sign_account_state_transition() -> Result<(), Box<dyn std::error::Error>> {
    let secret_manager = SecretManager::try_from_mnemonic(Client::generate_mnemonic()?)?;

    let address = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(0..1),
        )
        .await?[0]
        .clone()
        .into_inner();

    let protocol_parameters = iota_mainnet_protocol_parameters();
    let account_id = AccountId::from_str(ACCOUNT_ID_1)?;
    let slot_index = SlotIndex::from(10);

    let inputs = build_inputs(
        [(
            Account {
                amount: 1_000_000,
                mana: 0,
                account_id,
                address: address.clone(),
                sender: None,
                issuer: None,
            },
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        )],
        Some(slot_index),
    );

    let outputs = build_outputs([Account {
        amount: 1_000_000,
        mana: 0,
        account_id,
        address: address.clone(),
        sender: None,
        issuer: None,
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
async fn account_reference_unlocks() -> Result<(), Box<dyn std::error::Error>> {
    let secret_manager = SecretManager::try_from_mnemonic(Client::generate_mnemonic()?)?;

    let address = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(0..1),
        )
        .await?[0]
        .clone()
        .into_inner();

    let protocol_parameters = iota_mainnet_protocol_parameters();
    let account_id = AccountId::from_str(ACCOUNT_ID_1)?;
    let account_address = Address::Account(AccountAddress::new(account_id));
    let slot_index = SlotIndex::from(10);

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 1_000_000,
                    mana: 0,
                    account_id,
                    address: address.clone(),
                    sender: None,
                    issuer: None,
                },
                Some(Bip44::new(SHIMMER_COIN_TYPE)),
            ),
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: account_address.clone(),
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
                    mana: 0,
                    address: account_address.clone(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                None,
            ),
        ],
        Some(slot_index),
    );

    let outputs = build_outputs([
        Account {
            amount: 1_000_000,
            mana: 0,
            account_id,
            address,
            sender: None,
            issuer: None,
        },
        Basic {
            amount: 2_000_000,
            mana: 0,
            address: account_address,
            native_token: None,
            sender: None,
            sdruc: None,
            timelock: None,
            expiration: None,
        },
    ]);

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
        Unlock::Account(a) => {
            assert_eq!(a.index(), 0);
        }
        _ => panic!("Invalid unlock"),
    }
    match (*unlocks).get(2).unwrap() {
        Unlock::Account(a) => {
            assert_eq!(a.index(), 0);
        }
        _ => panic!("Invalid unlock"),
    }

    SignedTransactionPayload::new(prepared_transaction_data.transaction.clone(), unlocks)?;

    prepared_transaction_data.verify_semantic(protocol_parameters)?;

    Ok(())
}
