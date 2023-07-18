// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::api::plugins::participation::types::ParticipationEventType;

use crate::client::common::setup_client_with_node_health_ignored;

#[ignore]
#[tokio::test]
async fn test_get_all_events() {
    let client = setup_client_with_node_health_ignored().await;
    let response = client.events(None).await.unwrap();

    for event_id in response.event_ids.iter() {
        let event = client.event(event_id).await.unwrap();
        println!("{:?}", event);
        let status = client.event_status(event_id, None).await.unwrap();
        println!("{:?}", status);
    }
}

#[ignore]
#[tokio::test]
async fn test_get_voting_events() {
    let client = setup_client_with_node_health_ignored().await;
    let response = client.events(Some(ParticipationEventType::Voting)).await.unwrap();
    for event_id in response.event_ids.iter() {
        let event = client.event(event_id).await.unwrap();
        println!("{:?}", event);
        let status = client.event_status(event_id, None).await.unwrap();
        println!("{:?}", status);
    }
}

#[ignore]
#[tokio::test]
async fn test_get_staking_events() {
    let client = setup_client_with_node_health_ignored().await;
    let response = client.events(Some(ParticipationEventType::Staking)).await.unwrap();
    for event_id in response.event_ids.iter() {
        let event = client.event(event_id).await.unwrap();
        println!("{:?}", event);
        let status = client.event_status(event_id, None).await.unwrap();
        println!("{:?}", status);

        client.address_staking_status(bech32_address)
    }
}
