// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.

use iota_sdk::{
    client::{api::GetAddressesOptions, node_api::indexer::query_parameters::QueryParameter, Client, NodeInfoWrapper},
    types::block::{
        output::{Output, OutputId},
        payload::Payload,
        Block,
    },
};
use packable::PackableExt;

use super::{setup_secret_manager, setup_tagged_data_block, setup_transaction_block};
use crate::client::common::{setup_client_with_node_health_ignored, NODE_LOCAL};

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
async fn test_get_output_raw() {
    let (_block_id, transaction_id) = setup_transaction_block().await;
    let output_id = OutputId::new(transaction_id, 0).unwrap();

    let client = setup_client_with_node_health_ignored().await;

    let output = client.get_output(&output_id).await.unwrap().into_output();
    let output_raw = Output::unpack_verified(
        client.get_output_raw(&output_id).await.unwrap(),
        &client.get_protocol_parameters().await.unwrap(),
    )
    .unwrap();

    assert_eq!(output, output_raw);
}

#[ignore]
#[tokio::test]
async fn test_get_peers() {
    let r = setup_client_with_node_health_ignored().await.get_peers().await.unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_milestone_by_id() {
    let client = setup_client_with_node_health_ignored().await;

    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_milestone_by_id(&node_info.node_info.status.latest_milestone.milestone_id.unwrap())
        .await
        .unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_milestone_by_id_raw() {
    let client = setup_client_with_node_health_ignored().await;

    let latest_milestone_id = client
        .get_info()
        .await
        .unwrap()
        .node_info
        .status
        .latest_milestone
        .milestone_id
        .unwrap();

    let milestone = client.get_milestone_by_id(&latest_milestone_id).await.unwrap();
    let milestone_raw = Payload::unpack_verified(
        client.get_milestone_by_id_raw(&latest_milestone_id).await.unwrap(),
        &client.get_protocol_parameters().await.unwrap(),
    )
    .unwrap();

    if let Payload::Milestone(milestone_raw) = milestone_raw {
        assert_eq!(milestone, *milestone_raw);
    } else {
        panic!("expected a milestone payload")
    }
}

#[ignore]
#[tokio::test]
async fn test_get_milestone_by_index() {
    let client = setup_client_with_node_health_ignored().await;

    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_milestone_by_index(node_info.node_info.status.latest_milestone.index)
        .await
        .unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_milestone_by_index_raw() {
    let client = setup_client_with_node_health_ignored().await;

    let milestone_index = client.get_info().await.unwrap().node_info.status.latest_milestone.index;

    let milestone = client.get_milestone_by_index(milestone_index).await.unwrap();
    let milestone_raw = Payload::unpack_verified(
        client.get_milestone_by_index_raw(milestone_index).await.unwrap(),
        &client.get_protocol_parameters().await.unwrap(),
    )
    .unwrap();

    if let Payload::Milestone(milestone_raw) = milestone_raw {
        assert_eq!(milestone, *milestone_raw);
    } else {
        panic!("expected a milestone payload")
    }
}

#[ignore]
#[tokio::test]
async fn test_get_utxo_changes_by_id() {
    let client = setup_client_with_node_health_ignored().await;

    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_utxo_changes_by_id(&node_info.node_info.status.latest_milestone.milestone_id.unwrap())
        .await
        .unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_utxo_changes_by_index() {
    let client = setup_client_with_node_health_ignored().await;

    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_utxo_changes_by_index(node_info.node_info.status.latest_milestone.index)
        .await
        .unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_receipts() {
    let r = setup_client_with_node_health_ignored()
        .await
        .get_receipts()
        .await
        .unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn get_receipts_migrated_at() {
    let r = setup_client_with_node_health_ignored()
        .await
        .get_receipts_migrated_at(3)
        .await
        .unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_treasury() {
    let r = setup_client_with_node_health_ignored()
        .await
        .get_treasury()
        .await
        .unwrap();

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
async fn test_get_included_block_raw() {
    let (_block_id, transaction_id) = setup_transaction_block().await;

    let client = setup_client_with_node_health_ignored().await;

    let block = client.get_included_block(&transaction_id).await.unwrap();
    let block_raw = Block::unpack_verified(
        client.get_included_block_raw(&transaction_id).await.unwrap(),
        &client.get_protocol_parameters().await.unwrap(),
    )
    .unwrap();

    assert_eq!(block, block_raw);
}

#[ignore]
#[tokio::test]
async fn test_call_plugin_route() {
    let c = setup_client_with_node_health_ignored().await;

    // we call the "custom" plugin "node info"
    let plugin_res: NodeInfoWrapper = c
        .call_plugin_route("api/core/v2/", "GET", "info", vec![], None)
        .await
        .unwrap();

    let info = c.get_info().await.unwrap();

    // Just check name as info can change between 2 calls
    assert_eq!(plugin_res.node_info.name, info.node_info.name);
}
