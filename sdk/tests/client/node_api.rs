// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.

use iota_sdk::{
    client::{
        api::GetAddressesOptions, node_api::indexer::query_parameters::QueryParameter, request_funds_from_faucet,
        secret::SecretManager, Client,
    },
    types::block::{
        output::OutputId,
        payload::{transaction::TransactionId, Payload, TaggedDataPayload},
        BlockId,
    },
};

use crate::client::common::{setup_client_with_node_health_ignored, FAUCET_URL, NODE_LOCAL};

// THIS SEED SERVES FOR TESTING PURPOSES! DON'T USE THIS SEED IN PRODUCTION!
const DEFAULT_DEVELOPMENT_SEED: &str = "0x256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2";

// Sends a tagged data block to the node to test against it.
async fn setup_tagged_data_block() -> BlockId {
    let client = setup_client_with_node_health_ignored().await;

    client
        .finish_block_builder(
            None,
            Some(Payload::TaggedData(Box::new(
                TaggedDataPayload::new(b"Hello".to_vec(), b"Tangle".to_vec()).unwrap(),
            ))),
        )
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
    let mut round = 0;
    let output_id = loop {
        round += 1;
        if round > 30 {
            panic!("got no funds from faucet")
        }
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
            break output_ids_response.items[0];
        }
    };

    let block_id = *client.get_output_metadata(&output_id).await.unwrap().block_id();

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

#[ignore]
#[tokio::test]
async fn test_get_health() {
    let r = setup_client_with_node_health_ignored()
        .await
        .get_health(NODE_LOCAL)
        .await
        .unwrap();
    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_info() {
    let r = Client::get_node_info(NODE_LOCAL, None).await.unwrap();
    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_tips() {
    let r = setup_client_with_node_health_ignored().await.get_tips().await.unwrap();
    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_post_block_with_tagged_data() {
    let block_id = setup_tagged_data_block().await;
    println!("{block_id}");
}

#[ignore]
#[tokio::test]
async fn test_post_block_with_transaction() {
    let block_id = setup_transaction_block().await;
    println!("Block ID: {block_id:?}");
}

#[ignore]
#[tokio::test]
async fn test_get_block_data() {
    let client = setup_client_with_node_health_ignored().await;

    let block_id = setup_tagged_data_block().await;
    let r = client.get_block(&block_id).await.unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_block_metadata() {
    let block_id = setup_tagged_data_block().await;

    let r = setup_client_with_node_health_ignored()
        .await
        .get_block_metadata(&block_id)
        .await
        .unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_block_raw() {
    let block_id = setup_tagged_data_block().await;

    let r = setup_client_with_node_health_ignored()
        .await
        .get_block_raw(&block_id)
        .await
        .unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_address_outputs() {
    let client = setup_client_with_node_health_ignored().await;
    let secret_manager = setup_secret_manager();

    let address = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::from_client(&client)
                .await
                .unwrap()
                .with_range(0..1),
        )
        .await
        .unwrap()
        .into_iter()
        .next()
        .unwrap();

    let output_ids_response = client
        .basic_output_ids([QueryParameter::Address(address)])
        .await
        .unwrap();

    let r = client.get_outputs(&output_ids_response.items).await.unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_output() {
    let (_block_id, transaction_id) = setup_transaction_block().await;

    let r = setup_client_with_node_health_ignored()
        .await
        .get_output(&OutputId::new(transaction_id, 0).unwrap())
        .await
        .unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_peers() {
    let r = setup_client_with_node_health_ignored().await.get_peers().await.unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_included_block() {
    let (_block_id, transaction_id) = setup_transaction_block().await;

    let r = setup_client_with_node_health_ignored()
        .await
        .get_included_block(&transaction_id)
        .await
        .unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
#[cfg(feature = "mqtt")]
async fn test_mqtt() {
    use iota_sdk::client::mqtt::{MqttEvent, MqttPayload, Topic};
    use tokio::sync::mpsc::error::TrySendError;

    const BUFFER_SIZE: usize = 10;

    let client = setup_client_with_node_health_ignored().await;

    let (tx, mut rx) = tokio::sync::mpsc::channel(BUFFER_SIZE);

    client
        .subscribe([Topic::new("blocks").unwrap()], move |evt| {
            match &evt.payload {
                MqttPayload::Block(_) => {
                    assert_eq!(evt.topic, "blocks");
                }
                MqttPayload::Json(_) => {
                    panic!("unexpected mqtt payload type: {:?}", evt)
                }
            }
            match tx.try_send(()) {
                Ok(_) | Err(TrySendError::Full(_)) => (),
                e => e.unwrap(),
            }
        })
        .await
        .unwrap();

    // Wait for messages to come through
    for i in 0..BUFFER_SIZE {
        tokio::select! {
            _ = rx.recv() => {
                if i == 7 {
                    client.unsubscribe([Topic::new("blocks").unwrap()]).await.unwrap();
                }
            }
            _ = async {
                client.mqtt_event_receiver().await.wait_for(|msg| *msg == MqttEvent::Disconnected).await.unwrap();
            } => {
                panic!("mqtt disconnected");
            }
        }
    }
    client.subscriber().disconnect().await.unwrap();
}
