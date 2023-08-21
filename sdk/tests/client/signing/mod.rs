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
            input_selection::InputSelection, transaction::validate_transaction_payload_length, verify_semantic,
            GetAddressesOptions, PreparedTransactionData,
        },
        constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
        secret::{SecretManage, SecretManager},
        Result,
    },
    types::block::{
        address::{AccountAddress, Address, NftAddress, ToBech32Ext},
        input::{Input, UtxoInput},
        output::{AccountId, InputsCommitment, NftId},
        payload::{
            transaction::{RegularTransactionEssence, TransactionEssence},
            TransactionPayload,
        },
        protocol::protocol_parameters,
        rand::mana::rand_mana_allotment,
        semantic::ConflictReason,
        unlock::{SignatureUnlock, Unlock},
    },
};

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
    let ed25519_bech32_address_0 = &ed25519_bech32_addresses[0].to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let ed25519_bech32_address_1 = &ed25519_bech32_addresses[1].to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let ed25519_bech32_address_2 = &ed25519_bech32_addresses[2].to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1)?;
    let account_id_2 = AccountId::from_str(ACCOUNT_ID_2)?;
    let account_1_bech32_address =
        &Address::Account(AccountAddress::new(account_id_1)).to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let account_2_bech32_address =
        &Address::Account(AccountAddress::new(account_id_2)).to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    let nft_id_1 = NftId::from_str(NFT_ID_1)?;
    let nft_id_2 = NftId::from_str(NFT_ID_2)?;
    let nft_id_3 = NftId::from_str(NFT_ID_3)?;
    let nft_id_4 = NftId::from_str(NFT_ID_4)?;
    let nft_1_bech32_address = &Address::Nft(NftAddress::new(nft_id_1)).to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let nft_2_bech32_address = &Address::Nft(NftAddress::new(nft_id_2)).to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let nft_3_bech32_address = &Address::Nft(NftAddress::new(nft_id_3)).to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let nft_4_bech32_address = &Address::Nft(NftAddress::new(nft_id_4)).to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    let inputs = build_inputs([
        Account(
            1_000_000,
            account_id_1,
            0,
            &nft_1_bech32_address.to_string(),
            &nft_1_bech32_address.to_string(),
            None,
            None,
            None,
            None,
        ),
        Account(
            1_000_000,
            account_id_2,
            0,
            &ed25519_bech32_address_0.to_string(),
            &ed25519_bech32_address_1.to_string(),
            None,
            None,
            None,
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        ),
        Basic(
            1_000_000,
            &account_1_bech32_address.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            &account_2_bech32_address.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            &account_2_bech32_address.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            &nft_2_bech32_address.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            &nft_2_bech32_address.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            &nft_4_bech32_address.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            &ed25519_bech32_address_0.to_string(),
            None,
            None,
            None,
            None,
            None,
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        ),
        Basic(
            1_000_000,
            &ed25519_bech32_address_1.to_string(),
            None,
            None,
            None,
            None,
            None,
            Some(Bip44::new(SHIMMER_COIN_TYPE).with_address_index(1)),
        ),
        Basic(
            1_000_000,
            &ed25519_bech32_address_2.to_string(),
            None,
            None,
            None,
            None,
            None,
            Some(Bip44::new(SHIMMER_COIN_TYPE).with_address_index(2)),
        ),
        Basic(
            1_000_000,
            &ed25519_bech32_address_2.to_string(),
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
            &ed25519_bech32_address_0.to_string(),
            None,
            None,
            None,
            None,
            None,
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        ),
        Nft(
            1_000_000,
            nft_id_2,
            &account_1_bech32_address.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        // Expirations
        Basic(
            2_000_000,
            &ed25519_bech32_address_0.to_string(),
            None,
            None,
            None,
            None,
            Some((&account_1_bech32_address.to_string(), 50)),
            None,
        ),
        Basic(
            2_000_000,
            &ed25519_bech32_address_0.to_string(),
            None,
            None,
            None,
            None,
            Some((&nft_3_bech32_address.to_string(), 50)),
            None,
        ),
        Basic(
            2_000_000,
            &ed25519_bech32_address_0.to_string(),
            None,
            None,
            None,
            None,
            Some((&nft_3_bech32_address.to_string(), 150)),
            Some(Bip44::new(SHIMMER_COIN_TYPE)),
        ),
        Nft(
            1_000_000,
            nft_id_3,
            &account_1_bech32_address.to_string(),
            None,
            None,
            None,
            None,
            Some((&nft_4_bech32_address.to_string(), 50)),
            None,
        ),
        Nft(
            1_000_000,
            nft_id_4,
            &account_1_bech32_address.to_string(),
            None,
            None,
            None,
            None,
            Some((&nft_3_bech32_address.to_string(), 150)),
            None,
        ),
    ]);

    let outputs = build_outputs([
        Account(
            1_000_000,
            account_id_1,
            1,
            &nft_1_bech32_address.to_string(),
            &nft_1_bech32_address.to_string(),
            None,
            None,
            None,
            None,
        ),
        Account(
            1_000_000,
            account_id_2,
            1,
            &ed25519_bech32_address_0.to_string(),
            &ed25519_bech32_address_1.to_string(),
            None,
            None,
            None,
            None,
        ),
        Basic(
            10_000_000,
            &ed25519_bech32_address_0.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Nft(
            1_000_000,
            nft_id_1,
            &ed25519_bech32_address_0.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Nft(
            1_000_000,
            nft_id_2,
            &ed25519_bech32_address_0.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Nft(
            1_000_000,
            nft_id_3,
            &ed25519_bech32_address_0.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Nft(
            1_000_000,
            nft_id_4,
            &ed25519_bech32_address_0.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);

    let current_time = 100;

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [
            *ed25519_bech32_address_0.inner(),
            *ed25519_bech32_address_1.inner(),
            *ed25519_bech32_address_2.inner(),
        ],
        protocol_parameters.clone(),
    )
    .timestamp(current_time)
    .select()
    .unwrap();

    let essence = TransactionEssence::Regular(
        RegularTransactionEssence::builder(
            protocol_parameters.network_id(),
            InputsCommitment::new(selected.inputs.iter().map(|i| &i.output)),
        )
        .with_inputs(
            selected
                .inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect::<Vec<_>>(),
        )
        .with_outputs(outputs)
        .add_mana_allotment(rand_mana_allotment(10))
        .finish_with_params(protocol_parameters)?,
    );

    let prepared_transaction_data = PreparedTransactionData {
        essence,
        inputs_data: selected.inputs,
        remainder: None,
    };

    let unlocks = secret_manager
        .sign_transaction_essence(&prepared_transaction_data, Some(current_time))
        .await?;

    assert_eq!(unlocks.len(), 15);
    assert_eq!((*unlocks).get(0).unwrap().kind(), SignatureUnlock::KIND);
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

    let tx_payload = TransactionPayload::new(prepared_transaction_data.essence.as_regular().clone(), unlocks)?;

    validate_transaction_payload_length(&tx_payload)?;

    let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, current_time)?;

    if conflict != ConflictReason::None {
        panic!("{conflict:?}, with {tx_payload:#?}");
    }

    Ok(())
}
