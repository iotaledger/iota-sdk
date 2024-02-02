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
        address::{Address, NftAddress},
        input::{Input, UtxoInput},
        output::NftId,
        payload::{signed_transaction::Transaction, SignedTransactionPayload},
        protocol::protocol_parameters,
        slot::SlotIndex,
        unlock::{SignatureUnlock, Unlock},
    },
};
use pretty_assertions::assert_eq;

use crate::client::{
    build_inputs, build_outputs,
    Build::{Basic, Nft},
    NFT_ID_1,
};

#[tokio::test]
async fn nft_reference_unlocks() -> Result<()> {
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

    let protocol_parameters = protocol_parameters();
    let nft_id = NftId::from_str(NFT_ID_1)?;
    let nft_address = Address::Nft(NftAddress::new(nft_id));
    let slot_index = SlotIndex::from(10);

    let inputs = build_inputs(
        [
            Nft(
                1_000_000,
                nft_id,
                address_0.clone(),
                None,
                None,
                None,
                None,
                Some(Bip44::new(SHIMMER_COIN_TYPE)),
            ),
            Basic(1_000_000, nft_address.clone(), None, None, None, None, None, None),
            Basic(1_000_000, nft_address.clone(), None, None, None, None, None, None),
        ],
        Some(slot_index),
    );

    let outputs = build_outputs([
        Nft(1_000_000, nft_id, address_0, None, None, None, None, None),
        Basic(2_000_000, nft_address, None, None, None, None, None, None),
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
        .finish_with_params(&protocol_parameters)?;

    let prepared_transaction_data = PreparedTransactionData {
        transaction,
        inputs_data: inputs,
        remainders: Vec::new(),
        mana_rewards: Default::default(),
    };

    let unlocks = secret_manager
        .transaction_unlocks(&prepared_transaction_data, &protocol_parameters)
        .await?;

    assert_eq!(unlocks.len(), 3);
    assert_eq!((*unlocks).first().unwrap().kind(), SignatureUnlock::KIND);
    match (*unlocks).get(1).unwrap() {
        Unlock::Nft(a) => {
            assert_eq!(a.index(), 0);
        }
        _ => panic!("Invalid unlock"),
    }
    match (*unlocks).get(2).unwrap() {
        Unlock::Nft(a) => {
            assert_eq!(a.index(), 0);
        }
        _ => panic!("Invalid unlock"),
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
