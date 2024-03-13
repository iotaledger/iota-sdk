// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The node manager that takes care of sending requests with healthy nodes and quorum if enabled

pub mod builder;
pub(crate) mod http_client;
/// Structs for nodes
pub mod node;
pub(crate) mod syncing;

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::RwLock,
    time::Duration,
};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use self::{http_client::HttpClient, node::Node};
use super::ClientInner;
#[cfg(not(target_family = "wasm"))]
use crate::client::request_pool::RateLimitExt;
use crate::{
    client::{error::ClientError, node_api::core::routes::NodeInfoResponse, node_manager::builder::NodeManagerBuilder},
    types::api::core::InfoResponse,
};

// The node manager takes care of selecting node(s) for requests until a result is returned or if quorum is enabled it
// will send the requests for some endpoints to multiple nodes and compares the results.
pub struct NodeManager {
    pub(crate) primary_nodes: Vec<Node>,
    pub(crate) nodes: HashSet<Node>,
    pub(crate) ignore_node_health: bool,
    node_sync_interval: Duration,
    pub(crate) healthy_nodes: RwLock<HashSet<Node>>,
    quorum: bool,
    min_quorum_size: usize,
    quorum_threshold: usize,
    pub(crate) http_client: HttpClient,
}

impl Debug for NodeManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("NodeManager");
        d.field("primary_nodes", &self.primary_nodes);
        d.field("nodes", &self.nodes);
        d.field("ignore_node_health", &self.ignore_node_health);
        d.field("node_sync_interval", &self.node_sync_interval);
        d.field("healthy_nodes", &self.healthy_nodes);
        d.field("quorum", &self.quorum);
        d.field("min_quorum_size", &self.min_quorum_size);
        d.field("quorum_threshold", &self.quorum_threshold).finish()
    }
}

impl ClientInner {
    pub(crate) async fn get_request<T: DeserializeOwned + Debug + Serialize>(
        &self,
        path: &str,
        query: Option<&str>,
        need_quorum: bool,
    ) -> Result<T, ClientError> {
        let node_manager = self.node_manager.read().await;
        let request = node_manager.get_request(path, query, self.get_timeout().await, need_quorum);
        #[cfg(not(target_family = "wasm"))]
        let request = request.rate_limit(&self.request_pool);
        request.await
    }

    pub(crate) async fn get_request_bytes(&self, path: &str, query: Option<&str>) -> Result<Vec<u8>, ClientError> {
        let node_manager = self.node_manager.read().await;
        let request = node_manager.get_request_bytes(path, query, self.get_timeout().await);
        #[cfg(not(target_family = "wasm"))]
        let request = request.rate_limit(&self.request_pool);
        request.await
    }

    pub(crate) async fn post_request<T: DeserializeOwned>(&self, path: &str, json: Value) -> Result<T, ClientError> {
        let node_manager = self.node_manager.read().await;
        let request = node_manager.post_request(path, self.get_timeout().await, json);
        #[cfg(not(target_family = "wasm"))]
        let request = request.rate_limit(&self.request_pool);
        request.await
    }

    pub(crate) async fn post_request_bytes<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &[u8],
    ) -> Result<T, ClientError> {
        let node_manager = self.node_manager.read().await;
        let request = node_manager.post_request_bytes(path, self.get_timeout().await, body);
        #[cfg(not(target_family = "wasm"))]
        let request = request.rate_limit(&self.request_pool);
        request.await
    }
}

impl NodeManager {
    pub(crate) fn builder() -> NodeManagerBuilder {
        NodeManagerBuilder::new()
    }

    fn get_nodes(&self, path: &str, query: Option<&str>) -> Result<Vec<Node>, ClientError> {
        let mut nodes_with_modified_url: Vec<Node> = Vec::new();

        // Set primary nodes first, so they will also be used first for requests.
        for primary_node in &self.primary_nodes {
            if !nodes_with_modified_url.iter().any(|n| n.url == primary_node.url) {
                nodes_with_modified_url.push(primary_node.clone());
            }
        }

        // Add other nodes in random order, so they are not always used in the same order
        let nodes_random_order = if !self.ignore_node_health {
            #[cfg(not(target_family = "wasm"))]
            {
                self.healthy_nodes
                    .read()
                    .map_err(|_| ClientError::PoisonError)?
                    .iter()
                    .cloned()
                    .collect()
            }
            #[cfg(target_family = "wasm")]
            {
                self.nodes.clone()
            }
        } else {
            self.nodes.clone()
        };

        // Add remaining nodes in random order
        for node in nodes_random_order {
            if !nodes_with_modified_url.iter().any(|n| n.url == node.url) {
                nodes_with_modified_url.push(node);
            }
        }

        // remove disabled nodes
        nodes_with_modified_url.retain(|n| !n.disabled);

        if nodes_with_modified_url.is_empty() {
            return Err(ClientError::HealthyNodePoolEmpty);
        }

        // Set path and query parameters
        for node in &mut nodes_with_modified_url {
            if node.url.path().ends_with('/') {
                node.url.set_path(&format!("{}{}", node.url.path(), path));
            } else {
                node.url.set_path(&format!("{}/{}", node.url.path(), path));
            }
            node.url.set_query(query);
            if let Some(auth) = &node.auth {
                if let Some((name, password)) = &auth.basic_auth_name_pwd {
                    node.url
                        .set_username(name)
                        .map_err(|_| ClientError::UrlAuth("username"))?;
                    node.url
                        .set_password(Some(password))
                        .map_err(|_| ClientError::UrlAuth("password"))?;
                }
            }
        }

        Ok(nodes_with_modified_url)
    }

    pub(crate) async fn get_request<T: DeserializeOwned + Debug + Serialize>(
        &self,
        path: &str,
        query: Option<&str>,
        timeout: Duration,
        need_quorum: bool,
    ) -> Result<T, ClientError> {
        let mut result: HashMap<String, usize> = HashMap::new();
        // Get node urls and set path
        let nodes = self.get_nodes(path, query)?;
        if self.quorum && need_quorum && nodes.len() < self.min_quorum_size {
            return Err(ClientError::QuorumPoolSizeError {
                available_nodes: nodes.len(),
                minimum_threshold: self.min_quorum_size,
            });
        }

        // Track amount of results for quorum
        let mut result_counter = 0;
        let mut error: Option<ClientError> = None;
        // Send requests parallel for quorum
        #[cfg(target_family = "wasm")]
        let wasm = true;
        #[cfg(not(target_family = "wasm"))]
        let wasm = false;
        if !wasm && self.quorum && need_quorum && query.is_none() {
            #[cfg(not(target_family = "wasm"))]
            {
                let mut tasks = Vec::new();
                for (index, node) in nodes.into_iter().enumerate() {
                    if index < self.min_quorum_size {
                        let client_ = self.http_client.clone();
                        tasks.push(async move { tokio::spawn(async move { client_.get(&node, timeout).await }).await });
                    }
                }
                for res in futures::future::try_join_all(tasks).await? {
                    match res {
                        Ok(res) => (res.into_text().await).map_or_else(
                            |_| {
                                log::warn!("couldn't convert node response to text");
                            },
                            |res_text| {
                                let counters = result.entry(res_text).or_insert(0);
                                *counters += 1;
                                result_counter += 1;
                            },
                        ),
                        Err(err) => {
                            error.replace(err.into());
                        }
                    }
                }
            }
        } else {
            // Send requests
            for node in &nodes {
                match self.http_client.get(node, timeout).await {
                    Ok(res) => {
                        // Handle node_info extra because we also want to return the url
                        if path == crate::client::node_api::core::routes::INFO_PATH {
                            let info: InfoResponse = res.into_json().await?;
                            let node_info = NodeInfoResponse {
                                info,
                                url: format!("{}://{}", node.url.scheme(), node.url.host_str().unwrap_or("")),
                            };
                            let serde_res = serde_json::to_string(&node_info)?;
                            return Ok(serde_json::from_str(&serde_res)?);
                        }

                        match res.into_json::<T>().await {
                            Ok(result_data) => {
                                let counters = result.entry(serde_json::to_string(&result_data)?).or_insert(0);
                                *counters += 1;
                                result_counter += 1;
                                // Without quorum it's enough if we got one response
                                if !self.quorum
                                    || result_counter >= self.min_quorum_size
                                    || !need_quorum
                                    // with query we ignore quorum because the nodes can store a different amount of history
                                    || query.is_some()
                                {
                                    break;
                                }
                            }
                            Err(e) => {
                                error.replace(e.into());
                            }
                        }
                    }
                    Err(err) => {
                        error.replace(err.into());
                    }
                }
            }
        }

        // Safe unwrap, there are nodes because we throw on empty nodepool.
        // Each node will throw an error or return Ok()
        let res = result.into_iter().max_by_key(|v| v.1).ok_or_else(|| error.unwrap())?;

        // Return if quorum is false or check if quorum was reached
        if !self.quorum
            || res.1 as f64 >= self.min_quorum_size as f64 * (self.quorum_threshold as f64 / 100.0)
            || !need_quorum
            // with query we ignore quorum because the nodes can store a different amount of history
            || query.is_some()
        {
            Ok(serde_json::from_str(&res.0)?)
        } else {
            Err(ClientError::QuorumThresholdError {
                quorum_size: res.1,
                minimum_threshold: self.min_quorum_size,
            })
        }
    }

    // Only used for api/core/v3/blocks/{blockID}, that's why we don't need the quorum stuff
    pub(crate) async fn get_request_bytes(
        &self,
        path: &str,
        query: Option<&str>,
        timeout: Duration,
    ) -> Result<Vec<u8>, ClientError> {
        // Get node urls and set path
        let nodes = self.get_nodes(path, query)?;
        let mut error = None;
        // Send requests
        for node in nodes {
            match self.http_client.get_bytes(node, timeout).await {
                Ok(res) => {
                    match res.into_bytes().await {
                        Ok(res_text) => return Ok(res_text),
                        Err(e) => error.replace(e.into()),
                    };
                }
                Err(err) => {
                    error.replace(err.into());
                }
            }
        }
        // Safe unwrap, there are nodes because we throw on empty nodepool.
        // Each node will throw an error or return Ok()
        Err(error.unwrap())
    }

    pub(crate) async fn post_request_bytes<T: DeserializeOwned>(
        &self,
        path: &str,
        timeout: Duration,
        body: &[u8],
    ) -> Result<T, ClientError> {
        let nodes = self.get_nodes(path, None)?;
        let mut error = None;
        // Send requests
        for node in nodes {
            match self.http_client.post_bytes(node, timeout, body).await {
                Ok(res) => {
                    match res.into_json::<T>().await {
                        Ok(res) => return Ok(res),
                        Err(e) => error.replace(e.into()),
                    };
                }
                Err(e) => {
                    error.replace(ClientError::Node(e));
                }
            }
        }
        // Safe unwrap, there are nodes because we throw on empty nodepool.
        // Each node will throw an error or return Ok()
        Err(error.unwrap())
    }

    pub(crate) async fn post_request<T: DeserializeOwned>(
        &self,
        path: &str,
        timeout: Duration,
        json: Value,
    ) -> Result<T, ClientError> {
        let nodes = self.get_nodes(path, None)?;
        let mut error = None;
        // Send requests
        for node in nodes {
            match self.http_client.post_json(node, timeout, json.clone()).await {
                Ok(res) => {
                    match res.into_json::<T>().await {
                        Ok(res) => return Ok(res),
                        Err(e) => error.replace(e.into()),
                    };
                }
                Err(e) => {
                    error.replace(ClientError::Node(e));
                }
            }
        }
        // Safe unwrap, there are nodes because we throw on empty nodepool.
        // Each node will throw an error or return Ok()
        Err(error.unwrap())
    }
}
