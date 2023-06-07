// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: Example description
//!
//! cargo run --example 07_mqtt --features=mqtt --release

use iota_sdk::client::{
    mqtt::{MqttEvent, MqttPayload, Topic},
    Client, Result,
};

// Connecting to a MQTT broker using raw ip doesn't work with TCP. This is a limitation of rustls.
#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let client = Client::builder()
        .with_node("https://api.testnet.shimmer.network")?
        // .with_mqtt_broker_options(BrokerOptions::new().use_ws(false))
        .finish()
        .await?;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    client
        .subscribe(
            [
                Topic::new("milestone-info/latest")?,
                Topic::new("blocks")?,
                Topic::new("outputs/unlock/address/atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r")?,
            ],
            move |event| {
                println!("Topic: {}", event.topic);
                match &event.payload {
                    MqttPayload::Json(val) => println!("{}", serde_json::to_string(&val).unwrap()),
                    MqttPayload::Block(block) => println!("{block:?}"),
                    MqttPayload::MilestonePayload(ms) => println!("{ms:?}"),
                    MqttPayload::Receipt(receipt) => println!("{receipt:?}"),
                }
                tx.send(()).unwrap();
            },
        )
        .await?;

    for i in 0..10 {
        tokio::select! {
            _ = rx.recv() => {
                if i == 7 {
                    client.unsubscribe([Topic::new("blocks")?]).await?;
                }
            }
            _ = async {
                client.mqtt_event_receiver().await.wait_for(|msg| *msg == MqttEvent::Disconnected).await.unwrap();
            } => {
                panic!("mqtt disconnected");
            }
        }
    }

    client.subscriber().disconnect().await?;
    // alternatively
    // client.subscriber().unsubscribe().await?;
    Ok(())
}
