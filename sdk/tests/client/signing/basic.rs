// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        api::{transaction::validate_signed_transaction_payload_length, verify_semantic, PreparedTransactionData},
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, PublicKeyOptions, SecretManageExt, SignTransaction},
        Client, Result,
    },
    types::block::{
        address::{Address, Ed25519Address},
        input::{Input, UtxoInput},
        payload::{signed_transaction::Transaction, SignedTransactionPayload},
        protocol::protocol_parameters,
        slot::SlotIndex,
        unlock::{SignatureUnlock, Unlock},
    },
};
use pretty_assertions::assert_eq;

use crate::client::{build_inputs, build_outputs, Build::Basic};

#[tokio::test]
async fn single_ed25519_unlock() -> Result<()> {
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(Client::generate_mnemonic()?)?;

    let address_0 = Address::from(
        secret_manager
            .generate::<Ed25519Address>(&PublicKeyOptions::new(SHIMMER_COIN_TYPE))
            .await?,
    );

    let protocol_parameters = protocol_parameters();
    let slot_index = SlotIndex::from(10);

    let inputs = build_inputs(
        [Basic(1_000_000, address_0.clone(), None, None, None, None, None)],
        Some(slot_index),
    );

    let outputs = build_outputs([Basic(1_000_000, address_0, None, None, None, None, None)]);

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index + 1)
        .finish_with_params(&protocol_parameters)?;

    let prepared_transaction_data = PreparedTransactionData {
        transaction,
        inputs_data: inputs,
        remainder: None,
    };

    let signing_options = Bip44::new(SHIMMER_COIN_TYPE);

    let unlocks = secret_manager
        .transaction_unlocks(&prepared_transaction_data, &protocol_parameters, &signing_options)
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
async fn ed25519_reference_unlocks() -> Result<()> {
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(Client::generate_mnemonic()?)?;

    let address_0 = Address::from(
        secret_manager
            .generate::<Ed25519Address>(&PublicKeyOptions::new(SHIMMER_COIN_TYPE))
            .await?,
    );

    let protocol_parameters = protocol_parameters();
    let slot_index = SlotIndex::from(10);

    let inputs = build_inputs(
        [
            Basic(1_000_000, address_0.clone(), None, None, None, None, None),
            Basic(1_000_000, address_0.clone(), None, None, None, None, None),
            Basic(1_000_000, address_0.clone(), None, None, None, None, None),
        ],
        Some(slot_index),
    );

    let outputs = build_outputs([Basic(3_000_000, address_0, None, None, None, None, None)]);

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index + 1)
        .finish_with_params(&protocol_parameters)?;

    let prepared_transaction_data = PreparedTransactionData {
        transaction,
        inputs_data: inputs,
        remainder: None,
    };

    let signing_options = Bip44::new(SHIMMER_COIN_TYPE);

    let unlocks = secret_manager
        .transaction_unlocks(&prepared_transaction_data, &protocol_parameters, &signing_options)
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

    let tx_payload = SignedTransactionPayload::new(prepared_transaction_data.transaction.clone(), unlocks)?;

    validate_signed_transaction_payload_length(&tx_payload)?;

    let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, protocol_parameters)?;

    if let Some(conflict) = conflict {
        panic!("{conflict:?}, with {tx_payload:#?}");
    }

    Ok(())
}

#[tokio::test]
async fn two_signature_unlocks() -> Result<()> {
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(Client::generate_mnemonic()?)?;

    let address_0 = Address::from(
        secret_manager
            .generate::<Ed25519Address>(&PublicKeyOptions::new(SHIMMER_COIN_TYPE))
            .await?,
    );
    let address_1 = Address::from(
        secret_manager
            .generate::<Ed25519Address>(&PublicKeyOptions::new(SHIMMER_COIN_TYPE).with_address_index(1))
            .await?,
    );

    let protocol_parameters = protocol_parameters();
    let slot_index = SlotIndex::from(10);

    let inputs = build_inputs(
        [
            Basic(1_000_000, address_0.clone(), None, None, None, None, None),
            Basic(1_000_000, address_1, None, None, None, None, None),
        ],
        Some(slot_index),
    );

    let outputs = build_outputs([Basic(2_000_000, address_0, None, None, None, None, None)]);

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index + 1)
        .finish_with_params(&protocol_parameters)?;

    let prepared_transaction_data = PreparedTransactionData {
        transaction,
        inputs_data: inputs,
        remainder: None,
    };

    let signing_options = Bip44::new(SHIMMER_COIN_TYPE);

    let unlocks = secret_manager
        .transaction_unlocks(&prepared_transaction_data, &protocol_parameters, &signing_options)
        .await?;

    assert_eq!(unlocks.len(), 2);
    assert_eq!((*unlocks).first().unwrap().kind(), SignatureUnlock::KIND);
    assert_eq!((*unlocks).get(1).unwrap().kind(), SignatureUnlock::KIND);

    let tx_payload = SignedTransactionPayload::new(prepared_transaction_data.transaction.clone(), unlocks)?;

    validate_signed_transaction_payload_length(&tx_payload)?;

    let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, protocol_parameters)?;

    if let Some(conflict) = conflict {
        panic!("{conflict:?}, with {tx_payload:#?}");
    }

    Ok(())
}
