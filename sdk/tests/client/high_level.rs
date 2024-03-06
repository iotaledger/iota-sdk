// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{address::ToBech32Ext, payload::Payload};
use pretty_assertions::assert_eq;

use crate::client::{common::setup_client_with_node_health_ignored, node_api::setup_transaction_block};

#[ignore]
#[tokio::test]
async fn test_get_transaction_inputs() {
    let client = setup_client_with_node_health_ignored().await;
    let (_block_id, transaction_id) = setup_transaction_block(&client).await;
    let inputs = client.get_transaction_inputs(&transaction_id).await.unwrap();

    assert_eq!(inputs.len(), 1);
}

#[ignore]
#[tokio::test]
async fn test_find_blocks() {
    let client = setup_client_with_node_health_ignored().await;
    let (block_id, _transaction_id) = setup_transaction_block(&client).await;
    let blocks = client.find_blocks(&[block_id]).await.unwrap();

    assert_eq!(blocks.len(), 1);
    assert_eq!(client.block_id(&blocks[0]).await.unwrap(), block_id);
}

#[ignore]
#[tokio::test]
async fn test_find_inputs() {
    let client = setup_client_with_node_health_ignored().await;
    let (block_id, _transaction_id) = setup_transaction_block(&client).await;
    let block = client.get_block(&block_id).await.unwrap();
    let transaction = block.as_basic().payload().unwrap();

    if let Payload::SignedTransaction(transaction) = transaction {
        let basic_output = transaction.transaction().outputs().iter().next().unwrap().as_basic();
        let address = basic_output
            .unlock_conditions()
            .address()
            .unwrap()
            .address()
            .clone()
            .to_bech32(client.get_bech32_hrp().await.unwrap());

        let input = client.find_inputs(vec![address], 1_000_000).await.unwrap();

        // The ['setup_transaction_block'] generates one output with 1000000 tokens,
        // but there could be other transactions that also send tokens to the same address,
        // so we expect at least one input.
        assert!(!input.is_empty());
    } else {
        unreachable!();
    }
}
