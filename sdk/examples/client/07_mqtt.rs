// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to listen to MQTT events of a node.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example 07_mqtt [NUM_EVENTS] [ADDRESS]
//! ```

use iota_sdk::{
    client::{
        mqtt::{BrokerOptions, MqttEvent, MqttPayload, Topic},
        Client, Result,
    },
    types::block::address::Bech32Address,
};

// Connecting to a MQTT broker using raw ip doesn't work with TCP. This is a limitation of rustls.
#[tokio::main]
async fn main() -> Result<()> {
    let num_events: usize = std::env::args().nth(1).map(|s| s.parse().unwrap()).unwrap_or(10);

    let address: Bech32Address = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".to_string())
        .parse()?;

    // Create a node client.
    let client = Client::builder()
        .with_node("http://localhost:8050")?
        .with_mqtt_broker_options(BrokerOptions::new().use_ws(true))
        .finish()
        .await?;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    client
        .subscribe(
            [
                Topic::new("blocks")?,
                Topic::new(format!("outputs/unlock/address/{address}"))?,
            ],
            move |event| {
                println!("> Topic: {}", event.topic);
                match &event.payload {
                    MqttPayload::Json(val) => println!("{}", serde_json::to_string(&val).unwrap()),
                    MqttPayload::Block(block) => println!("{block:?}"),
                    e => println!("unknown event received: {e:?}"),
                }
                tx.send(()).unwrap();
            },
        )
        .await?;

    let mut event_count = 0;
    loop {
        tokio::select! {
            _ = rx.recv() => {
                event_count += 1;
                if event_count == num_events {
                    client.unsubscribe([Topic::new("commitment-info/latest")?]).await?;
                    client.unsubscribe([Topic::new("blocks")?]).await?;
                    client.unsubscribe([Topic::new(format!("outputs/unlock/address/{address}"))?]).await?;
                    break;
                }
            }
            _ = async {
                client.mqtt_event_receiver().await.wait_for(|msg| *msg == MqttEvent::Disconnected).await.unwrap();
                println!("Disconnected by remote");
            } => {
                panic!("mqtt disconnected");
            }
        }
    }

    client.subscriber().disconnect().await?;
    // alternatively
    // client.subscriber().unsubscribe().await?;

    println!("Example completed successfully");

    Ok(())
}
