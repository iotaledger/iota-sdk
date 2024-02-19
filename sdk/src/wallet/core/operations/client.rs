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
            mut network_info,
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
            // Update the protocol of the network_info to not have the default data, which can be wrong
            // Ignore errors, because there might be no node at all and then it should still not error
            if let Ok(info) = self.client.get_info().await {
                network_info.protocol_parameters = info.node_info.latest_protocol_parameters().parameters.clone();
            }
            *self.client.network_info.write().await = network_info;

            self.update_bech32_hrp().await?;
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
    pub async fn update_node_auth(&self, url: Url, auth: Option<NodeAuth>) -> crate::wallet::Result<()> {
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

        self.update_bech32_hrp().await?;

        Ok(())
    }
}
