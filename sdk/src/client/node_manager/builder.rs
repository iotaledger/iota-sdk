// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The node manager that takes care of sending requests with healthy nodes and quorum if enabled

use std::{collections::HashSet, sync::RwLock, time::Duration};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::client::{
    constants::{DEFAULT_MIN_QUORUM_SIZE, DEFAULT_QUORUM_THRESHOLD, DEFAULT_USER_AGENT, NODE_SYNC_INTERVAL},
    error::ClientError,
    node_manager::{
        http_client::HttpClient,
        node::{Node, NodeAuth, NodeDto},
        NodeManager,
    },
};

/// Node manager builder
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagerBuilder {
    /// Nodes which will be tried first for all requests
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub primary_nodes: Vec<NodeDto>,
    /// Nodes
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub nodes: HashSet<NodeDto>,
    /// If the node health should be ignored
    #[serde(default)]
    pub ignore_node_health: bool,
    /// Interval in which nodes will be checked for their sync status and the [NetworkInfo](crate::client::NetworkInfo)
    /// gets updated
    #[serde(default = "default_node_sync_interval")]
    pub node_sync_interval: Duration,
    /// If node quorum is enabled. Will compare the responses from multiple nodes and only returns the response if
    /// `quorum_threshold`% of the nodes return the same one
    #[serde(default)]
    pub quorum: bool,
    /// Minimum amount of nodes required for request when quorum is enabled
    #[serde(default = "default_min_quorum_size")]
    pub min_quorum_size: usize,
    /// % of nodes that have to return the same response so it gets accepted
    #[serde(default = "default_quorum_threshold")]
    pub quorum_threshold: usize,
    /// The User-Agent header for requests
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
}

fn default_user_agent() -> String {
    DEFAULT_USER_AGENT.to_string()
}

fn default_node_sync_interval() -> Duration {
    NODE_SYNC_INTERVAL
}

fn default_min_quorum_size() -> usize {
    DEFAULT_MIN_QUORUM_SIZE
}

fn default_quorum_threshold() -> usize {
    DEFAULT_QUORUM_THRESHOLD
}

impl NodeManagerBuilder {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn with_primary_node(mut self, mut node: Node) -> Result<Self, ClientError> {
        let mut url = validate_url(node.url.clone())?;
        if let Some(auth) = &node.auth {
            if let Some((name, password)) = &auth.basic_auth_name_pwd {
                url.set_username(name).map_err(|_| ClientError::UrlAuth("username"))?;
                url.set_password(Some(password))
                    .map_err(|_| ClientError::UrlAuth("password"))?;
            }
            node.url = url;
        }
        self.primary_nodes.push(NodeDto::Node(node));
        Ok(self)
    }
    pub(crate) fn with_primary_nodes(mut self, urls: &[&str]) -> Result<Self, ClientError> {
        for url in urls {
            let url = validate_url(Url::parse(url)?)?;
            self.primary_nodes.push(NodeDto::Node(Node {
                url,
                auth: None,
                disabled: false,
                permanode: false,
            }));
        }
        Ok(self)
    }

    pub(crate) fn with_node(mut self, url: &str) -> Result<Self, ClientError> {
        let url = validate_url(Url::parse(url)?)?;
        self.nodes.insert(NodeDto::Node(Node {
            url,
            auth: None,
            disabled: false,
            permanode: false,
        }));
        Ok(self)
    }

    pub(crate) fn with_node_auth(mut self, url: &str, auth: impl Into<Option<NodeAuth>>) -> Result<Self, ClientError> {
        let mut url = validate_url(Url::parse(url)?)?;
        let auth = auth.into();
        if let Some(auth) = &auth {
            if let Some((name, password)) = &auth.basic_auth_name_pwd {
                url.set_username(name).map_err(|_| ClientError::UrlAuth("username"))?;
                url.set_password(Some(password))
                    .map_err(|_| ClientError::UrlAuth("password"))?;
            }
        }
        self.nodes.insert(NodeDto::Node(Node {
            url,
            auth,
            disabled: false,
            permanode: false,
        }));
        Ok(self)
    }

    pub(crate) fn with_nodes(mut self, urls: &[&str]) -> Result<Self, ClientError> {
        for url in urls {
            let url = validate_url(Url::parse(url)?)?;
            self.nodes.insert(NodeDto::Node(Node {
                url,
                auth: None,
                disabled: false,
                permanode: false,
            }));
        }
        Ok(self)
    }

    pub(crate) fn with_ignore_node_health(mut self) -> Self {
        self.ignore_node_health = true;
        self
    }

    pub(crate) fn with_node_sync_interval(mut self, node_sync_interval: Duration) -> Self {
        self.node_sync_interval = node_sync_interval;
        self
    }

    pub(crate) fn with_quorum(mut self, quorum: bool) -> Self {
        self.quorum = quorum;
        self
    }

    pub(crate) fn with_min_quorum_size(mut self, min_quorum_size: usize) -> Self {
        self.min_quorum_size = min_quorum_size;
        self
    }

    pub(crate) fn with_quorum_threshold(mut self, threshold: usize) -> Self {
        self.quorum_threshold = threshold;
        self
    }

    pub(crate) fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    pub(crate) fn build(self, healthy_nodes: HashSet<Node>) -> NodeManager {
        NodeManager {
            primary_nodes: self.primary_nodes.into_iter().map(Into::into).collect(),
            nodes: self.nodes.into_iter().map(Into::into).collect(),
            ignore_node_health: self.ignore_node_health,
            node_sync_interval: self.node_sync_interval,
            healthy_nodes: RwLock::new(healthy_nodes),
            quorum: self.quorum,
            min_quorum_size: self.min_quorum_size,
            quorum_threshold: self.quorum_threshold,
            http_client: HttpClient::new(self.user_agent),
        }
    }
}

impl Default for NodeManagerBuilder {
    fn default() -> Self {
        Self {
            primary_nodes: Vec::new(),
            nodes: HashSet::new(),
            ignore_node_health: false,
            node_sync_interval: NODE_SYNC_INTERVAL,
            quorum: false,
            min_quorum_size: DEFAULT_MIN_QUORUM_SIZE,
            quorum_threshold: DEFAULT_QUORUM_THRESHOLD,
            user_agent: DEFAULT_USER_AGENT.to_string(),
        }
    }
}

/// Validates if the url starts with http or https
pub fn validate_url(url: Url) -> Result<Url, ClientError> {
    if url.scheme() != "http" && url.scheme() != "https" {
        return Err(ClientError::UrlValidation(format!("invalid scheme: {}", url.scheme())));
    }
    Ok(url)
}

impl From<&NodeManager> for NodeManagerBuilder {
    fn from(value: &NodeManager) -> Self {
        Self {
            primary_nodes: value.primary_nodes.iter().cloned().map(NodeDto::Node).collect(),
            nodes: value.nodes.iter().cloned().map(NodeDto::Node).collect(),
            ignore_node_health: value.ignore_node_health,
            node_sync_interval: value.node_sync_interval,
            quorum: value.quorum,
            min_quorum_size: value.min_quorum_size,
            quorum_threshold: value.quorum_threshold,
            user_agent: value.http_client.user_agent.clone(),
        }
    }
}
