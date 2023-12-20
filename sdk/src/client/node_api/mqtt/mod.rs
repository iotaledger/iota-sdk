// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node MQTT API

mod error;
pub mod types;

use std::{sync::Arc, time::Instant};

use crypto::utils;
use log::warn;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, NetworkOptions, QoS, SubscribeFilter, Transport};
use tokio::sync::watch::Receiver as WatchReceiver;

pub use self::{error::Error, types::*};
use crate::{
    client::{Client, ClientInner},
    types::{
        block::{Block, BlockDto},
        TryFromDto,
    },
};

impl Client {
    /// Returns a handle to the MQTT topics manager.
    pub fn subscriber(&self) -> MqttManager<'_> {
        MqttManager::new(self)
    }

    /// Subscribe to MQTT events with a callback.
    pub async fn subscribe<C: Fn(&TopicEvent) + Send + Sync + 'static>(
        &self,
        topics: impl IntoIterator<Item = Topic> + Send,
        callback: C,
    ) -> Result<(), Error> {
        MqttManager::new(self).with_topics(topics).subscribe(callback).await
    }

    /// Unsubscribe from MQTT events.
    pub async fn unsubscribe(&self, topics: impl IntoIterator<Item = Topic> + Send) -> Result<(), Error> {
        MqttManager::new(self).with_topics(topics).unsubscribe().await
    }
}

impl ClientInner {
    /// Returns the mqtt event receiver.
    pub async fn mqtt_event_receiver(&self) -> WatchReceiver<MqttEvent> {
        self.mqtt.receiver.read().await.clone()
    }
}

async fn set_mqtt_client(client: &Client) -> Result<(), Error> {
    // if the client was disconnected, we clear it so we can start over
    if *client.mqtt_event_receiver().await.borrow() == MqttEvent::Disconnected {
        *client.mqtt.client.write().await = None;
    }
    let exists = client.mqtt.client.read().await.is_some();

    if !exists {
        let node_manager = client.node_manager.read().await;
        let nodes = if !node_manager.ignore_node_health {
            #[cfg(not(target_family = "wasm"))]
            {
                node_manager
                    .healthy_nodes
                    .read()
                    .map_or(node_manager.nodes.clone(), |healthy_nodes| {
                        healthy_nodes.iter().map(|(node, _)| node.clone()).collect()
                    })
            }
            #[cfg(target_family = "wasm")]
            {
                client.node_manager.nodes.clone()
            }
        } else {
            node_manager.nodes.clone()
        };
        for node in &nodes {
            let host = node.url.host_str().expect("can't get host from URL");
            let mut entropy = [0u8; 8];
            utils::rand::fill(&mut entropy)?;
            let id = format!("iotasdk{}", prefix_hex::encode(entropy));
            let broker_options = client.mqtt.broker_options.read().await;
            let port = broker_options.port;
            let secure = node.url.scheme() == "https";
            let mqtt_options = if broker_options.use_ws {
                let path = if node.url.path().ends_with('/') {
                    &node.url.path()[0..node.url.path().len() - 1]
                } else {
                    node.url.path()
                };
                let uri = format!(
                    "{}://{host}:{}{path}/api/mqtt/v2",
                    if secure { "wss" } else { "ws" },
                    node.url.port_or_known_default().unwrap_or(port),
                );
                let mut mqtt_options = MqttOptions::new(id, uri, port);
                if secure {
                    mqtt_options.set_transport(Transport::wss_with_default_config());
                } else {
                    mqtt_options.set_transport(Transport::ws());
                }
                mqtt_options
            } else {
                let uri = host.to_string();
                let mut mqtt_options = MqttOptions::new(id, uri, port);
                if secure {
                    mqtt_options.set_transport(Transport::tls_with_default_config());
                }
                mqtt_options
            };
            let (_, mut connection) = AsyncClient::new(mqtt_options.clone(), 10);
            let mut network_options = NetworkOptions::new();
            network_options.set_connection_timeout(broker_options.timeout.as_secs());
            connection.set_network_options(network_options);
            // poll the event loop until we find a ConnAck event,
            // which means that the mqtt client is ready to be used on this host
            // if the event loop returns an error, we check the next node
            let mut got_ack = false;
            while let Ok(event) = connection.poll().await {
                if let Event::Incoming(Incoming::ConnAck(_)) = event {
                    got_ack = true;
                    break;
                }
            }

            // if we found a valid mqtt connection, loop it on a separate thread
            if got_ack {
                let (mqtt_client, connection) = AsyncClient::new(mqtt_options, 10);
                client.mqtt.client.write().await.replace(mqtt_client);
                poll_mqtt(client, connection);
            }
        }
    }
    Ok(())
}

fn poll_mqtt(client: &Client, mut event_loop: EventLoop) {
    let client = client.clone();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to create Tokio runtime");
        runtime.block_on(async move {
            // rumqttc performs automatic reconnection since we keep running the event loop
            // but the subscriptions are lost on reconnection, so we need to resubscribe
            // the `is_subscribed` flag is set to false on event error, so the ConnAck event
            // can perform the re-subscriptions and reset `is_subscribed` to true.
            // we need the flag since the first ConnAck must be ignored.
            let mut is_subscribed = true;
            let mut error_instant = Instant::now();
            let mut connection_failure_count = 0;

            loop {
                let event = event_loop.poll().await;

                match event {
                    Ok(Event::Incoming(Incoming::ConnAck(_))) => {
                        let _ = client.mqtt.sender.read().await.send(MqttEvent::Connected);
                        if !is_subscribed {
                            is_subscribed = true;
                            // resubscribe topics
                            let topics = client
                                .inner
                                .mqtt
                                .topic_handlers
                                .read()
                                .await
                                .keys()
                                .map(|t| SubscribeFilter::new(t.as_str().to_owned(), QoS::AtLeastOnce))
                                .collect::<Vec<SubscribeFilter>>();
                            if !topics.is_empty() {
                                let _ = client
                                    .inner
                                    .mqtt
                                    .client
                                    .write()
                                    .await
                                    .as_mut()
                                    .unwrap()
                                    .subscribe_many(topics)
                                    .await;
                            }
                        }
                    }
                    Ok(Event::Incoming(Incoming::Publish(p))) => {
                        let client = client.clone();

                        crate::client::async_runtime::spawn(async move {
                            let mqtt_topic_handlers = client.mqtt.topic_handlers.read().await;

                            if let Some(handlers) = mqtt_topic_handlers.get(&Topic::new_unchecked(&p.topic)) {
                                let event = {
                                    if p.topic.contains("blocks") || p.topic.contains("included-block") {
                                        let payload = &*p.payload;
                                        let protocol_parameters = &client.network_info.read().await.protocol_parameters;

                                        match serde_json::from_slice::<BlockDto>(payload) {
                                            Ok(block_dto) => {
                                                match Block::try_from_dto_with_params(block_dto, protocol_parameters) {
                                                    Ok(block) => Ok(TopicEvent {
                                                        topic: p.topic.clone(),
                                                        payload: MqttPayload::Block((&block).into()),
                                                    }),
                                                    Err(e) => {
                                                        warn!("Block dto conversion failed: {:?}", e);
                                                        Err(())
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                warn!("Block parsing failed: {:?}", e);
                                                Err(())
                                            }
                                        }
                                    } else {
                                        match serde_json::from_slice(&p.payload) {
                                            Ok(value) => Ok(TopicEvent {
                                                topic: p.topic.clone(),
                                                payload: MqttPayload::Json(value),
                                            }),
                                            Err(e) => {
                                                warn!("Cannot parse JSON: {:?}", e);
                                                Err(())
                                            }
                                        }
                                    }
                                };
                                if let Ok(event) = event {
                                    for handler in handlers {
                                        handler(&event);
                                    }
                                };
                            }
                        });
                    }
                    Err(_) => {
                        if error_instant.elapsed().as_secs() < 5 {
                            connection_failure_count += 1;
                        } else {
                            connection_failure_count = 1;
                        }
                        if connection_failure_count == client.mqtt.broker_options.read().await.max_reconnection_attempts
                        {
                            let _ = client.mqtt.sender.read().await.send(MqttEvent::Disconnected);
                            break;
                        }
                        error_instant = Instant::now();
                        is_subscribed = false;
                    }
                    _ => {}
                }
            }
        });
    });
}

/// MQTT subscriber.
pub struct MqttManager<'a> {
    client: &'a Client,
}

impl<'a> MqttManager<'a> {
    /// Initializes a new instance of the mqtt subscriber.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Add a new topic to the list.
    pub fn with_topic(self, topic: Topic) -> MqttTopicManager<'a> {
        MqttTopicManager::new(self.client).with_topic(topic)
    }

    /// Add a collection of topics to the list.
    pub fn with_topics(self, topics: impl IntoIterator<Item = Topic>) -> MqttTopicManager<'a> {
        MqttTopicManager::new(self.client).with_topics(topics)
    }

    /// Unsubscribes from all subscriptions.
    pub async fn unsubscribe(self) -> Result<(), Error> {
        MqttTopicManager::new(self.client).unsubscribe().await
    }

    /// Disconnects the broker.
    /// This will clear the stored topic handlers and close the MQTT connection.
    pub async fn disconnect(self) -> Result<(), Error> {
        if let Some(client) = &*self.client.mqtt.client.write().await {
            client.disconnect().await?;
            self.client.mqtt.topic_handlers.write().await.clear();
        }

        *self.client.mqtt.client.write().await = None;

        Ok(())
    }
}

/// The MQTT topic manager.
/// Subscribes and unsubscribes from topics.
pub struct MqttTopicManager<'a> {
    client: &'a Client,
    topics: Vec<Topic>,
}

impl<'a> MqttTopicManager<'a> {
    /// Initializes a new instance of the mqtt topic manager.
    fn new(client: &'a Client) -> Self {
        Self {
            client,
            topics: Vec::new(),
        }
    }

    /// Add a new topic to the list.
    pub fn with_topic(mut self, topic: Topic) -> Self {
        self.topics.push(topic);
        self
    }

    /// Add a collection of topics to the list.
    pub fn with_topics(mut self, topics: impl IntoIterator<Item = Topic>) -> Self {
        self.topics.extend(topics);
        self
    }

    /// Subscribe to the given topics with the callback.
    pub async fn subscribe<C: Fn(&crate::client::node_api::mqtt::TopicEvent) + Send + Sync + 'static>(
        self,
        callback: C,
    ) -> Result<(), Error> {
        let cb =
            Arc::new(Box::new(callback)
                as Box<
                    dyn Fn(&crate::client::node_api::mqtt::TopicEvent) + Send + Sync + 'static,
                >);
        set_mqtt_client(self.client).await?;
        self.client
            .inner
            .mqtt
            .client
            .write()
            .await
            .as_ref()
            .ok_or(Error::ConnectionNotFound)?
            .subscribe_many(
                self.topics
                    .iter()
                    .map(|t| SubscribeFilter::new(t.as_str().to_owned(), QoS::AtLeastOnce)),
            )
            .await?;
        {
            let mut mqtt_topic_handlers = self.client.mqtt.topic_handlers.write().await;
            for topic in self.topics {
                mqtt_topic_handlers.entry(topic).or_default().push(cb.clone());
            }
        }
        Ok(())
    }

    /// Unsubscribe from the given topics.
    /// If no topics were provided, the function will unsubscribe from every subscribed topic.
    pub async fn unsubscribe(self) -> Result<(), Error> {
        let topics = {
            let mqtt_topic_handlers = self.client.mqtt.topic_handlers.read().await;
            if self.topics.is_empty() {
                mqtt_topic_handlers.keys().cloned().collect()
            } else {
                self.topics
            }
        };

        if let Some(client) = &*self.client.mqtt.client.write().await {
            for topic in &topics {
                client.unsubscribe(topic.as_str()).await?;
            }
        }

        let empty_topic_handlers = {
            let mut mqtt_topic_handlers = self.client.mqtt.topic_handlers.write().await;
            for topic in topics {
                mqtt_topic_handlers.remove(&topic);
            }
            mqtt_topic_handlers.is_empty()
        };

        if self.client.mqtt.broker_options.read().await.automatic_disconnect && empty_topic_handlers {
            MqttManager::new(self.client).disconnect().await?;
        }

        Ok(())
    }
}
