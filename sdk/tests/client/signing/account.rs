// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        api::{
            transaction::validate_signed_transaction_payload_length, verify_semantic, GetAddressesOptions,
            PreparedTransactionData,
        },
        constants::SHIMMER_COIN_TYPE,
        secret::{SecretManage, SecretManager},
        Client, Result,
    },
    types::block::{
        address::{AccountAddress, Address},
        input::{Input, UtxoInput},
        output::AccountId,
        payload::{signed_transaction::Transaction, SignedTransactionPayload},
        protocol::protocol_parameters,
        rand::mana::rand_mana_allotment,
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
async fn sign_account_state_transition() -> Result<()> {
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

    let protocol_parameters = protocol_parameters();
    let account_id = AccountId::from_str(ACCOUNT_ID_1)?;

    let inputs = build_inputs([Account(
        1_000_000,
        account_id,
        address.clone(),
        None,
        None,
        Some(Bip44::new(SHIMMER_COIN_TYPE)),
    )]);

    let outputs = build_outputs([Account(1_000_000, account_id, address.clone(), None, None, None)]);

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .add_mana_allotment(rand_mana_allotment(&protocol_parameters))
        .finish_with_params(&protocol_parameters)?;

    let prepared_transaction_data = PreparedTransactionData {
        transaction,
        inputs_data: inputs,
        remainder: None,
    };

    let unlocks = secret_manager
        .transaction_unlocks(&prepared_transaction_data, &protocol_parameters)
        .await?;

    assert_eq!(unlocks.len(), 1);
    assert_eq!((*unlocks).first().unwrap().kind(), SignatureUnlock::KIND);

    let tx_payload = SignedTransactionPayload::new(prepared_transaction_data.transaction.clone(), unlocks)?;

    validate_signed_transaction_payload_length(&tx_payload)?;

    let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, protocol_parameters)?;

    if let Some(conflict) = conflict {
        panic!("{conflict:?}, with {tx_payload:#?}");
    }

    Ok(())
}

#[tokio::test]
async fn account_reference_unlocks() -> Result<()> {
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

    let protocol_parameters = protocol_parameters();
    let account_id = AccountId::from_str(ACCOUNT_ID_1)?;
    let account_address = Address::Account(AccountAddress::new(account_id));

    let inputs = build_inputs([
        Account(
            1_000_000,
            account_id,
            address.clone(),
            None,
            None,
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        ),
        Basic(1_000_000, account_address.clone(), None, None, None, None, None, None),
        Basic(1_000_000, account_address.clone(), None, None, None, None, None, None),
    ]);

    let outputs = build_outputs([
        Account(1_000_000, account_id, address, None, None, None),
        Basic(2_000_000, account_address, None, None, None, None, None, None),
    ]);

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .add_mana_allotment(rand_mana_allotment(&protocol_parameters))
        .finish_with_params(&protocol_parameters)?;

    let prepared_transaction_data = PreparedTransactionData {
        transaction,
        inputs_data: inputs,
        remainder: None,
    };

    let unlocks = secret_manager
        .transaction_unlocks(&prepared_transaction_data, &protocol_parameters)
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

    let tx_payload = SignedTransactionPayload::new(prepared_transaction_data.transaction.clone(), unlocks)?;

    validate_signed_transaction_payload_length(&tx_payload)?;

    let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, protocol_parameters)?;

    if let Some(conflict) = conflict {
        panic!("{conflict:?}, with {tx_payload:#?}");
    }

    Ok(())
}