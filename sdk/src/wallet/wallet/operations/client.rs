// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

#[cfg(feature = "storage")]
use crate::wallet::WalletBuilder;
use crate::{
    client::{
        node_manager::node::{Node, NodeAuth, NodeDto},
        Client, NodeInfoWrapper, Url,
    },
    wallet::{ClientOptions, Wallet},
};

impl Wallet {
    /// Sets the client options for all accounts and sets the new bech32_hrp for the addresses.
    pub async fn set_client_options(&self, options: ClientOptions) -> crate::wallet::Result<()> {
        log::debug!("[set_client_options]");

        let mut client_options = self.client_options.write().await;
        *client_options = options.clone();
        drop(client_options);

        let new_client = options.clone().finish().await?;

        for account in self.accounts.write().await.iter_mut() {
            account.update_account_with_new_client(new_client.clone()).await?;
        }

        #[cfg(feature = "storage")]
        {
            // Update wallet data with new client options
            let wallet_builder = WalletBuilder::from_wallet(self).await.with_client_options(options);

            self.storage_manager
                .lock()
                .await
                .save_wallet_data(&wallet_builder)
                .await?;
        }

        Ok(())
    }

    /// Try to get the Client from the first account and only build a new one if we have no account
    pub async fn get_client(&self) -> crate::wallet::Result<Client> {
        let accounts = self.accounts.read().await;

        let client = match &accounts.first() {
            Some(account) => account.client.clone(),
            None => self.client_options.read().await.clone().finish().await?,
        };

        Ok(client)
    }

    /// Get the used client options.
    pub async fn get_client_options(&self) -> ClientOptions {
        self.client_options.read().await.clone()
    }

    /// Get the node info.
    pub async fn get_node_info(&self) -> crate::wallet::Result<NodeInfoWrapper> {
        let node_info_wrapper = self.get_client().await?.get_info().await?;

        Ok(node_info_wrapper)
    }

    /// Update the authentication for a node.
    pub async fn update_node_auth(&self, url: Url, auth: Option<NodeAuth>) -> crate::wallet::Result<()> {
        log::debug!("[update_node_auth]");
        let mut client_options = self.client_options.write().await;

        if let Some(primary_node) = &client_options.node_manager_builder.primary_node {
            let (node_url, disabled) = match &primary_node {
                NodeDto::Url(node_url) => (node_url, false),
                NodeDto::Node(node) => (&node.url, node.disabled),
            };

            if node_url == &url {
                client_options.node_manager_builder.primary_node = Some(NodeDto::Node(Node {
                    url: url.clone(),
                    auth: auth.clone(),
                    disabled,
                }));
            }
        }

        if let Some(primary_pow_node) = &client_options.node_manager_builder.primary_pow_node {
            let (node_url, disabled) = match &primary_pow_node {
                NodeDto::Url(node_url) => (node_url, false),
                NodeDto::Node(node) => (&node.url, node.disabled),
            };

            if node_url == &url {
                client_options.node_manager_builder.primary_pow_node = Some(NodeDto::Node(Node {
                    url: url.clone(),
                    auth: auth.clone(),
                    disabled,
                }));
            }
        }

        if let Some(permanodes) = &client_options.node_manager_builder.permanodes {
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
            client_options.node_manager_builder.permanodes = Some(new_permanodes);
        }

        let mut new_nodes = HashSet::new();
        for node in client_options.node_manager_builder.nodes.iter() {
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
        client_options.node_manager_builder.nodes = new_nodes;

        let new_client_options = client_options.clone();
        // Need to drop client_options here to prevent a deadlock
        drop(client_options);

        #[cfg(feature = "storage")]
        {
            // Update wallet data with new client options
            let wallet_builder = WalletBuilder::from_wallet(self)
                .await
                .with_client_options(new_client_options.clone());

            self.storage_manager
                .lock()
                .await
                .save_wallet_data(&wallet_builder)
                .await?;
        }

        let new_client = new_client_options.finish().await?;

        for account in self.accounts.write().await.iter_mut() {
            account.update_account_with_new_client(new_client.clone()).await?;
        }

        Ok(())
    }
}
