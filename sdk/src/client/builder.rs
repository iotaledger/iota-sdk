// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Builder of the Client Instance
use std::{collections::HashMap, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};

use super::{node_manager::builder::NodeManagerBuilder, ClientInner};
#[cfg(feature = "mqtt")]
use crate::client::node_api::mqtt::{BrokerOptions, MqttEvent};
use crate::{
    client::{
        constants::{DEFAULT_API_TIMEOUT, DEFAULT_REMOTE_POW_API_TIMEOUT, DEFAULT_TIPS_INTERVAL},
        error::Result,
        node_manager::{
            builder::validate_url,
            node::{Node, NodeAuth},
        },
        Client,
    },
    types::block::protocol::ProtocolParameters,
};

/// Builder to construct client instance with sensible default values
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[must_use]
pub struct ClientBuilder {
    /// Node manager builder
    #[serde(flatten)]
    pub node_manager_builder: crate::client::node_manager::builder::NodeManagerBuilder,
    /// Options for the MQTT broker
    #[cfg(feature = "mqtt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mqtt")))]
    #[serde(flatten)]
    pub broker_options: BrokerOptions,
    /// Data related to the used network
    #[serde(flatten, default)]
    pub network_info: NetworkInfo,
    /// Timeout for API requests
    #[serde(default = "default_api_timeout")]
    pub api_timeout: Duration,
    /// Timeout when sending a block that requires remote proof of work
    #[serde(default = "default_remote_pow_timeout")]
    pub remote_pow_timeout: Duration,
    /// The amount of threads to be used for proof of work
    #[cfg(not(target_family = "wasm"))]
    #[serde(default)]
    pub pow_worker_count: Option<usize>,
}

fn default_api_timeout() -> Duration {
    DEFAULT_API_TIMEOUT
}

fn default_remote_pow_timeout() -> Duration {
    DEFAULT_REMOTE_POW_API_TIMEOUT
}

impl Default for NetworkInfo {
    fn default() -> Self {
        Self {
            // TODO do we really want a default?
            protocol_parameters: ProtocolParameters::default(),
            local_pow: default_local_pow(),
            fallback_to_local_pow: true,
            tips_interval: DEFAULT_TIPS_INTERVAL,
            latest_milestone_timestamp: None,
        }
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            node_manager_builder: crate::client::node_manager::NodeManager::builder(),
            #[cfg(feature = "mqtt")]
            broker_options: Default::default(),
            network_info: NetworkInfo::default(),
            api_timeout: DEFAULT_API_TIMEOUT,
            remote_pow_timeout: DEFAULT_REMOTE_POW_API_TIMEOUT,
            #[cfg(not(target_family = "wasm"))]
            pow_worker_count: None,
        }
    }
}

impl ClientBuilder {
    /// Creates an IOTA client builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the fields from a client JSON config
    #[allow(unused_assignments)]
    pub fn from_json(mut self, client_config: &str) -> Result<Self> {
        self = serde_json::from_str::<Self>(client_config)?;
        // validate URLs
        if let Some(node_dto) = &self.node_manager_builder.primary_node {
            let node: Node = node_dto.into();
            validate_url(node.url)?;
        }
        if let Some(node_dto) = &self.node_manager_builder.primary_pow_node {
            let node: Node = node_dto.into();
            validate_url(node.url)?;
        }
        for node_dto in &self.node_manager_builder.nodes {
            let node: Node = node_dto.into();
            validate_url(node.url)?;
        }
        if let Some(permanodes) = &self.node_manager_builder.permanodes {
            for node_dto in permanodes {
                let node: Node = node_dto.into();
                validate_url(node.url)?;
            }
        }
        Ok(self)
    }

    /// Adds an IOTA node by its URL.
    pub fn with_node(mut self, url: &str) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_node(url)?;
        Ok(self)
    }

    /// Adds an IOTA node by its URL to be used as primary node, with optional jwt and or basic authentication
    pub fn with_primary_node(mut self, url: &str, auth: Option<NodeAuth>) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_primary_node(url, auth)?;
        Ok(self)
    }

    /// Adds an IOTA node by its URL to be used as primary PoW node (for remote Pow), with optional jwt and or basic
    /// authentication
    pub fn with_primary_pow_node(mut self, url: &str, auth: Option<NodeAuth>) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_primary_pow_node(url, auth)?;
        Ok(self)
    }

    /// Adds a permanode by its URL, with optional jwt and or basic authentication
    pub fn with_permanode(mut self, url: &str, auth: Option<NodeAuth>) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_permanode(url, auth)?;
        Ok(self)
    }

    /// Adds an IOTA node by its URL with optional jwt and or basic authentication
    pub fn with_node_auth(mut self, url: &str, auth: Option<NodeAuth>) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_node_auth(url, auth)?;
        Ok(self)
    }

    /// Adds a list of IOTA nodes by their URLs.
    pub fn with_nodes(mut self, urls: &[&str]) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_nodes(urls)?;
        Ok(self)
    }

    /// Set the node sync interval
    pub fn with_node_sync_interval(mut self, node_sync_interval: Duration) -> Self {
        self.node_manager_builder = self.node_manager_builder.with_node_sync_interval(node_sync_interval);
        self
    }

    /// Ignores the node health status.
    /// Every node will be considered healthy and ready to use.
    pub fn with_ignore_node_health(mut self) -> Self {
        self.node_manager_builder = self.node_manager_builder.with_ignore_node_health();
        self
    }

    /// Set if quorum should be used or not
    pub fn with_quorum(mut self, quorum: bool) -> Self {
        self.node_manager_builder = self.node_manager_builder.with_quorum(quorum);
        self
    }

    /// Set amount of nodes which should be used for quorum
    pub fn with_min_quorum_size(mut self, min_quorum_size: usize) -> Self {
        self.node_manager_builder = self.node_manager_builder.with_min_quorum_size(min_quorum_size);
        self
    }

    /// Set quorum_threshold
    pub fn with_quorum_threshold(mut self, threshold: usize) -> Self {
        let threshold = if threshold > 100 { 100 } else { threshold };
        self.node_manager_builder = self.node_manager_builder.with_quorum_threshold(threshold);
        self
    }

    /// Sets the MQTT broker options.
    #[cfg(feature = "mqtt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mqtt")))]
    pub fn with_mqtt_broker_options(mut self, options: BrokerOptions) -> Self {
        self.broker_options = options;
        self
    }

    /// Sets whether the PoW should be done locally or remotely.
    pub fn with_local_pow(mut self, local: bool) -> Self {
        self.network_info.local_pow = local;
        self
    }

    /// Sets the amount of workers that should be used for PoW, default is num_cpus::get().
    #[cfg(not(target_family = "wasm"))]
    pub fn with_pow_worker_count(mut self, worker_count: impl Into<Option<usize>>) -> Self {
        self.pow_worker_count = worker_count.into();
        self
    }

    /// Sets whether the PoW should be done locally in case a node doesn't support remote PoW.
    pub fn with_fallback_to_local_pow(mut self, fallback_to_local_pow: bool) -> Self {
        self.network_info.fallback_to_local_pow = fallback_to_local_pow;
        self
    }

    /// Sets after how many seconds new tips will be requested during PoW
    pub fn with_tips_interval(mut self, tips_interval: u64) -> Self {
        self.network_info.tips_interval = tips_interval;
        self
    }

    /// Sets the default request timeout.
    pub fn with_api_timeout(mut self, timeout: Duration) -> Self {
        self.api_timeout = timeout;
        self
    }

    /// Sets the request timeout for API usage.
    pub fn with_remote_pow_timeout(mut self, timeout: Duration) -> Self {
        self.remote_pow_timeout = timeout;
        self
    }

    /// Set User-Agent header for requests
    /// Default is "iota-client/{version}"
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.node_manager_builder = self.node_manager_builder.with_user_agent(user_agent);
        self
    }

    /// Build the Client instance.
    #[cfg(not(target_family = "wasm"))]
    pub async fn finish(self) -> Result<Client> {
        use tokio::sync::RwLock;

        let node_sync_interval = self.node_manager_builder.node_sync_interval;
        let ignore_node_health = self.node_manager_builder.ignore_node_health;
        let nodes = self
            .node_manager_builder
            .primary_node
            .iter()
            .chain(self.node_manager_builder.nodes.iter())
            .map(|node| node.clone().into())
            .collect();

        #[cfg(feature = "mqtt")]
        let (mqtt_event_tx, mqtt_event_rx) = tokio::sync::watch::channel(MqttEvent::Connected);

        let client_inner = Arc::new(ClientInner {
            node_manager: RwLock::new(self.node_manager_builder.build(HashMap::new())),
            network_info: RwLock::new(self.network_info),
            api_timeout: RwLock::new(self.api_timeout),
            remote_pow_timeout: RwLock::new(self.remote_pow_timeout),
            pow_worker_count: RwLock::new(self.pow_worker_count),
            #[cfg(feature = "mqtt")]
            mqtt: super::MqttInner {
                client: Default::default(),
                topic_handlers: Default::default(),
                broker_options: RwLock::new(self.broker_options),
                sender: RwLock::new(mqtt_event_tx),
                receiver: RwLock::new(mqtt_event_rx),
            },
        });

        client_inner.sync_nodes(&nodes, ignore_node_health).await?;
        let client_clone = client_inner.clone();

        let sync_handle = tokio::spawn(async move {
            client_clone
                .start_sync_process(nodes, node_sync_interval, ignore_node_health)
                .await
        });

        let client = Client {
            inner: client_inner,
            _sync_handle: Arc::new(RwLock::new(super::SyncHandle(Some(sync_handle)))),
        };

        Ok(client)
    }

    /// Build the Client instance.
    #[cfg(target_family = "wasm")]
    pub async fn finish(self) -> Result<Client> {
        use tokio::sync::RwLock;

        #[cfg(feature = "mqtt")]
        let (mqtt_event_tx, mqtt_event_rx) = tokio::sync::watch::channel(MqttEvent::Connected);

        let client = Client {
            inner: Arc::new(ClientInner {
                node_manager: RwLock::new(self.node_manager_builder.build(HashMap::new())),
                network_info: RwLock::new(self.network_info),
                api_timeout: RwLock::new(self.api_timeout),
                remote_pow_timeout: RwLock::new(self.remote_pow_timeout),
                #[cfg(feature = "mqtt")]
                mqtt: super::MqttInner {
                    client: Default::default(),
                    topic_handlers: Default::default(),
                    broker_options: RwLock::new(self.broker_options),
                    sender: RwLock::new(mqtt_event_tx),
                    receiver: RwLock::new(mqtt_event_rx),
                },
            }),
        };

        Ok(client)
    }

    pub async fn from_client(client: &Client) -> Self {
        Self {
            node_manager_builder: NodeManagerBuilder::from(&*client.node_manager.read().await),
            #[cfg(feature = "mqtt")]
            broker_options: *client.mqtt.broker_options.read().await,
            network_info: client.network_info.read().await.clone(),
            api_timeout: client.get_timeout().await,
            remote_pow_timeout: client.get_remote_pow_timeout().await,
            #[cfg(not(target_family = "wasm"))]
            pow_worker_count: *client.pow_worker_count.read().await,
        }
    }
}

/// Struct containing network and PoW related information
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkInfo {
    /// Protocol parameters.
    #[serde(default)]
    pub protocol_parameters: ProtocolParameters,
    /// Local proof of work.
    #[serde(default = "default_local_pow")]
    pub local_pow: bool,
    /// Fallback to local proof of work if the node doesn't support remote PoW.
    #[serde(default = "default_fallback_to_local_pow")]
    pub fallback_to_local_pow: bool,
    /// Tips request interval during PoW in seconds.
    #[serde(default = "default_tips_interval")]
    pub tips_interval: u64,
    /// The latest cached milestone timestamp.
    #[serde(skip)]
    pub latest_milestone_timestamp: Option<u32>,
}

fn default_local_pow() -> bool {
    #[cfg(not(target_family = "wasm"))]
    {
        true
    }
    #[cfg(target_family = "wasm")]
    {
        false
    }
}

fn default_fallback_to_local_pow() -> bool {
    true
}

fn default_tips_interval() -> u64 {
    DEFAULT_TIPS_INTERVAL
}
