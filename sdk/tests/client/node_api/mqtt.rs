// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::common::setup_client_with_node_health_ignored;

#[ignore]
#[tokio::test]
async fn test_mqtt() {
    use iota_sdk::client::mqtt::{MqttEvent, MqttPayload, Topic};
    use tokio::sync::mpsc::error::TrySendError;

    const BUFFER_SIZE: usize = 10;

    let client = setup_client_with_node_health_ignored().await;

    let (tx, mut rx) = tokio::sync::mpsc::channel(BUFFER_SIZE);

    client
        .subscribe(
            [
                Topic::new("milestone-info/latest").unwrap(),
                Topic::new("blocks").unwrap(),
            ],
            move |evt| {
                match &evt.payload {
                    MqttPayload::Block(_) => {
                        assert_eq!(evt.topic, "blocks");
                    }
                    MqttPayload::Json(_) => {
                        assert_eq!(evt.topic, "milestone-info/latest");
                    }
                    _ => panic!("unexpected mqtt payload type: {:?}", evt),
                }
                match tx.try_send(()) {
                    Ok(_) | Err(TrySendError::Full(_)) => (),
                    e => e.unwrap(),
                }
            },
        )
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
