// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(not(target_family = "wasm"))]
use {
    crate::{client::NetworkInfo, types::block::PROTOCOL_VERSION},
    std::{collections::HashSet, time::Duration},
    tokio::time::sleep,
};

use super::{Node, NodeManager};
use crate::client::{Client, ClientError, ClientInner};

impl ClientInner {
    /// Get a node candidate from the healthy node pool.
    pub async fn get_node(&self) -> Result<Node, ClientError> {
        if let Some(primary_node) = self.node_manager.read().await.primary_nodes.first() {
            return Ok(primary_node.clone());
        }

        let pool = self.node_manager.read().await.nodes.clone();

        pool.into_iter().next().ok_or(ClientError::HealthyNodePoolEmpty)
    }

    /// returns the unhealthy nodes.
    #[cfg(not(target_family = "wasm"))]
    pub async fn unhealthy_nodes(&self) -> HashSet<Node> {
        let node_manager = self.node_manager.read().await;

        node_manager
            .healthy_nodes
            .read()
            .map_or(HashSet::new(), |healthy_nodes| {
                node_manager
                    .nodes
                    .iter()
                    .filter(|node| !healthy_nodes.contains(node))
                    .cloned()
                    .collect()
            })
    }
}

#[cfg(not(target_family = "wasm"))]
impl Client {
    /// Sync the node lists per node_sync_interval milliseconds
    pub(crate) async fn start_sync_process(
        &self,
        nodes: HashSet<Node>,
        node_sync_interval: Duration,
        ignore_node_health: bool,
    ) {
        loop {
            // Delay first since the first `sync_nodes` call is made by the builder to ensure the node list is
            // filled before the client is used.
            sleep(node_sync_interval).await;
            if let Err(e) = self.sync_nodes(&nodes, ignore_node_health).await {
                log::warn!("Syncing nodes failed: {e}");
            }
        }
    }

    pub(crate) async fn sync_nodes(&self, nodes: &HashSet<Node>, ignore_node_health: bool) -> Result<(), ClientError> {
        use std::collections::HashMap;

        log::debug!("sync_nodes");
        let mut healthy_nodes = HashSet::new();
        let mut network_nodes: HashMap<String, Vec<(_, Node, Option<u64>)>> = HashMap::new();

        for node in nodes {
            // Put the healthy node url into the network_nodes
            if node.permanode {
                match Self::get_permanode_info(node.clone()).await {
                    Ok(info) => {
                        if info.is_healthy || ignore_node_health {
                            // Unwrap: We should always have parameters for this version. If we don't we can't recover.
                            let protocol_parameters = info
                                .protocol_parameters_by_version(PROTOCOL_VERSION)
                                .expect("missing v3 protocol parameters")
                                .parameters
                                .clone();
                            let network_name = protocol_parameters.network_name();
                            match network_nodes.get_mut(network_name) {
                                Some(network_node_entry) => {
                                    network_node_entry.push((protocol_parameters, node.clone(), None));
                                }
                                None => {
                                    network_nodes.insert(
                                        network_name.to_owned(),
                                        vec![(protocol_parameters, node.clone(), None)],
                                    );
                                }
                            }
                        } else {
                            log::warn!("{} is not healthy: {:?}", node.url, info);
                        }
                    }
                    Err(err) => {
                        log::error!("Couldn't get node info: {err}");
                    }
                }
            } else {
                match Self::get_info(node.url.as_ref(), node.auth.clone()).await {
                    Ok(info) => {
                        if info.status.is_healthy || ignore_node_health {
                            // Unwrap: We should always have parameters for this version. If we don't we can't recover.
                            let protocol_parameters = info
                                .protocol_parameters_by_version(PROTOCOL_VERSION)
                                .expect("missing v3 protocol parameters")
                                .parameters
                                .clone();
                            let network_name = protocol_parameters.network_name();
                            match network_nodes.get_mut(network_name) {
                                Some(network_node_entry) => {
                                    network_node_entry.push((
                                        protocol_parameters,
                                        node.clone(),
                                        info.status.relative_accepted_tangle_time,
                                    ));
                                }
                                None => {
                                    network_nodes.insert(
                                        network_name.to_owned(),
                                        vec![(
                                            protocol_parameters,
                                            node.clone(),
                                            info.status.relative_accepted_tangle_time,
                                        )],
                                    );
                                }
                            }
                        } else {
                            log::warn!("{} is not healthy: {:?}", node.url, info);
                        }
                    }
                    Err(err) => {
                        log::error!("Couldn't get node info: {err}");
                    }
                }
            }
        }

        // Get the nodes of which the most are in the same network
        if let Some((_network_name, nodes)) = network_nodes.iter().max_by_key(|a| a.1.len()) {
            // Set the protocol_parameters to the parameters that most nodes have in common and only use these nodes as
            // healthy_nodes
            if let Some((parameters, _node_url, tangle_time)) = nodes.first() {
                // Unwrap: We should always have parameters for this version. If we don't we can't recover.
                *self.network_info.write().await = NetworkInfo {
                    protocol_parameters: parameters.clone(),
                    tangle_time: *tangle_time,
                };
            }

            healthy_nodes.extend(nodes.iter().map(|(_info, node_url, _time)| node_url).cloned())
        };

        // Update the sync list.
        *self
            .node_manager
            .read()
            .await
            .healthy_nodes
            .write()
            .map_err(|_| ClientError::PoisonError)? = healthy_nodes;

        Ok(())
    }

    pub async fn update_node_manager(&self, node_manager: NodeManager) -> Result<(), ClientError> {
        let node_sync_interval = node_manager.node_sync_interval;
        let ignore_node_health = node_manager.ignore_node_health;
        let nodes = node_manager
            .primary_nodes
            .iter()
            .chain(node_manager.nodes.iter())
            .cloned()
            .collect();

        *self.node_manager.write().await = node_manager;

        self.sync_nodes(&nodes, ignore_node_health).await?;
        let client = self.clone();

        let sync_handle = tokio::spawn(async move {
            client
                .start_sync_process(nodes, node_sync_interval, ignore_node_health)
                .await
        });

        *self._sync_handle.write().await = crate::client::SyncHandle(Some(sync_handle));
        Ok(())
    }
}

#[cfg(target_family = "wasm")]
impl Client {
    pub async fn update_node_manager(&self, node_manager: NodeManager) -> Result<(), ClientError> {
        *self.node_manager.write().await = node_manager;
        Ok(())
    }
}
