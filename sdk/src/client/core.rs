// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The Client module to connect through HORNET or Bee with API usages

use std::{sync::Arc, time::Duration};

use tokio::sync::RwLock;
#[cfg(feature = "mqtt")]
use {
    crate::client::node_api::mqtt::{BrokerOptions, MqttEvent, TopicHandlerMap},
    rumqttc::AsyncClient as MqttClient,
    tokio::sync::watch::{Receiver as WatchReceiver, Sender as WatchSender},
};

#[cfg(not(target_family = "wasm"))]
use super::request_pool::RequestPool;
#[cfg(target_family = "wasm")]
use crate::client::constants::CACHE_NETWORK_INFO_TIMEOUT_IN_SECONDS;
use crate::{
    client::{
        builder::{ClientBuilder, NetworkInfo},
        node_manager::NodeManager,
        ClientError,
    },
    types::block::{address::Hrp, output::StorageScoreParameters, protocol::ProtocolParameters},
};

/// An IOTA node client.
#[derive(Clone)]
pub struct Client {
    pub(crate) inner: Arc<ClientInner>,
    pub(crate) network_info: Arc<RwLock<NetworkInfo>>,
    #[cfg(not(target_family = "wasm"))]
    pub(crate) _sync_handle: Arc<RwLock<SyncHandle>>,
}

impl core::ops::Deref for Client {
    type Target = ClientInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct ClientInner {
    /// Node manager
    pub(crate) node_manager: RwLock<NodeManager>,
    /// HTTP request timeout.
    pub(crate) api_timeout: RwLock<Duration>,
    #[cfg(feature = "mqtt")]
    pub(crate) mqtt: MqttInner,
    #[cfg(target_family = "wasm")]
    pub(crate) last_sync: tokio::sync::Mutex<Option<u32>>,
    #[cfg(not(target_family = "wasm"))]
    pub(crate) request_pool: RequestPool,
}

#[cfg(not(target_family = "wasm"))]
#[derive(Default)]
pub(crate) struct SyncHandle(pub(crate) Option<tokio::task::JoinHandle<()>>);

#[cfg(not(target_family = "wasm"))]
impl Drop for SyncHandle {
    fn drop(&mut self) {
        if let Some(sync_handle) = self.0.take() {
            sync_handle.abort();
        }
    }
}

#[cfg(feature = "mqtt")]
pub(crate) struct MqttInner {
    /// A MQTT client to subscribe/unsubscribe to topics.
    pub(crate) client: RwLock<Option<MqttClient>>,
    pub(crate) topic_handlers: RwLock<TopicHandlerMap>,
    pub(crate) broker_options: RwLock<BrokerOptions>,
    pub(crate) sender: RwLock<WatchSender<MqttEvent>>,
    pub(crate) receiver: RwLock<WatchReceiver<MqttEvent>>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Client");
        d.field("node_manager", &self.node_manager);
        #[cfg(feature = "mqtt")]
        d.field("broker_options", &self.mqtt.broker_options);
        d.field("network_info", &self.network_info);
        #[cfg(not(target_family = "wasm"))]
        d.field("request_pool", &self.request_pool);
        d.finish()
    }
}

impl Client {
    /// Create the builder to instantiate the IOTA Client.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Gets the network related information such as network_id and if it's the default one, sync it first and set the
    /// NetworkInfo.
    pub async fn get_network_info(&self) -> Result<NetworkInfo, ClientError> {
        // For WASM we don't have the node syncing process, which updates the network_info every 60 seconds, so we
        // request the node info every time, so we don't create invalid transactions/blocks.
        #[cfg(target_family = "wasm")]
        {
            let current_time = crate::client::unix_timestamp_now().as_secs() as u32;
            if let Some(last_sync) = *self.last_sync.lock().await {
                if current_time < last_sync {
                    return Ok(self.network_info.read().await.clone());
                }
            }
            let network_info = self.fetch_network_info().await?;
            *self.network_info.write().await = network_info.clone();

            *self.last_sync.lock().await = Some(current_time + CACHE_NETWORK_INFO_TIMEOUT_IN_SECONDS);

            Ok(network_info)
        }

        #[cfg(not(target_family = "wasm"))]
        Ok(self.network_info.read().await.clone())
    }

    /// Gets the protocol parameters of the node we're connecting to.
    pub async fn get_protocol_parameters(&self) -> Result<ProtocolParameters, ClientError> {
        Ok(self.get_network_info().await?.protocol_parameters)
    }

    /// Gets the protocol version of the node we're connecting to.
    pub async fn get_protocol_version(&self) -> Result<u8, ClientError> {
        Ok(self.get_network_info().await?.protocol_parameters.version())
    }

    /// Gets the network name of the node we're connecting to.
    pub async fn get_network_name(&self) -> Result<String, ClientError> {
        Ok(self.get_network_info().await?.protocol_parameters.network_name().into())
    }

    /// Gets the network id of the node we're connecting to.
    pub async fn get_network_id(&self) -> Result<u64, ClientError> {
        Ok(self.get_network_info().await?.protocol_parameters.network_id())
    }

    /// Gets the bech32 HRP of the node we're connecting to.
    pub async fn get_bech32_hrp(&self) -> Result<Hrp, ClientError> {
        Ok(self.get_network_info().await?.protocol_parameters.bech32_hrp())
    }

    /// Gets the storage score parameters of the node we're connecting to.
    pub async fn get_storage_score_parameters(&self) -> Result<StorageScoreParameters, ClientError> {
        Ok(self
            .get_network_info()
            .await?
            .protocol_parameters
            .storage_score_parameters())
    }

    /// Gets the token supply of the node we're connecting to.
    pub async fn get_token_supply(&self) -> Result<u64, ClientError> {
        Ok(self.get_network_info().await?.protocol_parameters.token_supply())
    }

    /// Validates if a bech32 HRP matches the one from the connected network.
    pub async fn bech32_hrp_matches(&self, bech32_hrp: &Hrp) -> Result<(), ClientError> {
        let expected = self.get_bech32_hrp().await?;
        if bech32_hrp != &expected {
            return Err(ClientError::Bech32HrpMismatch {
                provided: bech32_hrp.to_string(),
                expected: expected.to_string(),
            });
        };
        Ok(())
    }
}

impl ClientInner {
    pub(crate) async fn fetch_network_info(&self) -> Result<NetworkInfo, ClientError> {
        let info = self.get_node_info().await?.info;
        let protocol_parameters = info
            .protocol_parameters_by_version(crate::types::block::PROTOCOL_VERSION)
            .expect("missing v3 protocol parameters")
            .parameters
            .clone();
        let network_info = NetworkInfo {
            protocol_parameters,
            tangle_time: info.status.relative_accepted_tangle_time,
        };

        Ok(network_info)
    }

    pub(crate) async fn get_timeout(&self) -> Duration {
        *self.api_timeout.read().await
    }

    /// Resize the client's request pool
    #[cfg(not(target_family = "wasm"))]
    pub async fn resize_request_pool(&self, new_size: usize) {
        self.request_pool.resize(new_size).await;
    }
}
