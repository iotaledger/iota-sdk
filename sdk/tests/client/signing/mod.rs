// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod basic;
mod delegation;
mod nft;

use std::str::FromStr;

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        api::{transaction_builder::TransactionBuilder, GetAddressesOptions, PreparedTransactionData},
        constants::SHIMMER_COIN_TYPE,
        secret::{SecretManage, SecretManager},
    },
    types::block::{
        address::{AccountAddress, Address, NftAddress},
        context_input::{CommitmentContextInput, ContextInput},
        input::{Input, UtxoInput},
        output::{AccountId, NftId},
        payload::{signed_transaction::Transaction, SignedTransactionPayload},
        protocol::iota_mainnet_protocol_parameters,
        slot::{SlotCommitmentHash, SlotCommitmentId, SlotIndex},
    },
};
use pretty_assertions::assert_eq;

use crate::client::{
    build_inputs, build_outputs,
    Build::{Account, Basic, Nft},
    ACCOUNT_ID_1, ACCOUNT_ID_2, NFT_ID_1, NFT_ID_2, NFT_ID_3, NFT_ID_4,
};

#[tokio::test]
async fn all_combined() -> Result<(), Box<dyn std::error::Error>> {
    let secret_manager = SecretManager::try_from_mnemonic(
        // mnemonic needs to be hardcoded to make the ordering deterministic
        "mirror add nothing long orphan hat this rough scare gallery fork twelve old shrug voyage job table obscure mimic holiday possible proud giraffe fan".to_owned(),
    )?;

    let protocol_parameters = iota_mainnet_protocol_parameters();

    let ed25519_bech32_addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(0..3),
        )
        .await?;
    let ed25519_0 = ed25519_bech32_addresses[0].clone().into_inner();
    let ed25519_1 = ed25519_bech32_addresses[1].clone().into_inner();
    let ed25519_2 = ed25519_bech32_addresses[2].clone().into_inner();

    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1)?;
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2)?;
    let account_1 = Address::Account(AccountAddress::new(account_id_1));
    let account_2 = Address::Account(AccountAddress::new(account_id_2));

    let nft_id_1 = NftId::from_str(NFT_ID_1)?;
    let nft_id_2 = NftId::from_str(NFT_ID_2)?;
    let nft_id_3 = NftId::from_str(NFT_ID_3)?;
    let nft_id_4 = NftId::from_str(NFT_ID_4)?;
    let nft_1 = Address::Nft(NftAddress::new(nft_id_1));
    let nft_2 = Address::Nft(NftAddress::new(nft_id_2));
    let nft_3 = Address::Nft(NftAddress::new(nft_id_3));
    let nft_4 = Address::Nft(NftAddress::new(nft_id_4));

    let slot_index = SlotIndex::from(90);
    let slot_commitment_id = SlotCommitmentHash::null().into_slot_commitment_id(89);

    let inputs = build_inputs(
        [
            (
                Account {
                    amount: 1_000_000,
                    mana: 0,
                    account_id: account_id_1,
                    address: nft_1.clone(),
                    sender: None,
                    issuer: None,
                },
                None,
            ),
            (
                Account {
                    amount: 1_000_000,
                    mana: 0,
                    account_id: account_id_2,
                    address: ed25519_0.clone(),
                    sender: None,
                    issuer: None,
                },
                Some(Bip44::new(SHIMMER_COIN_TYPE)),
            ),
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: account_1.clone(),
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
                    address: account_2.clone(),
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
                    address: account_2,
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
                    address: nft_2.clone(),
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
                    address: nft_2,
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
                    address: nft_4.clone(),
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
                    address: ed25519_0.clone(),
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
                    address: ed25519_1.clone(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                Some(Bip44::new(SHIMMER_COIN_TYPE).with_address_index(1)),
            ),
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: ed25519_2.clone(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                Some(Bip44::new(SHIMMER_COIN_TYPE).with_address_index(2)),
            ),
            (
                Basic {
                    amount: 1_000_000,
                    mana: 0,
                    address: ed25519_2.clone(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: None,
                },
                Some(Bip44::new(SHIMMER_COIN_TYPE).with_address_index(2)),
            ),
            (
                Nft {
                    amount: 1_000_000,
                    mana: 0,
                    nft_id: nft_id_1,
                    address: ed25519_0.clone(),
                    sender: None,
                    issuer: None,
                    sdruc: None,
                    expiration: None,
                },
                Some(Bip44::new(SHIMMER_COIN_TYPE)),
            ),
            (
                Nft {
                    amount: 1_000_000,
                    mana: 0,
                    nft_id: nft_id_2,
                    address: account_1.clone(),
                    sender: None,
                    issuer: None,
                    sdruc: None,
                    expiration: None,
                },
                None,
            ),
            // Expirations
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: ed25519_0.clone(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: Some((account_1.clone(), 50)),
                },
                None,
            ),
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: ed25519_0.clone(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: Some((nft_3.clone(), 50)),
                },
                None,
            ),
            (
                Basic {
                    amount: 2_000_000,
                    mana: 0,
                    address: ed25519_0.clone(),
                    native_token: None,
                    sender: None,
                    sdruc: None,
                    timelock: None,
                    expiration: Some((nft_3.clone(), 150)),
                },
                Some(Bip44::new(SHIMMER_COIN_TYPE)),
            ),
            (
                Nft {
                    amount: 1_000_000,
                    mana: 0,
                    nft_id: nft_id_3,
                    address: account_1.clone(),
                    sender: None,
                    issuer: None,
                    sdruc: None,
                    expiration: Some((nft_4, 50)),
                },
                None,
            ),
            (
                Nft {
                    amount: 1_000_000,
                    mana: 0,
                    nft_id: nft_id_4,
                    address: account_1,
                    sender: None,
                    issuer: None,
                    sdruc: None,
                    expiration: Some((nft_3, 150)),
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
            account_id: account_id_1,
            address: nft_1,
            sender: None,
            issuer: None,
        },
        Account {
            amount: 1_000_000,
            mana: 0,
            account_id: account_id_2,
            address: ed25519_0.clone(),
            sender: None,
            issuer: None,
        },
        Basic {
            amount: 10_000_000,
            mana: 0,
            address: ed25519_0.clone(),
            native_token: None,
            sender: None,
            sdruc: None,
            timelock: None,
            expiration: None,
        },
        Nft {
            amount: 1_000_000,
            mana: 0,
            nft_id: nft_id_1,
            address: ed25519_0.clone(),
            sender: None,
            issuer: None,
            sdruc: None,
            expiration: None,
        },
        Nft {
            amount: 1_000_000,
            mana: 0,
            nft_id: nft_id_2,
            address: ed25519_0.clone(),
            sender: None,
            issuer: None,
            sdruc: None,
            expiration: None,
        },
        Nft {
            amount: 1_000_000,
            mana: 0,
            nft_id: nft_id_3,
            address: ed25519_0.clone(),
            sender: None,
            issuer: None,
            sdruc: None,
            expiration: None,
        },
        Nft {
            amount: 1_000_000,
            mana: 0,
            nft_id: nft_id_4,
            address: ed25519_0.clone(),
            sender: None,
            issuer: None,
            sdruc: None,
            expiration: None,
        },
    ]);

    let selected = TransactionBuilder::new(
        inputs.clone(),
        outputs.clone(),
        [ed25519_0, ed25519_1, ed25519_2],
        slot_index,
        slot_commitment_id,
        protocol_parameters.clone(),
    )
    .finish()
    .unwrap();

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_context_inputs(vec![ContextInput::Commitment(CommitmentContextInput::new(
            SlotCommitmentId::from_str("0x000000000000000000000000000000000000000000000000000000000000000064000000")?,
        ))])
        .with_inputs(
            selected
                .inputs_data
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index)
        .finish_with_params(protocol_parameters)?;

    let prepared_transaction_data = PreparedTransactionData {
        transaction,
        inputs_data: selected.inputs_data,
        remainders: Vec::new(),
        mana_rewards: Default::default(),
        issuer_id: None,
    };

    let unlocks = secret_manager
        .transaction_unlocks(&prepared_transaction_data, protocol_parameters)
        .await?;

    assert_eq!(unlocks.len(), 13);
    assert_eq!(unlocks.iter().filter(|u| u.is_signature()).count(), 2);
    assert_eq!(unlocks.iter().filter(|u| u.is_reference()).count(), 3);
    assert_eq!(unlocks.iter().filter(|u| u.is_account()).count(), 3);
    assert_eq!(unlocks.iter().filter(|u| u.is_nft()).count(), 5);

    SignedTransactionPayload::new(prepared_transaction_data.transaction.clone(), unlocks)?;

    prepared_transaction_data.verify_semantic(protocol_parameters)?;

    Ok(())
}
