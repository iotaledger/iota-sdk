// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! MQTT types

use std::{collections::HashMap, sync::Arc, time::Duration};

use regex::RegexSet;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize};
use serde_json::Value;

use super::Error;
use crate::types::block::BlockDto;

type TopicHandler = Box<dyn Fn(&TopicEvent) + Send + Sync>;

pub(crate) type TopicHandlerMap = HashMap<Topic, Vec<Arc<TopicHandler>>>;

/// An event from a MQTT topic.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TopicEvent {
    /// the MQTT topic.
    pub topic: String,
    /// The MQTT event payload.
    pub payload: MqttPayload,
}

/// The payload of an `TopicEvent`.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MqttPayload {
    /// In case it contains JSON.
    Json(Value),
    /// In case it contains a `Block` object.
    Block(BlockDto),
}

/// Mqtt events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MqttEvent {
    /// Client was connected.
    Connected,
    /// Client was disconnected.
    Disconnected,
}

/// The MQTT broker options.
#[derive(Copy, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[must_use]
pub struct BrokerOptions {
    #[serde(default = "default_broker_automatic_disconnect")]
    pub(crate) automatic_disconnect: bool,
    #[serde(default = "default_broker_timeout")]
    pub(crate) timeout: Duration,
    #[serde(default = "default_broker_use_ws")]
    pub(crate) use_ws: bool,
    #[serde(default = "default_broker_port")]
    pub(crate) port: u16,
    #[serde(default = "default_max_reconnection_attempts")]
    pub(crate) max_reconnection_attempts: usize,
}

fn default_broker_automatic_disconnect() -> bool {
    true
}

fn default_broker_timeout() -> Duration {
    Duration::from_secs(30)
}

fn default_broker_use_ws() -> bool {
    true
}

fn default_broker_port() -> u16 {
    1883
}

fn default_max_reconnection_attempts() -> usize {
    0
}

impl Default for BrokerOptions {
    fn default() -> Self {
        Self {
            automatic_disconnect: default_broker_automatic_disconnect(),
            timeout: default_broker_timeout(),
            use_ws: default_broker_use_ws(),
            port: default_broker_port(),
            max_reconnection_attempts: default_max_reconnection_attempts(),
        }
    }
}

impl BrokerOptions {
    /// Creates the default broker options.
    pub fn new() -> Self {
        Default::default()
    }

    /// Whether the MQTT broker should be automatically disconnected when all topics are unsubscribed or not.
    pub fn automatic_disconnect(mut self, automatic_disconnect: bool) -> Self {
        self.automatic_disconnect = automatic_disconnect;
        self
    }

    /// Sets the timeout used for the MQTT operations.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the use_ws used for the MQTT operations.
    pub fn use_ws(mut self, use_ws: bool) -> Self {
        self.use_ws = use_ws;
        self
    }

    /// Sets the port used for the MQTT operations.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets the maximum number of reconnection attempts. 0 is unlimited.
    pub fn max_reconnection_attempts(mut self, max_reconnection_attempts: usize) -> Self {
        self.max_reconnection_attempts = max_reconnection_attempts;
        self
    }
}

/// A MQTT topic.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub struct Topic(String);

impl<'de> Deserialize<'de> for Topic {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::new(s).map_err(|err| D::Error::custom(format!("{err}")))
    }
}

impl Topic {
    /// Creates a new topic and checks if it's valid.
    pub fn new(topic: impl Into<String>) -> Result<Self, Error> {
        let topic = Self::new_unchecked(topic);

        if topic.is_valid() {
            Ok(topic)
        } else {
            Err(Error::InvalidTopic(topic.0))
        }
    }

    /// Creates a new topic without checking if the given string represents a valid topic.
    pub(crate) fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub(crate) fn is_valid(&self) -> bool {
        let valid_topics = lazy_static!(
            RegexSet::new([
                // Commitment topics.
                r"^commitment-info/latest$",
                r"^commitment-info/finalized$",
                r"^commitments$",
                // Block topics.
                r"^blocks$",
                r"^blocks/transaction$",
                r"^blocks/transaction/tagged-data$",
                r"^blocks/transaction/tagged-data/0x((?:[a-f0-9]{2}){1,64})$",
                r"^blocks/tagged-data$",
                r"^blocks/tagged-data/0x((?:[a-f0-9]{2}){1,64})$",
                r"^block-metadata/0x([a-f0-9]{64})$",
                r"^block-metadata/accepted$",
                r"^block-metadata/confirmed$",
                // Transaction topics.
                r"^transactions/0x([a-f0-9]{64})/included-block$",
                // Output topics.
                r"^outputs/0x([a-f0-9]{64})(\d{4})$",
                r"^outputs/account/0x([a-f0-9]{64})$",
                r"^outputs/anchor/0x([a-f0-9]{64})$",
                r"^outputs/nft/0x([a-f0-9]{64})$",
                r"^outputs/foundry/0x([a-f0-9]{76})$",
                r"^outputs/unlock/(\+|address|storage-return|expiration|state-controller|governor|immutable-account)/[\x21-\x7E]{1,30}1[A-Za-z0-9]+(?:/spent)?$",
            ]).expect("cannot build regex set") => RegexSet);
        valid_topics.is_match(&self.0)
    }

    /// Returns the topic as a str.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
