// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod core;
mod indexer;
#[cfg(feature = "mqtt")]
mod mqtt;

use iota_sdk::{
    client::{
        api::GetAddressesOptions, bech32_to_hex, node_api::indexer::query_parameters::QueryParameter,
        request_funds_from_faucet, secret::SecretManager, Result,
    },
    types::block::{
        address::ToBech32Ext,
        output::{Output, OutputId},
        payload::{
            transaction::{TransactionEssence, TransactionId},
            Payload,
        },
        BlockId,
    },
};

use crate::client::common::{setup_client_with_node_health_ignored, FAUCET_URL};

// THIS SEED SERVES FOR TESTING PURPOSES! DON'T USE THIS SEED IN PRODUCTION!
const DEFAULT_DEVELOPMENT_SEED: &str = "0x256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2";

// Sends a tagged data block to the node to test against it.
async fn setup_tagged_data_block() -> BlockId {
    let client = setup_client_with_node_health_ignored().await;

    client
        .build_block()
        .with_tag(b"Hello".to_vec())
        .with_data(b"Tangle".to_vec())
        .finish()
        .await
        .unwrap()
        .id()
}

pub fn setup_secret_manager() -> SecretManager {
    SecretManager::try_from_hex_seed(DEFAULT_DEVELOPMENT_SEED.to_owned()).unwrap()
}

// Sends a transaction block to the node to test against it.
async fn setup_transaction_block() -> (BlockId, TransactionId) {
    let client = setup_client_with_node_health_ignored().await;
    let secret_manager = setup_secret_manager();

    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::from_client(&client)
                .await
                .unwrap()
                .with_range(0..2),
        )
        .await
        .unwrap();
    println!(
        "{}",
        request_funds_from_faucet(FAUCET_URL, &addresses[0]).await.unwrap()
    );

    // Continue only after funds are received
    for _ in 0..30 {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        let output_ids_response = client
            .basic_output_ids([
                QueryParameter::Address(addresses[0]),
                QueryParameter::HasExpiration(false),
                QueryParameter::HasTimelock(false),
                QueryParameter::HasStorageDepositReturn(false),
            ])
            .await
            .unwrap();

        if !output_ids_response.items.is_empty() {
            break;
        }
    }

    let block_id = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_output_hex(
            // Send funds back to the sender.
            &bech32_to_hex(addresses[1].to_bech32(client.get_bech32_hrp().await.unwrap())).unwrap(),
            // The amount to spend, cannot be zero.
            1_000_000,
        )
        .await
        .unwrap()
        .finish()
        .await
        .unwrap()
        .id();

    let block = setup_client_with_node_health_ignored()
        .await
        .get_block(&block_id)
        .await
        .unwrap();

    let transaction_id = match block.payload() {
        Some(Payload::Transaction(t)) => t.id(),
        _ => unreachable!(),
    };

    let _ = client.retry_until_included(&block.id(), None, None).await.unwrap();

    (block_id, transaction_id)
}

// helper function to get the output id for the first alias output
fn get_alias_output_id(payload: &Payload) -> Result<OutputId> {
    match payload {
        Payload::Transaction(tx_payload) => {
            let TransactionEssence::Regular(regular) = tx_payload.essence();
            for (index, output) in regular.outputs().iter().enumerate() {
                if let Output::Alias(_alias_output) = output {
                    return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
                }
            }
            panic!("No alias output in transaction essence")
        }
        _ => panic!("No tx payload"),
    }
}

// helper function to get the output id for the first foundry output
fn get_foundry_output_id(payload: &Payload) -> Result<OutputId> {
    match payload {
        Payload::Transaction(tx_payload) => {
            let TransactionEssence::Regular(regular) = tx_payload.essence();
            for (index, output) in regular.outputs().iter().enumerate() {
                if let Output::Foundry(_foundry_output) = output {
                    return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
                }
            }
            panic!("No foundry output in transaction essence")
        }
        _ => panic!("No tx payload"),
    }
}

// helper function to get the output id for the first NFT output
fn get_nft_output_id(payload: &Payload) -> Result<OutputId> {
    match payload {
        Payload::Transaction(tx_payload) => {
            let TransactionEssence::Regular(regular) = tx_payload.essence();
            for (index, output) in regular.outputs().iter().enumerate() {
                if let Output::Nft(_nft_output) = output {
                    return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
                }
            }
            panic!("No nft output in transaction essence")
        }
        _ => panic!("No tx payload"),
    }
}
