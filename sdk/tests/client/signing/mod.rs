// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod basic;
mod nft;

use std::str::FromStr;

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        api::{
            input_selection::InputSelection, transaction::validate_signed_transaction_payload_length, verify_semantic,
            GetAddressesOptions, PreparedTransactionData,
        },
        constants::SHIMMER_COIN_TYPE,
        secret::{SecretManage, SecretManager},
        Result,
    },
    types::block::{
        address::{AccountAddress, Address, NftAddress},
        context_input::{CommitmentContextInput, ContextInput},
        input::{Input, UtxoInput},
        output::{AccountId, NftId},
        payload::{signed_transaction::Transaction, SignedTransactionPayload},
        protocol::protocol_parameters,
        slot::{SlotCommitmentId, SlotIndex},
        unlock::{SignatureUnlock, Unlock},
    },
};
use pretty_assertions::assert_eq;

use crate::client::{
    build_inputs, build_outputs,
    Build::{Account, Basic, Nft},
    ACCOUNT_ID_1, ACCOUNT_ID_2, NFT_ID_1, NFT_ID_2, NFT_ID_3, NFT_ID_4,
};

#[tokio::test]
async fn all_combined() -> Result<()> {
    let secret_manager = SecretManager::try_from_mnemonic(
        // mnemonic needs to be hardcoded to make the ordering deterministic
        "mirror add nothing long orphan hat this rough scare gallery fork twelve old shrug voyage job table obscure mimic holiday possible proud giraffe fan".to_owned(),
    )?;

    let protocol_parameters = protocol_parameters();

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

    let inputs = build_inputs(
        [
            Account(1_000_000, account_id_1, nft_1.clone(), None, None, None),
            Account(
                1_000_000,
                account_id_2,
                ed25519_0.clone(),
                None,
                None,
                Some(Bip44::new(SHIMMER_COIN_TYPE)),
            ),
            Basic(1_000_000, account_1.clone(), None, None, None, None, None, None),
            Basic(1_000_000, account_2.clone(), None, None, None, None, None, None),
            Basic(1_000_000, account_2, None, None, None, None, None, None),
            Basic(1_000_000, nft_2.clone(), None, None, None, None, None, None),
            Basic(1_000_000, nft_2, None, None, None, None, None, None),
            Basic(1_000_000, nft_4.clone(), None, None, None, None, None, None),
            Basic(
                1_000_000,
                ed25519_0.clone(),
                None,
                None,
                None,
                None,
                None,
                Some(Bip44::new(SHIMMER_COIN_TYPE)),
            ),
            Basic(
                1_000_000,
                ed25519_1.clone(),
                None,
                None,
                None,
                None,
                None,
                Some(Bip44::new(SHIMMER_COIN_TYPE).with_address_index(1)),
            ),
            Basic(
                1_000_000,
                ed25519_2.clone(),
                None,
                None,
                None,
                None,
                None,
                Some(Bip44::new(SHIMMER_COIN_TYPE).with_address_index(2)),
            ),
            Basic(
                1_000_000,
                ed25519_2.clone(),
                None,
                None,
                None,
                None,
                None,
                Some(Bip44::new(SHIMMER_COIN_TYPE).with_address_index(2)),
            ),
            Nft(
                1_000_000,
                nft_id_1,
                ed25519_0.clone(),
                None,
                None,
                None,
                None,
                Some(Bip44::new(SHIMMER_COIN_TYPE)),
            ),
            Nft(1_000_000, nft_id_2, account_1.clone(), None, None, None, None, None),
            // Expirations
            Basic(
                2_000_000,
                ed25519_0.clone(),
                None,
                None,
                None,
                None,
                Some((account_1.clone(), 50)),
                None,
            ),
            Basic(
                2_000_000,
                ed25519_0.clone(),
                None,
                None,
                None,
                None,
                Some((nft_3.clone(), 50)),
                None,
            ),
            Basic(
                2_000_000,
                ed25519_0.clone(),
                None,
                None,
                None,
                None,
                Some((nft_3.clone(), 150)),
                Some(Bip44::new(SHIMMER_COIN_TYPE)),
            ),
            Nft(
                1_000_000,
                nft_id_3,
                account_1.clone(),
                None,
                None,
                None,
                Some((nft_4, 50)),
                None,
            ),
            Nft(
                1_000_000,
                nft_id_4,
                account_1,
                None,
                None,
                None,
                Some((nft_3, 150)),
                None,
            ),
        ],
        Some(slot_index),
    );

    let outputs = build_outputs([
        Account(1_000_000, account_id_1, nft_1, None, None, None),
        Account(1_000_000, account_id_2, ed25519_0.clone(), None, None, None),
        Basic(10_000_000, ed25519_0.clone(), None, None, None, None, None, None),
        Nft(1_000_000, nft_id_1, ed25519_0.clone(), None, None, None, None, None),
        Nft(1_000_000, nft_id_2, ed25519_0.clone(), None, None, None, None, None),
        Nft(1_000_000, nft_id_3, ed25519_0.clone(), None, None, None, None, None),
        Nft(1_000_000, nft_id_4, ed25519_0.clone(), None, None, None, None, None),
    ]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [ed25519_0, ed25519_1, ed25519_2],
        slot_index,
        protocol_parameters.clone(),
    )
    .select()
    .unwrap();

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_context_inputs(vec![ContextInput::Commitment(CommitmentContextInput::new(
            SlotCommitmentId::from_str("0x000000000000000000000000000000000000000000000000000000000000000064000000")?,
        ))])
        .with_inputs(
            selected
                .inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .with_creation_slot(slot_index)
        .finish_with_params(&protocol_parameters)?;

    let prepared_transaction_data = PreparedTransactionData {
        transaction,
        inputs_data: selected.inputs,
        remainders: Vec::new(),
        mana_rewards: Default::default(),
    };

    let unlocks = secret_manager
        .transaction_unlocks(&prepared_transaction_data, &protocol_parameters)
        .await?;

    assert_eq!(unlocks.len(), 15);
    assert_eq!((*unlocks).first().unwrap().kind(), SignatureUnlock::KIND);
    match (*unlocks).get(1).unwrap() {
        Unlock::Reference(a) => {
            assert_eq!(a.index(), 0);
        }
        _ => panic!("Invalid unlock 1"),
    }
    assert_eq!((*unlocks).get(2).unwrap().kind(), SignatureUnlock::KIND);
    assert_eq!((*unlocks).get(3).unwrap().kind(), SignatureUnlock::KIND);
    match (*unlocks).get(4).unwrap() {
        Unlock::Reference(a) => {
            assert_eq!(a.index(), 3);
        }
        _ => panic!("Invalid unlock 4"),
    }
    match (*unlocks).get(5).unwrap() {
        Unlock::Reference(a) => {
            assert_eq!(a.index(), 3);
        }
        _ => panic!("Invalid unlock 5"),
    }
    match (*unlocks).get(6).unwrap() {
        Unlock::Account(a) => {
            assert_eq!(a.index(), 5);
        }
        _ => panic!("Invalid unlock 6"),
    }
    match (*unlocks).get(7).unwrap() {
        Unlock::Account(a) => {
            assert_eq!(a.index(), 5);
        }
        _ => panic!("Invalid unlock 7"),
    }
    match (*unlocks).get(8).unwrap() {
        Unlock::Reference(a) => {
            assert_eq!(a.index(), 3);
        }
        _ => panic!("Invalid unlock 8"),
    }

    match (*unlocks).get(9).unwrap() {
        Unlock::Nft(a) => {
            assert_eq!(a.index(), 8);
        }
        _ => panic!("Invalid unlock 9"),
    }
    match (*unlocks).get(10).unwrap() {
        Unlock::Account(a) => {
            assert_eq!(a.index(), 9);
        }
        _ => panic!("Invalid unlock 10"),
    }
    match (*unlocks).get(11).unwrap() {
        Unlock::Account(a) => {
            assert_eq!(a.index(), 9);
        }
        _ => panic!("Invalid unlock 11"),
    }
    match (*unlocks).get(12).unwrap() {
        Unlock::Account(a) => {
            assert_eq!(a.index(), 9);
        }
        _ => panic!("Invalid unlock 12"),
    }
    match (*unlocks).get(13).unwrap() {
        Unlock::Nft(a) => {
            assert_eq!(a.index(), 11);
        }
        _ => panic!("Invalid unlock 13"),
    }
    match (*unlocks).get(14).unwrap() {
        Unlock::Nft(a) => {
            assert_eq!(a.index(), 10);
        }
        _ => panic!("Invalid unlock 14"),
    }

    let tx_payload = SignedTransactionPayload::new(prepared_transaction_data.transaction.clone(), unlocks)?;

    validate_signed_transaction_payload_length(&tx_payload)?;

    let conflict = verify_semantic(
        &prepared_transaction_data.inputs_data,
        &tx_payload,
        prepared_transaction_data.mana_rewards,
        protocol_parameters,
    )?;

    if let Some(conflict) = conflict {
        panic!("{conflict:?}, with {tx_payload:#?}");
    }

    Ok(())
}
