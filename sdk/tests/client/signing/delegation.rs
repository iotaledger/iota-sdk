// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        api::{GetAddressesOptions, PreparedTransactionData},
        constants::SHIMMER_COIN_TYPE,
        secret::{SecretManage, SecretManager},
        Client,
    },
    types::block::{
        context_input::{CommitmentContextInput, RewardContextInput},
        input::{Input, UtxoInput},
        output::DelegationId,
        payload::{
            signed_transaction::{Transaction, TransactionCapabilityFlag},
            PayloadError, SignedTransactionPayload,
        },
        protocol::iota_mainnet_protocol_parameters,
        rand::{address::rand_account_address, output::rand_delegation_id, slot::rand_slot_commitment_id},
        semantic::TransactionFailureReason,
        slot::SlotCommitmentHash,
        unlock::SignatureUnlock,
    },
};
use pretty_assertions::assert_eq;

use crate::client::{
    build_inputs, build_outputs,
    Build::{Basic, Delegation},
};

#[tokio::test]
async fn valid_creation() -> Result<(), Box<dyn std::error::Error>> {
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
    let slot_commitment_id = rand_slot_commitment_id();
    let slot_index = slot_commitment_id.slot_index();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                mana: 0,
                address: address.clone(),
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

    let outputs = build_outputs([Delegation {
        amount: 1_000_000,
        delegation_amount: 1_000_000,
        delegation_id: DelegationId::null(),
        address: address.clone(),
        validator_address: rand_account_address(),
        start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id),
        end_epoch: 0,
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
        .with_context_inputs([CommitmentContextInput::new(slot_commitment_id).into()])
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
async fn creation_missing_commitment_input() -> Result<(), Box<dyn std::error::Error>> {
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
    let slot_commitment_id = rand_slot_commitment_id();
    let slot_index = slot_commitment_id.slot_index();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                mana: 0,
                address: address.clone(),
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

    let outputs = build_outputs([Delegation {
        amount: 1_000_000,
        delegation_amount: 1_000_000,
        delegation_id: DelegationId::null(),
        address: address.clone(),
        validator_address: rand_account_address(),
        start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id),
        end_epoch: 0,
    }]);

    let err = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index + 1)
        .with_capabilities([TransactionCapabilityFlag::BurnMana])
        .finish_with_params(protocol_parameters)
        .unwrap_err();

    assert_eq!(err, PayloadError::MissingCommitmentInputForDelegationOutput);

    Ok(())
}

#[tokio::test]
async fn non_null_id_creation() -> Result<(), Box<dyn std::error::Error>> {
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
    let slot_commitment_id = rand_slot_commitment_id();
    let slot_index = slot_commitment_id.slot_index();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                mana: 0,
                address: address.clone(),
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

    let outputs = build_outputs([Delegation {
        amount: 1_000_000,
        delegation_amount: 1_000_000,
        delegation_id: rand_delegation_id(),
        address,
        validator_address: rand_account_address(),
        start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id),
        end_epoch: 0,
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
        .with_context_inputs([
            CommitmentContextInput::new(SlotCommitmentHash::null().into_slot_commitment_id(0)).into(),
        ])
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

    let conflict = prepared_transaction_data.verify_semantic(protocol_parameters);

    assert_eq!(conflict, Err(TransactionFailureReason::NewChainOutputHasNonZeroedId));

    Ok(())
}

#[tokio::test]
async fn mismatch_amount_creation() -> Result<(), Box<dyn std::error::Error>> {
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
    let slot_commitment_id = rand_slot_commitment_id();
    let slot_index = slot_commitment_id.slot_index();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                mana: 0,
                address: address.clone(),
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

    let outputs = build_outputs([Delegation {
        amount: 1_000_000,
        delegation_amount: 1_500_000,
        delegation_id: DelegationId::null(),
        address,
        validator_address: rand_account_address(),
        start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id),
        end_epoch: 0,
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
        .with_context_inputs([
            CommitmentContextInput::new(SlotCommitmentHash::null().into_slot_commitment_id(0)).into(),
        ])
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

    let conflict = prepared_transaction_data.verify_semantic(protocol_parameters);

    assert_eq!(conflict, Err(TransactionFailureReason::DelegationAmountMismatch));

    Ok(())
}

#[tokio::test]
async fn non_zero_end_epoch_creation() -> Result<(), Box<dyn std::error::Error>> {
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
    let slot_commitment_id = rand_slot_commitment_id();
    let slot_index = slot_commitment_id.slot_index();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                mana: 0,
                address: address.clone(),
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

    let outputs = build_outputs([Delegation {
        amount: 1_000_000,
        delegation_amount: 1_000_000,
        delegation_id: DelegationId::null(),
        address,
        validator_address: rand_account_address(),
        start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id),
        end_epoch: 100,
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
        .with_context_inputs([
            CommitmentContextInput::new(SlotCommitmentHash::null().into_slot_commitment_id(0)).into(),
        ])
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

    let conflict = prepared_transaction_data.verify_semantic(protocol_parameters);

    assert_eq!(conflict, Err(TransactionFailureReason::DelegationEndEpochNotZero));

    Ok(())
}

#[tokio::test]
async fn invalid_start_epoch_creation() -> Result<(), Box<dyn std::error::Error>> {
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
    let slot_commitment_id = rand_slot_commitment_id();
    let slot_index = slot_commitment_id.slot_index();

    let inputs = build_inputs(
        [(
            Basic {
                amount: 1_000_000,
                mana: 0,
                address: address.clone(),
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

    let outputs = build_outputs([Delegation {
        amount: 1_000_000,
        delegation_amount: 1_000_000,
        delegation_id: DelegationId::null(),
        address,
        validator_address: rand_account_address(),
        start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id) + 5,
        end_epoch: 0,
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
        .with_context_inputs([CommitmentContextInput::new(slot_commitment_id).into()])
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

    let conflict = prepared_transaction_data.verify_semantic(protocol_parameters);

    assert_eq!(conflict, Err(TransactionFailureReason::DelegationStartEpochInvalid));

    Ok(())
}

#[tokio::test]
async fn delay_not_null_id() -> Result<(), Box<dyn std::error::Error>> {
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
    let slot_commitment_id_1 = rand_slot_commitment_id();
    let slot_index_1 = slot_commitment_id_1.slot_index();
    let slot_commitment_id_2 = rand_slot_commitment_id()
        .hash()
        .into_slot_commitment_id(slot_index_1 + 100);
    let slot_index_2 = slot_commitment_id_2.slot_index();

    let validator_address = rand_account_address();

    let delegation_id = rand_delegation_id();

    let inputs = build_inputs(
        [(
            Delegation {
                amount: 1_000_000,
                delegation_amount: 1_000_000,
                delegation_id,
                address: address.clone(),
                validator_address,
                start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id_1),
                end_epoch: *protocol_parameters.delegation_end_epoch(slot_commitment_id_1),
            },
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        )],
        Some(slot_index_1),
    );

    let outputs = build_outputs([Delegation {
        amount: 1_000_000,
        delegation_amount: 1_000_000,
        delegation_id,
        address,
        validator_address,
        start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id_1),
        end_epoch: *protocol_parameters.delegation_end_epoch(slot_commitment_id_2),
    }]);

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index_2 + 1)
        .with_context_inputs([
            CommitmentContextInput::new(slot_commitment_id_2).into(),
            RewardContextInput::new(0)?.into(),
        ])
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

    let conflict = prepared_transaction_data.verify_semantic(protocol_parameters);

    assert_eq!(
        conflict,
        Err(TransactionFailureReason::DelegationOutputTransitionedTwice)
    );

    Ok(())
}

#[tokio::test]
async fn delay_modified_amount() -> Result<(), Box<dyn std::error::Error>> {
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
    let slot_commitment_id_1 = rand_slot_commitment_id();
    let slot_index_1 = slot_commitment_id_1.slot_index();
    let slot_commitment_id_2 = rand_slot_commitment_id()
        .hash()
        .into_slot_commitment_id(slot_index_1 + 100);
    let slot_index_2 = slot_commitment_id_2.slot_index();

    let validator_address = rand_account_address();

    let inputs = build_inputs(
        [(
            Delegation {
                amount: 1_000_000,
                delegation_amount: 1_000_000,
                delegation_id: DelegationId::null(),
                address: address.clone(),
                validator_address,
                start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id_1),
                end_epoch: 0,
            },
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        )],
        Some(slot_index_1),
    );

    let delegation_id = DelegationId::from(inputs[0].output_id());

    let outputs = build_outputs([Delegation {
        amount: 1_000_000,
        delegation_amount: 900_000,
        delegation_id,
        address,
        validator_address,
        start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id_1),
        end_epoch: *protocol_parameters.delegation_end_epoch(slot_commitment_id_2),
    }]);

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index_2 + 1)
        .with_context_inputs([
            CommitmentContextInput::new(slot_commitment_id_2).into(),
            RewardContextInput::new(0)?.into(),
        ])
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

    let conflict = prepared_transaction_data.verify_semantic(protocol_parameters);

    assert_eq!(conflict, Err(TransactionFailureReason::DelegationModified));

    Ok(())
}

#[tokio::test]
async fn delay_modified_validator() -> Result<(), Box<dyn std::error::Error>> {
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
    let slot_commitment_id_1 = rand_slot_commitment_id();
    let slot_index_1 = slot_commitment_id_1.slot_index();
    let slot_commitment_id_2 = rand_slot_commitment_id()
        .hash()
        .into_slot_commitment_id(slot_index_1 + 100);
    let slot_index_2 = slot_commitment_id_2.slot_index();

    let validator_address = rand_account_address();

    let inputs = build_inputs(
        [(
            Delegation {
                amount: 1_000_000,
                delegation_amount: 1_000_000,
                delegation_id: DelegationId::null(),
                address: address.clone(),
                validator_address,
                start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id_1),
                end_epoch: 0,
            },
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        )],
        Some(slot_index_1),
    );

    let delegation_id = DelegationId::from(inputs[0].output_id());

    let outputs = build_outputs([Delegation {
        amount: 1_000_000,
        delegation_amount: 1_000_000,
        delegation_id,
        address,
        validator_address: rand_account_address(),
        start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id_1),
        end_epoch: *protocol_parameters.delegation_end_epoch(slot_commitment_id_2),
    }]);

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index_2 + 1)
        .with_context_inputs([
            CommitmentContextInput::new(slot_commitment_id_2).into(),
            RewardContextInput::new(0)?.into(),
        ])
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

    let conflict = prepared_transaction_data.verify_semantic(protocol_parameters);

    assert_eq!(conflict, Err(TransactionFailureReason::DelegationModified));

    Ok(())
}

#[tokio::test]
async fn delay_modified_start_epoch() -> Result<(), Box<dyn std::error::Error>> {
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
    let slot_commitment_id_1 = rand_slot_commitment_id();
    let slot_index_1 = slot_commitment_id_1.slot_index();
    let slot_commitment_id_2 = rand_slot_commitment_id()
        .hash()
        .into_slot_commitment_id(slot_index_1 + 100);
    let slot_index_2 = slot_commitment_id_2.slot_index();

    let validator_address = rand_account_address();

    let inputs = build_inputs(
        [(
            Delegation {
                amount: 1_000_000,
                delegation_amount: 1_000_000,
                delegation_id: DelegationId::null(),
                address: address.clone(),
                validator_address,
                start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id_1),
                end_epoch: 0,
            },
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        )],
        Some(slot_index_1),
    );

    let delegation_id = DelegationId::from(inputs[0].output_id());

    let outputs = build_outputs([Delegation {
        amount: 1_000_000,
        delegation_amount: 1_000_000,
        delegation_id,
        address,
        validator_address,
        start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id_1) + 1,
        end_epoch: *protocol_parameters.delegation_end_epoch(slot_commitment_id_2),
    }]);

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index_2 + 1)
        .with_context_inputs([
            CommitmentContextInput::new(slot_commitment_id_2).into(),
            RewardContextInput::new(0)?.into(),
        ])
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

    let conflict = prepared_transaction_data.verify_semantic(protocol_parameters);

    assert_eq!(conflict, Err(TransactionFailureReason::DelegationModified));

    Ok(())
}

#[tokio::test]
async fn delay_pre_registration_slot_end_epoch() -> Result<(), Box<dyn std::error::Error>> {
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
    let slot_commitment_id_1 = rand_slot_commitment_id();
    let slot_index_1 = slot_commitment_id_1.slot_index();
    let slot_commitment_id_2 = rand_slot_commitment_id()
        .hash()
        .into_slot_commitment_id(slot_index_1 + 100);
    let slot_index_2 = slot_commitment_id_2.slot_index();

    let validator_address = rand_account_address();

    let inputs = build_inputs(
        [(
            Delegation {
                amount: 1_000_000,
                delegation_amount: 1_000_000,
                delegation_id: DelegationId::null(),
                address: address.clone(),
                validator_address,
                start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id_1),
                end_epoch: 0,
            },
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        )],
        Some(slot_index_1),
    );

    let delegation_id = DelegationId::from(inputs[0].output_id());

    let outputs = build_outputs([Delegation {
        amount: 1_000_000,
        delegation_amount: 1_000_000,
        delegation_id,
        address,
        validator_address,
        start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id_1),
        end_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id_1) + 1,
    }]);

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index_2 + 1)
        .with_context_inputs([
            CommitmentContextInput::new(slot_commitment_id_2).into(),
            RewardContextInput::new(0)?.into(),
        ])
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

    let conflict = prepared_transaction_data.verify_semantic(protocol_parameters);

    assert_eq!(conflict, Err(TransactionFailureReason::DelegationEndEpochInvalid));

    Ok(())
}

#[tokio::test]
async fn destroy_null_id() -> Result<(), Box<dyn std::error::Error>> {
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
    let slot_commitment_id_1 = rand_slot_commitment_id();
    let slot_index_1 = slot_commitment_id_1.slot_index();
    let slot_commitment_id_2 = rand_slot_commitment_id()
        .hash()
        .into_slot_commitment_id(slot_index_1 + 100);
    let slot_index_2 = slot_commitment_id_2.slot_index();

    let validator_address = rand_account_address();

    let inputs = build_inputs(
        [(
            Delegation {
                amount: 1_000_000,
                delegation_amount: 1_000_000,
                delegation_id: DelegationId::null(),
                address: address.clone(),
                validator_address,
                start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id_1),
                end_epoch: 0,
            },
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        )],
        Some(slot_index_1),
    );

    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        mana: 0,
        address,
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
        .with_creation_slot(slot_index_2 + 1)
        .with_context_inputs([
            CommitmentContextInput::new(slot_commitment_id_2).into(),
            RewardContextInput::new(0)?.into(),
        ])
        .with_capabilities([TransactionCapabilityFlag::BurnMana])
        .finish_with_params(protocol_parameters)?;

    let mut mana_rewards = BTreeMap::default();
    mana_rewards.insert(*inputs[0].output_id(), 0);

    let prepared_transaction_data = PreparedTransactionData {
        transaction,
        inputs_data: inputs,
        remainders: Vec::new(),
        mana_rewards,
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
async fn destroy_reward_missing() -> Result<(), Box<dyn std::error::Error>> {
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
    let slot_commitment_id_1 = rand_slot_commitment_id();
    let slot_index_1 = slot_commitment_id_1.slot_index();
    let slot_commitment_id_2 = rand_slot_commitment_id()
        .hash()
        .into_slot_commitment_id(slot_index_1 + 100);
    let slot_index_2 = slot_commitment_id_2.slot_index();

    let validator_address = rand_account_address();

    let inputs = build_inputs(
        [(
            Delegation {
                amount: 1_000_000,
                delegation_amount: 1_000_000,
                delegation_id: DelegationId::null(),
                address: address.clone(),
                validator_address,
                start_epoch: *protocol_parameters.delegation_start_epoch(slot_commitment_id_1),
                end_epoch: 0,
            },
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        )],
        Some(slot_index_1),
    );

    let outputs = build_outputs([Basic {
        amount: 1_000_000,
        mana: 0,
        address,
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
        .with_creation_slot(slot_index_2 + 1)
        .with_context_inputs([CommitmentContextInput::new(slot_commitment_id_2).into()])
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

    let conflict = prepared_transaction_data.verify_semantic(protocol_parameters);

    assert_eq!(conflict, Err(TransactionFailureReason::DelegationRewardInputMissing));

    Ok(())
}
