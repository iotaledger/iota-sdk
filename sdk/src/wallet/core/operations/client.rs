// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

use url::Url;

use super::storage::SaveLoadWallet;
use crate::{
    client::{
        node_manager::{
            builder::NodeManagerBuilder,
            node::{Node, NodeAuth, NodeDto},
        },
        secret::SecretManage,
        Client, ClientBuilder,
    },
    wallet::{Wallet, WalletBuilder},
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
    crate::client::Error: From<S::Error>,
    crate::wallet::Error: From<S::Error>,
    WalletBuilder<S>: SaveLoadWallet,
{
    pub async fn set_client_options(&self, client_options: ClientBuilder) -> crate::wallet::Result<()> {
        let ClientBuilder {
            node_manager_builder,
            #[cfg(feature = "mqtt")]
            broker_options,
            network_info,
            api_timeout,
            #[cfg(not(target_family = "wasm"))]
            max_parallel_api_requests,
        } = client_options;
        self.client
            .update_node_manager(node_manager_builder.build(HashMap::new()))
            .await?;
        *self.client.network_info.write().await = network_info;
        *self.client.api_timeout.write().await = api_timeout;
        #[cfg(not(target_family = "wasm"))]
        self.client.request_pool.resize(max_parallel_api_requests).await;
        #[cfg(feature = "mqtt")]
        {
            *self.client.mqtt.broker_options.write().await = broker_options;
        }
        #[cfg(feature = "storage")]
        {
            WalletBuilder::from_wallet(self)
                .await
                .save(&*self.storage_manager.read().await)
                .await?;
        }
        Ok(())
    }

    /// Update the authentication for a node.
    pub async fn update_node_auth(&self, url: Url, auth: Option<NodeAuth>) -> crate::wallet::Result<()> {
        log::debug!("[update_node_auth]");
        let mut node_manager_builder = NodeManagerBuilder::from(&*self.client.node_manager.read().await);

        if let Some(primary_node) = &node_manager_builder.primary_node {
            let (node_url, disabled) = match &primary_node {
                NodeDto::Url(node_url) => (node_url, false),
                NodeDto::Node(node) => (&node.url, node.disabled),
            };

            if node_url == &url {
                node_manager_builder.primary_node = Some(NodeDto::Node(Node {
                    url: url.clone(),
                    auth: auth.clone(),
                    disabled,
                }));
            }
        }

        node_manager_builder.permanodes = node_manager_builder
            .permanodes
            .into_iter()
            .map(|node| {
                let (node_url, disabled) = match &node {
                    NodeDto::Url(node_url) => (node_url, false),
                    NodeDto::Node(node) => (&node.url, node.disabled),
                };

                if node_url == &url {
                    NodeDto::Node(Node {
                        url: url.clone(),
                        auth: auth.clone(),
                        disabled,
                    })
                } else {
                    node
                }
            })
            .collect();

        let mut new_nodes = HashSet::new();
        for node in node_manager_builder.nodes.iter() {
            let (node_url, disabled) = match &node {
                NodeDto::Url(node_url) => (node_url, false),
                NodeDto::Node(node) => (&node.url, node.disabled),
            };

            if node_url == &url {
                new_nodes.insert(NodeDto::Node(Node {
                    url: url.clone(),
                    auth: auth.clone(),
                    disabled,
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
                .save(&*self.storage_manager.read().await)
                .await?;
        }

        self.client
            .update_node_manager(node_manager_builder.build(HashMap::new()))
            .await?;

        self.update_bech32_hrp().await?;

        Ok(())
    }
}
