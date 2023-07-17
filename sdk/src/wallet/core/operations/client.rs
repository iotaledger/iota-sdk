// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

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
    Url,
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
            remote_pow_timeout,
            #[cfg(not(target_family = "wasm"))]
            pow_worker_count,
        } = client_options;
        self.client
            .update_node_manager(node_manager_builder.build(HashMap::new()))
            .await?;
        *self.client.network_info.write().await = network_info;
        *self.client.api_timeout.write().await = api_timeout;
        *self.client.remote_pow_timeout.write().await = remote_pow_timeout;
        #[cfg(not(target_family = "wasm"))]
        {
            *self.client.pow_worker_count.write().await = pow_worker_count;
        }
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

        if let Some(primary_pow_node) = &node_manager_builder.primary_pow_node {
            let (node_url, disabled) = match &primary_pow_node {
                NodeDto::Url(node_url) => (node_url, false),
                NodeDto::Node(node) => (&node.url, node.disabled),
            };

            if node_url == &url {
                node_manager_builder.primary_pow_node = Some(NodeDto::Node(Node {
                    url: url.clone(),
                    auth: auth.clone(),
                    disabled,
                }));
            }
        }

        if let Some(permanodes) = &node_manager_builder.permanodes {
            let mut new_permanodes = HashSet::new();
            for node in permanodes.iter() {
                let (node_url, disabled) = match &node {
                    NodeDto::Url(node_url) => (node_url, false),
                    NodeDto::Node(node) => (&node.url, node.disabled),
                };

                if node_url == &url {
                    new_permanodes.insert(NodeDto::Node(Node {
                        url: url.clone(),
                        auth: auth.clone(),
                        disabled,
                    }));
                } else {
                    new_permanodes.insert(node.clone());
                }
            }
            node_manager_builder.permanodes = Some(new_permanodes);
        }

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

        for account in self.accounts.write().await.iter_mut() {
            account.update_account_bech32_hrp().await?;
        }

        Ok(())
    }
}
