// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        api::{
            transaction::validate_transaction_payload_length, verify_semantic, GetAddressesOptions,
            PreparedTransactionData,
        },
        constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
        secret::{SecretManage, SecretManager},
        Client, Result,
    },
    types::block::{
        address::{Address, AliasAddress, ToBech32Ext},
        input::{Input, UtxoInput},
        output::{AliasId, InputsCommitment},
        payload::{
            transaction::{RegularTransactionEssence, TransactionEssence},
            TransactionPayload,
        },
        protocol::protocol_parameters,
        semantic::ConflictReason,
        unlock::{SignatureUnlock, Unlock},
    },
};

use crate::client::{
    build_inputs, build_outputs,
    Build::{Alias, Basic},
    ALIAS_ID_1,
};

#[tokio::test]
async fn sign_alias_state_transition() -> Result<()> {
    let secret_manager = SecretManager::try_from_mnemonic(Client::generate_mnemonic()?)?;

    let bech32_address_0 = &secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(0..1),
        )
        .await?[0]
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let bech32_address_1 = &secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(1..2),
        )
        .await?[0]
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    let protocol_parameters = protocol_parameters();
    let alias_id = AliasId::from_str(ALIAS_ID_1)?;

    let inputs = build_inputs([Alias(
        1_000_000,
        alias_id,
        0,
        &bech32_address_0.to_string(),
        &bech32_address_1.to_string(),
        None,
        None,
        None,
        Some(Bip44::new(SHIMMER_COIN_TYPE)),
    )]);

    let outputs = build_outputs([Alias(
        1_000_000,
        alias_id,
        1,
        &bech32_address_0.to_string(),
        &bech32_address_1.to_string(),
        None,
        None,
        None,
        None,
    )]);

    let essence = TransactionEssence::Regular(
        RegularTransactionEssence::builder(
            protocol_parameters.network_id(),
            InputsCommitment::new(inputs.iter().map(|i| &i.output)),
        )
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .finish(&protocol_parameters)?,
    );

    let prepared_transaction_data = PreparedTransactionData {
        essence,
        inputs_data: inputs,
        remainder: None,
    };

    let unlocks = secret_manager
        .sign_transaction_essence(&prepared_transaction_data, Some(0))
        .await?;

    assert_eq!(unlocks.len(), 1);
    assert_eq!((*unlocks).get(0).unwrap().kind(), SignatureUnlock::KIND);

    let tx_payload = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlocks)?;

    validate_transaction_payload_length(&tx_payload)?;

    let current_time = 100;

    let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, current_time)?;

    if conflict != ConflictReason::None {
        panic!("{conflict:?}, with {tx_payload:#?}");
    }

    Ok(())
}

#[tokio::test]
async fn sign_alias_governance_transition() -> Result<()> {
    let secret_manager = SecretManager::try_from_mnemonic(Client::generate_mnemonic()?)?;

    let bech32_address_0 = &secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(0..1),
        )
        .await?[0]
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let bech32_address_1 = &secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(1..2),
        )
        .await?[0]
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    let protocol_parameters = protocol_parameters();
    let alias_id = AliasId::from_str(ALIAS_ID_1)?;

    let inputs = build_inputs([Alias(
        1_000_000,
        alias_id,
        0,
        &bech32_address_0.to_string(),
        &bech32_address_1.to_string(),
        None,
        None,
        None,
        Some(Bip44::new(SHIMMER_COIN_TYPE).with_address_index(1)),
    )]);

    let outputs = build_outputs([Alias(
        1_000_000,
        alias_id,
        0,
        &bech32_address_0.to_string(),
        &bech32_address_1.to_string(),
        None,
        None,
        None,
        None,
    )]);

    let essence = TransactionEssence::Regular(
        RegularTransactionEssence::builder(
            protocol_parameters.network_id(),
            InputsCommitment::new(inputs.iter().map(|i| &i.output)),
        )
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .finish(&protocol_parameters)?,
    );

    let prepared_transaction_data = PreparedTransactionData {
        essence,
        inputs_data: inputs,
        remainder: None,
    };

    let unlocks = secret_manager
        .sign_transaction_essence(&prepared_transaction_data, Some(0))
        .await?;

    assert_eq!(unlocks.len(), 1);
    assert_eq!((*unlocks).get(0).unwrap().kind(), SignatureUnlock::KIND);

    let tx_payload = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlocks)?;

    validate_transaction_payload_length(&tx_payload)?;

    let current_time = 100;

    let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, current_time)?;

    if conflict != ConflictReason::None {
        panic!("{conflict:?}, with {tx_payload:#?}");
    }

    Ok(())
}

#[tokio::test]
async fn alias_reference_unlocks() -> Result<()> {
    let secret_manager = SecretManager::try_from_mnemonic(Client::generate_mnemonic()?)?;

    let bech32_address_0 = &secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(0..1),
        )
        .await?[0]
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let bech32_address_1 = &secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(1..2),
        )
        .await?[0]
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    let protocol_parameters = protocol_parameters();
    let alias_id = AliasId::from_str(ALIAS_ID_1)?;
    let alias_bech32_address = &Address::Alias(AliasAddress::new(alias_id)).to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    let inputs = build_inputs([
        Alias(
            1_000_000,
            alias_id,
            0,
            &bech32_address_0.to_string(),
            &bech32_address_1.to_string(),
            None,
            None,
            None,
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        ),
        Basic(
            1_000_000,
            &alias_bech32_address.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            &alias_bech32_address.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);

    let outputs = build_outputs([
        Alias(
            1_000_000,
            alias_id,
            1,
            &bech32_address_0.to_string(),
            &bech32_address_1.to_string(),
            None,
            None,
            None,
            None,
        ),
        Basic(
            2_000_000,
            &alias_bech32_address.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);

    let essence = TransactionEssence::Regular(
        RegularTransactionEssence::builder(
            protocol_parameters.network_id(),
            InputsCommitment::new(inputs.iter().map(|i| &i.output)),
        )
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .finish(&protocol_parameters)?,
    );

    let prepared_transaction_data = PreparedTransactionData {
        essence,
        inputs_data: inputs,
        remainder: None,
    };

    let unlocks = secret_manager
        .sign_transaction_essence(&prepared_transaction_data, Some(0))
        .await?;

    assert_eq!(unlocks.len(), 3);
    assert_eq!((*unlocks).get(0).unwrap().kind(), SignatureUnlock::KIND);
    match (*unlocks).get(1).unwrap() {
        Unlock::Alias(a) => {
            assert_eq!(a.index(), 0);
        }
        _ => panic!("Invalid unlock"),
    }
    match (*unlocks).get(2).unwrap() {
        Unlock::Alias(a) => {
            assert_eq!(a.index(), 0);
        }
        _ => panic!("Invalid unlock"),
    }

    let tx_payload = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlocks)?;

    validate_transaction_payload_length(&tx_payload)?;

    let current_time = 100;

    let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, current_time)?;

    if conflict != ConflictReason::None {
        panic!("{conflict:?}, with {tx_payload:#?}");
    }

    Ok(())
}
