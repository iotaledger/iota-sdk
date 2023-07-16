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

#[cfg(target_family = "wasm")]
use crate::client::constants::CACHE_NETWORK_INFO_TIMEOUT_IN_SECONDS;
use crate::{
    client::{
        builder::{ClientBuilder, NetworkInfo},
        error::Result,
        node_manager::NodeManager,
        Error,
    },
    types::block::{address::Hrp, output::RentStructure, protocol::ProtocolParameters},
};

/// An IOTA node client.
#[derive(Clone)]
pub struct Client {
    pub(crate) inner: Arc<ClientInner>,
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
    pub(crate) network_info: RwLock<NetworkInfo>,
    /// HTTP request timeout.
    pub(crate) api_timeout: RwLock<Duration>,
    /// HTTP request timeout for remote PoW API call.
    pub(crate) remote_pow_timeout: RwLock<Duration>,
    /// pow_worker_count for local PoW.
    #[cfg(not(target_family = "wasm"))]
    pub(crate) pow_worker_count: RwLock<Option<usize>>,
    #[cfg(feature = "mqtt")]
    pub(crate) mqtt: MqttInner,
}

#[derive(Default)]
pub(crate) struct SyncHandle(pub(crate) Option<tokio::task::JoinHandle<()>>);

impl Drop for SyncHandle {
    fn drop(&mut self) {
        #[cfg(not(target_family = "wasm"))]
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
        d.field("node_manager", &self.inner.node_manager);
        #[cfg(feature = "mqtt")]
        d.field("broker_options", &self.inner.mqtt.broker_options);
        d.field("network_info", &self.inner.network_info).finish()
    }
}

impl Client {
    /// Create the builder to instantiate the IOTA Client.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}

impl ClientInner {
    /// Gets the network related information such as network_id and min_pow_score
    /// and if it's the default one, sync it first and set the NetworkInfo.
    pub async fn get_network_info(&self) -> Result<NetworkInfo> {
        // For WASM we don't have the node syncing process, which updates the network_info every 60 seconds, but the Pow
        // difficulty or the byte cost could change via a milestone, so we request the node info every time, so we don't
        // create invalid transactions/blocks.
        #[cfg(target_family = "wasm")]
        {
            lazy_static::lazy_static! {
                static ref LAST_SYNC: std::sync::Mutex<Option<u32>> = std::sync::Mutex::new(None);
            };
            let current_time = crate::utils::unix_timestamp_now().as_secs() as u32;
            if let Some(last_sync) = *LAST_SYNC.lock().unwrap() {
                if current_time < last_sync {
                    return Ok(self.network_info.read().await.clone());
                }
            }
            let info = self.get_info().await?.node_info;
            let mut client_network_info = self.network_info.write().await;
            client_network_info.protocol_parameters = info.protocol.clone();

            *LAST_SYNC.lock().unwrap() = Some(current_time + CACHE_NETWORK_INFO_TIMEOUT_IN_SECONDS);
        }

        Ok(self.network_info.read().await.clone())
    }

    /// Gets the protocol parameters of the node we're connecting to.
    pub async fn get_protocol_parameters(&self) -> Result<ProtocolParameters> {
        Ok(self.get_network_info().await?.protocol_parameters)
    }

    /// Gets the protocol version of the node we're connecting to.
    pub async fn get_protocol_version(&self) -> Result<u8> {
        Ok(self.get_network_info().await?.protocol_parameters.protocol_version())
    }

    /// Gets the network name of the node we're connecting to.
    pub async fn get_network_name(&self) -> Result<String> {
        Ok(self.get_network_info().await?.protocol_parameters.network_name().into())
    }

    /// Gets the network id of the node we're connecting to.
    pub async fn get_network_id(&self) -> Result<u64> {
        Ok(self.get_network_info().await?.protocol_parameters.network_id())
    }

    /// Gets the bech32 HRP of the node we're connecting to.
    pub async fn get_bech32_hrp(&self) -> Result<Hrp> {
        Ok(*self.get_network_info().await?.protocol_parameters.bech32_hrp())
    }

    /// Gets the minimum pow score of the node we're connecting to.
    pub async fn get_min_pow_score(&self) -> Result<u32> {
        Ok(self.get_network_info().await?.protocol_parameters.min_pow_score())
    }

    /// Gets the below maximum depth of the node we're connecting to.
    pub async fn get_below_max_depth(&self) -> Result<u8> {
        Ok(self.get_network_info().await?.protocol_parameters.below_max_depth())
    }

    /// Gets the rent structure of the node we're connecting to.
    pub async fn get_rent_structure(&self) -> Result<RentStructure> {
        Ok(*self.get_network_info().await?.protocol_parameters.rent_structure())
    }

    /// Gets the token supply of the node we're connecting to.
    pub async fn get_token_supply(&self) -> Result<u64> {
        Ok(self.get_network_info().await?.protocol_parameters.token_supply())
    }

    /// returns the tips interval
    pub async fn get_tips_interval(&self) -> u64 {
        self.network_info.read().await.tips_interval
    }

    /// returns if local pow should be used or not
    pub async fn get_local_pow(&self) -> bool {
        self.network_info.read().await.local_pow
    }

    pub(crate) async fn get_timeout(&self) -> Duration {
        *self.api_timeout.read().await
    }

    pub(crate) async fn get_remote_pow_timeout(&self) -> Duration {
        *self.remote_pow_timeout.read().await
    }

    /// returns the fallback_to_local_pow
    pub async fn get_fallback_to_local_pow(&self) -> bool {
        self.network_info.read().await.fallback_to_local_pow
    }

    /// Validates if a bech32 HRP matches the one from the connected network.
    pub async fn bech32_hrp_matches(&self, bech32_hrp: &Hrp) -> Result<()> {
        let expected = self.get_bech32_hrp().await?;
        if bech32_hrp != &expected {
            return Err(Error::Bech32HrpMismatch {
                provided: bech32_hrp.to_string(),
                expected: expected.to_string(),
            });
        };
        Ok(())
    }
}
