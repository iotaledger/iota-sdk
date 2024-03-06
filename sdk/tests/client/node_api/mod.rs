// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod core;
mod indexer;
#[cfg(feature = "mqtt")]
mod mqtt;

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        api::GetAddressesOptions,
        constants::IOTA_COIN_TYPE,
        node_api::indexer::query_parameters::BasicOutputQueryParameters,
        request_funds_from_faucet,
        secret::{SecretManager, SignBlock},
        Client,
    },
    types::block::{
        output::AccountId,
        payload::{signed_transaction::TransactionId, tagged_data::TaggedDataPayload, Payload},
        BlockId,
    },
};

use crate::client::common::{setup_client_with_node_health_ignored, FAUCET_URL};

// THIS SEED SERVES FOR TESTING PURPOSES! DON'T USE THIS SEED IN PRODUCTION!
const DEFAULT_DEVELOPMENT_SEED: &str = "0x256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2";

// Sends a tagged data block to the node to test against it.
async fn setup_tagged_data_block(secret_manager: &SecretManager) -> BlockId {
    let client = setup_client_with_node_health_ignored().await;

    let protocol_params = client.get_protocol_parameters().await.unwrap();

    client
        .build_basic_block(
            AccountId::null(),
            Some(Payload::TaggedData(Box::new(
                TaggedDataPayload::new(b"Hello".to_vec(), b"Tangle".to_vec()).unwrap(),
            ))),
        )
        .await
        .unwrap()
        .sign_ed25519(secret_manager, Bip44::new(IOTA_COIN_TYPE))
        .await
        .unwrap()
        .id(&protocol_params)
}

pub fn setup_secret_manager() -> SecretManager {
    SecretManager::try_from_hex_seed(DEFAULT_DEVELOPMENT_SEED.to_owned()).unwrap()
}

// Sends a transaction block to the node to test against it.
pub async fn setup_transaction_block(client: &Client) -> (BlockId, TransactionId) {
    let secret_manager = setup_secret_manager();

    let addresses = secret_manager
        .generate_ed25519_addresses_as_bech32(GetAddressesOptions::from_client(client).await.unwrap().with_range(0..2))
        .await
        .unwrap();
    println!(
        "{}",
        request_funds_from_faucet(FAUCET_URL, &addresses[0]).await.unwrap()
    );

    // Continue only after funds are received
    let mut round = 0;
    let output_id = loop {
        round += 1;
        if round > 30 {
            panic!("got no funds from faucet")
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        let output_ids_response = client
            .basic_output_ids(BasicOutputQueryParameters::only_address_unlock_condition(
                addresses[0].clone(),
            ))
            .await
            .unwrap();

        if !output_ids_response.items.is_empty() {
            break output_ids_response.items[0];
        }
    };

    let block_id = *client.get_output_metadata(&output_id).await.unwrap().block_id();

    let block = client.get_block(&block_id).await.unwrap();

    let transaction_id = block
        .as_basic()
        .payload()
        .unwrap()
        .as_signed_transaction()
        .transaction()
        .id();

    // TODO reenable but `wait_for_transaction_acceptance` is on Wallet and not on Client.
    // let retried_blocks = client.retry_until_included(&block.id(), None, None).await.unwrap();
    // (retried_blocks[0].0, transaction_id)

    (block_id, transaction_id)
}

// TODO uncomment

// // helper function to get the output id for the first alias output
// fn get_alias_output_id(payload: &Payload) -> Result<OutputId, ClientError> {
//     match payload {
//         Payload::Transaction(tx_payload) => {
//             for (index, output) in tx_payload.transaction().as_regular().outputs().iter().enumerate() {
//                 if let Output::Alias(_alias_output) = output {
//                     return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
//                 }
//             }
//             panic!("No alias output in transaction")
//         }
//         _ => panic!("No tx payload"),
//     }
// }

// // helper function to get the output id for the first foundry output
// fn get_foundry_output_id(payload: &Payload) -> Result<OutputId, ClientError> {
//     match payload {
//         Payload::Transaction(tx_payload) => {
//             for (index, output) in tx_payload.transaction().as_regular().outputs().iter().enumerate() {
//                 if let Output::Foundry(_foundry_output) = output {
//                     return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
//                 }
//             }
//             panic!("No foundry output in transaction")
//         }
//         _ => panic!("No tx payload"),
//     }
// }

// // helper function to get the output id for the first NFT output
// fn get_nft_output_id(payload: &Payload) -> Result<OutputId, ClientError> {
//     match payload {
//         Payload::Transaction(tx_payload) => {
//             for (index, output) in tx_payload.transaction().as_regular().outputs().iter().enumerate() {
//                 if let Output::Nft(_nft_output) = output {
//                     return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
//                 }
//             }
//             panic!("No nft output in transaction")
//         }
//         _ => panic!("No tx payload"),
//     }
// }
