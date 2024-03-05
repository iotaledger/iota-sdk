// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use url::Url;

use super::storage::SaveLoadWallet;
use crate::{
    client::{
        node_manager::{
            builder::NodeManagerBuilder,
            node::{Node, NodeAuth, NodeDto},
        },
        secret::SecretManage,
        Client, ClientBuilder, ClientError, NetworkInfo,
    },
    wallet::{Wallet, WalletBuilder, WalletError},
};

impl<S: 'static + SecretManage> Wallet<S> {
    pub fn client(&self) -> &Client {
        &self.client
    }

    pub async fn client_options(&self) -> ClientBuilder {
        ClientBuilder::from_client(self.client()).await
    }
}

impl<S: 'static + SecretManage> Wallet<S>
where
    ClientError: From<S::Error>,
    WalletError: From<S::Error>,
    WalletBuilder<S>: SaveLoadWallet,
{
    pub async fn set_client_options(&self, client_options: ClientBuilder) -> Result<(), WalletError> {
        let ClientBuilder {
            node_manager_builder,
            #[cfg(feature = "mqtt")]
            broker_options,
            protocol_parameters,
            api_timeout,
            #[cfg(not(target_family = "wasm"))]
            max_parallel_api_requests,
        } = client_options;

        // Only check bech32 if something in the node_manager_builder changed
        let change_in_node_manager = self.client_options().await.node_manager_builder != node_manager_builder;

        self.client
            .update_node_manager(node_manager_builder.build(HashSet::new()))
            .await?;
        *self.client.api_timeout.write().await = api_timeout;
        #[cfg(not(target_family = "wasm"))]
        self.client.request_pool.resize(max_parallel_api_requests).await;
        #[cfg(feature = "mqtt")]
        {
            *self.client.mqtt.broker_options.write().await = broker_options;
        }

        if change_in_node_manager {
            if let Ok(node_info) = self.client.get_node_info().await {
                let params = &node_info.info.latest_protocol_parameters().parameters;

                *self.client.network_info.write().await = NetworkInfo {
                    protocol_parameters: params.clone(),
                    tangle_time: node_info.info.status.relative_accepted_tangle_time,
                };
            } else if let Some(protocol_parameters) = protocol_parameters {
                *self.client.network_info.write().await = NetworkInfo {
                    protocol_parameters,
                    tangle_time: None,
                };
            }

            self.update_address_hrp().await?;
        }

        #[cfg(feature = "storage")]
        {
            WalletBuilder::from_wallet(self)
                .await
                .save(self.storage_manager())
                .await?;
        }
        Ok(())
    }

    /// Update the authentication for a node.
    pub async fn update_node_auth(&self, url: Url, auth: Option<NodeAuth>) -> Result<(), WalletError> {
        log::debug!("[update_node_auth]");
        let mut node_manager_builder = NodeManagerBuilder::from(&*self.client.node_manager.read().await);

        node_manager_builder.primary_nodes = node_manager_builder
            .primary_nodes
            .into_iter()
            .map(|node| {
                let (node_url, disabled, permanode) = match &node {
                    NodeDto::Url(node_url) => (node_url, false, false),
                    NodeDto::Node(node) => (&node.url, node.disabled, node.permanode),
                };

                if node_url == &url {
                    NodeDto::Node(Node {
                        url: url.clone(),
                        auth: auth.clone(),
                        disabled,
                        permanode,
                    })
                } else {
                    node
                }
            })
            .collect();

        let mut new_nodes = HashSet::new();
        for node in node_manager_builder.nodes.iter() {
            let (node_url, disabled, permanode) = match &node {
                NodeDto::Url(node_url) => (node_url, false, false),
                NodeDto::Node(node) => (&node.url, node.disabled, node.permanode),
            };

            if node_url == &url {
                new_nodes.insert(NodeDto::Node(Node {
                    url: url.clone(),
                    auth: auth.clone(),
                    disabled,
                    permanode,
                }));
            } else {
                new_nodes.insert(node.clone());
            }
        }
        node_manager_builder.nodes = new_nodes;

        #[cfg(feature = "storage")]
        {
            WalletBuilder::from_wallet(self)
                .await
                .save(self.storage_manager())
                .await?;
        }

        self.client
            .update_node_manager(node_manager_builder.build(HashSet::new()))
            .await?;

        self.update_address_hrp().await?;

        Ok(())
    }
}
