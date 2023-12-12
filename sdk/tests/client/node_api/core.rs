// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.

use iota_sdk::{
    client::{
        api::GetAddressesOptions, node_api::indexer::query_parameters::BasicOutputQueryParameters, Client,
        NodeInfoWrapper,
    },
    types::{
        api::core::TransactionState,
        block::{
            output::{Output, OutputId},
            Block,
        },
    },
};
use packable::PackableExt;
use pretty_assertions::assert_eq;

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
async fn test_get_issuance() {
    let r = setup_client_with_node_health_ignored()
        .await
        .get_issuance()
        .await
        .unwrap();
    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_post_block_with_tagged_data() {
    let secret_manager = setup_secret_manager();
    let block_id = setup_tagged_data_block(&secret_manager).await;
    println!("{block_id}");
}

#[ignore]
#[tokio::test]
async fn test_post_block_with_transaction() {
    let client = setup_client_with_node_health_ignored().await;
    let block_id = setup_transaction_block(&client).await;
    println!("Block ID: {block_id:?}");
}

#[ignore]
#[tokio::test]
async fn test_get_block_data() {
    let client = setup_client_with_node_health_ignored().await;
    let secret_manager = setup_secret_manager();

    let block_id = setup_tagged_data_block(&secret_manager).await;
    let r = client.get_block(&block_id).await.unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_block_metadata() {
    let secret_manager = setup_secret_manager();
    let block_id = setup_tagged_data_block(&secret_manager).await;

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
    let secret_manager = setup_secret_manager();
    let block_id = setup_tagged_data_block(&secret_manager).await;

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
        .basic_output_ids(BasicOutputQueryParameters::new().address(address))
        .await
        .unwrap();

    let r = client.get_outputs(&output_ids_response.items).await.unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_output() {
    let client = setup_client_with_node_health_ignored().await;
    let (_block_id, transaction_id) = setup_transaction_block(&client).await;

    let r = client.get_output(&OutputId::new(transaction_id, 0)).await.unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_output_raw() {
    let client = setup_client_with_node_health_ignored().await;
    let (_block_id, transaction_id) = setup_transaction_block(&client).await;
    let output_id = OutputId::new(transaction_id, 0);

    let output = client.get_output(&output_id).await.unwrap();
    let output_raw = Output::unpack_verified(
        client.get_output_raw(&output_id).await.unwrap(),
        &client.get_protocol_parameters().await.unwrap(),
    )
    .unwrap();

    assert_eq!(output.output, output_raw);
}

#[ignore]
#[tokio::test]
async fn test_get_included_block() {
    let client = setup_client_with_node_health_ignored().await;
    let (_block_id, transaction_id) = setup_transaction_block(&client).await;

    let r = client.get_included_block(&transaction_id).await.unwrap();

    println!("{r:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_included_block_raw() {
    let client = setup_client_with_node_health_ignored().await;
    let (_block_id, transaction_id) = setup_transaction_block(&client).await;

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

#[ignore]
#[tokio::test]
async fn test_get_routes() {
    let client = setup_client_with_node_health_ignored().await;

    let routes_response = client.get_routes().await.unwrap();
    // At at least one route, which is not created by plugin, is available
    assert!(routes_response.routes.contains(&"core/v2".to_string()));

    println!("{routes_response:#?}");
}

#[ignore]
#[tokio::test]
async fn test_get_included_block_metadata() {
    let client = setup_client_with_node_health_ignored().await;
    let (block_id, transaction_id) = setup_transaction_block(&client).await;
    let metadata_response = client.get_included_block_metadata(&transaction_id).await.unwrap();

    assert_eq!(metadata_response.block_id, block_id);
    assert_eq!(
        metadata_response
            .transaction_metadata
            .as_ref()
            .map(|m| m.transaction_state),
        Some(TransactionState::Finalized)
    );

    println!("{metadata_response:#?}");
}
