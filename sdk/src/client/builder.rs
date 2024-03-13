// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Builder of the Client Instance
use std::{collections::HashSet, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};

use super::{node_manager::builder::NodeManagerBuilder, ClientInner};
#[cfg(feature = "mqtt")]
use crate::client::node_api::mqtt::{BrokerOptions, MqttEvent};
use crate::{
    client::{
        constants::DEFAULT_API_TIMEOUT,
        node_manager::{
            builder::validate_url,
            node::{Node, NodeAuth},
        },
        Client, ClientError,
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
    #[serde(default)]
    pub broker_options: BrokerOptions,
    /// Protocol parameters
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol_parameters: Option<ProtocolParameters>,
    /// Timeout for API requests
    #[serde(default = "default_api_timeout")]
    pub api_timeout: Duration,
    /// The maximum parallel API requests
    #[cfg(not(target_family = "wasm"))]
    #[serde(default = "default_max_parallel_api_requests")]
    pub max_parallel_api_requests: usize,
}

fn default_api_timeout() -> Duration {
    DEFAULT_API_TIMEOUT
}

#[cfg(not(target_family = "wasm"))]
fn default_max_parallel_api_requests() -> usize {
    super::constants::MAX_PARALLEL_API_REQUESTS
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            node_manager_builder: crate::client::node_manager::NodeManager::builder(),
            #[cfg(feature = "mqtt")]
            broker_options: Default::default(),
            protocol_parameters: None,
            api_timeout: DEFAULT_API_TIMEOUT,
            #[cfg(not(target_family = "wasm"))]
            max_parallel_api_requests: super::constants::MAX_PARALLEL_API_REQUESTS,
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
    pub fn from_json(mut self, client_config: &str) -> Result<Self, ClientError> {
        self = serde_json::from_str::<Self>(client_config)?;
        // validate URLs
        for node_dto in &self.node_manager_builder.primary_nodes {
            let node: Node = node_dto.into();
            validate_url(node.url)?;
        }
        for node_dto in &self.node_manager_builder.nodes {
            let node: Node = node_dto.into();
            validate_url(node.url)?;
        }
        Ok(self)
    }

    /// Adds an IOTA node by its URL.
    pub fn with_node(mut self, url: &str) -> Result<Self, ClientError> {
        self.node_manager_builder = self.node_manager_builder.with_node(url)?;
        Ok(self)
    }

    // Adds a node as primary node.
    pub fn with_primary_node(mut self, node: Node) -> Result<Self, ClientError> {
        self.node_manager_builder = self.node_manager_builder.with_primary_node(node)?;
        Ok(self)
    }

    /// Adds a list of IOTA nodes by their URLs to the primary nodes list.
    pub fn with_primary_nodes(mut self, urls: &[&str]) -> Result<Self, ClientError> {
        self.node_manager_builder = self.node_manager_builder.with_primary_nodes(urls)?;
        Ok(self)
    }

    /// Adds an IOTA node by its URL with optional jwt and or basic authentication
    pub fn with_node_auth(mut self, url: &str, auth: impl Into<Option<NodeAuth>>) -> Result<Self, ClientError> {
        self.node_manager_builder = self.node_manager_builder.with_node_auth(url, auth)?;
        Ok(self)
    }

    /// Adds a list of IOTA nodes by their URLs.
    pub fn with_nodes(mut self, urls: &[&str]) -> Result<Self, ClientError> {
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

    /// Sets the default request timeout.
    pub fn with_api_timeout(mut self, timeout: Duration) -> Self {
        self.api_timeout = timeout;
        self
    }

    /// Set User-Agent header for requests
    /// Default is "iota-client/{version}"
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.node_manager_builder = self.node_manager_builder.with_user_agent(user_agent);
        self
    }

    /// Set maximum parallel API requests.
    #[cfg(not(target_family = "wasm"))]
    pub fn with_max_parallel_api_requests(mut self, max_parallel_api_requests: usize) -> Self {
        self.max_parallel_api_requests = max_parallel_api_requests;
        self
    }

    /// Set the protocol parameters.
    pub fn with_protocol_parameters(mut self, protocol_parameters: ProtocolParameters) -> Self {
        self.protocol_parameters.replace(protocol_parameters);
        self
    }

    /// Build the Client instance.
    #[cfg(not(target_family = "wasm"))]
    pub async fn finish(self) -> Result<Client, ClientError> {
        use tokio::sync::RwLock;

        let node_sync_interval = self.node_manager_builder.node_sync_interval;
        let ignore_node_health = self.node_manager_builder.ignore_node_health;
        let nodes: HashSet<Node> = self
            .node_manager_builder
            .primary_nodes
            .iter()
            .chain(self.node_manager_builder.nodes.iter())
            .map(|node| node.clone().into())
            .collect();

        #[cfg(feature = "mqtt")]
        let (mqtt_event_tx, mqtt_event_rx) = tokio::sync::watch::channel(MqttEvent::Connected);

        let client_inner = Arc::new(ClientInner {
            // Initially assume all nodes are healthy, so `fetch_network_info()` works. `sync_nodes()` will afterwards
            // update the healthy nodes.
            node_manager: RwLock::new(self.node_manager_builder.build(nodes.clone())),
            api_timeout: RwLock::new(self.api_timeout),
            #[cfg(feature = "mqtt")]
            mqtt: super::MqttInner {
                client: Default::default(),
                topic_handlers: Default::default(),
                broker_options: RwLock::new(self.broker_options),
                sender: RwLock::new(mqtt_event_tx),
                receiver: RwLock::new(mqtt_event_rx),
            },
            request_pool: crate::client::request_pool::RequestPool::new(self.max_parallel_api_requests),
        });

        let network_info = match self.protocol_parameters {
            Some(protocol_parameters) => NetworkInfo {
                protocol_parameters,
                tangle_time: None,
            },
            None => client_inner.fetch_network_info().await?,
        };

        let client = Client {
            inner: client_inner,
            network_info: Arc::new(RwLock::new(network_info)),
            _sync_handle: Arc::new(RwLock::new(super::SyncHandle(None))),
        };

        client.sync_nodes(&nodes, ignore_node_health).await?;
        let client_clone = client.clone();

        let sync_handle = tokio::spawn(async move {
            client_clone
                .start_sync_process(nodes, node_sync_interval, ignore_node_health)
                .await
        });
        *client._sync_handle.write().await = super::SyncHandle(Some(sync_handle));

        Ok(client)
    }

    /// Build the Client instance.
    #[cfg(target_family = "wasm")]
    pub async fn finish(self) -> Result<Client, ClientError> {
        use tokio::sync::RwLock;

        #[cfg(feature = "mqtt")]
        let (mqtt_event_tx, mqtt_event_rx) = tokio::sync::watch::channel(MqttEvent::Connected);

        let client_inner = ClientInner {
            node_manager: RwLock::new(self.node_manager_builder.build(HashSet::new())),
            api_timeout: RwLock::new(self.api_timeout),
            #[cfg(feature = "mqtt")]
            mqtt: super::MqttInner {
                client: Default::default(),
                topic_handlers: Default::default(),
                broker_options: RwLock::new(self.broker_options),
                sender: RwLock::new(mqtt_event_tx),
                receiver: RwLock::new(mqtt_event_rx),
            },
            last_sync: tokio::sync::Mutex::new(None),
        };

        let network_info = match self.protocol_parameters {
            Some(protocol_parameters) => NetworkInfo {
                protocol_parameters,
                tangle_time: None,
            },
            None => client_inner.fetch_network_info().await?,
        };

        let client = Client {
            inner: Arc::new(client_inner),
            network_info: Arc::new(RwLock::new(network_info)),
        };

        Ok(client)
    }

    pub async fn from_client(client: &Client) -> Self {
        Self {
            node_manager_builder: NodeManagerBuilder::from(&*client.node_manager.read().await),
            #[cfg(feature = "mqtt")]
            broker_options: *client.mqtt.broker_options.read().await,
            protocol_parameters: Some(client.network_info.read().await.protocol_parameters.clone()),
            api_timeout: client.get_timeout().await,
            #[cfg(not(target_family = "wasm"))]
            max_parallel_api_requests: client.request_pool.size().await,
        }
    }
}

/// Struct containing network related information
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInfo {
    /// Protocol parameters.
    pub protocol_parameters: ProtocolParameters,
    /// The current tangle time.
    #[serde(skip)]
    pub tangle_time: Option<u64>,
}

impl NetworkInfo {
    pub fn with_protocol_parameters(mut self, protocol_parameters: impl Into<ProtocolParameters>) -> Self {
        self.protocol_parameters = protocol_parameters.into();
        self
    }

    pub fn with_tangle_time(mut self, tangle_time: u64) -> Self {
        self.tangle_time = Some(tangle_time);
        self
    }
}
