// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The Client module to connect through HORNET or Bee with API usages

use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

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
        constants::DEFAULT_TIPS_INTERVAL,
        error::Result,
        node_manager::NodeManager,
        Error,
    },
    types::block::{output::RentStructure, protocol::ProtocolParameters},
};

/// An instance of the client using HORNET or Bee URI
#[derive(Clone)]
pub struct Client {
    pub(crate) inner: Arc<ClientInner>,
    #[cfg(not(target_family = "wasm"))]
    pub(crate) _sync_handle: Arc<SyncHandle>,
}

pub(crate) struct ClientInner {
    /// Node manager
    pub(crate) node_manager: NodeManager,
    pub(crate) network_info: RwLock<NetworkInfo>,
    /// HTTP request timeout.
    pub(crate) api_timeout: Duration,
    /// HTTP request timeout for remote PoW API call.
    pub(crate) remote_pow_timeout: Duration,
    /// pow_worker_count for local PoW.
    #[cfg(not(target_family = "wasm"))]
    pub(crate) pow_worker_count: Option<usize>,
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
    pub(crate) client: tokio::sync::RwLock<Option<MqttClient>>,
    pub(crate) topic_handlers: tokio::sync::RwLock<TopicHandlerMap>,
    pub(crate) broker_options: BrokerOptions,
    pub(crate) sender: WatchSender<MqttEvent>,
    pub(crate) receiver: WatchReceiver<MqttEvent>,
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

impl Drop for ClientInner {
    /// Gracefully shutdown the `Client`
    fn drop(&mut self) {
        #[cfg(feature = "mqtt")]
        {
            if let Some(mqtt_client) = self.mqtt.client.blocking_write().take() {
                mqtt_client.try_disconnect().unwrap();
            }
        }
    }
}

impl Client {
    /// Create the builder to instantiate the IOTA Client.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

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
                    return Ok(self
                        .inner
                        .network_info
                        .read()
                        .map_err(|_| crate::client::Error::PoisonError)?
                        .clone());
                }
            }
            let info = self.get_info().await?.node_info;
            let mut client_network_info = self
                .inner
                .network_info
                .write()
                .map_err(|_| crate::client::Error::PoisonError)?;
            client_network_info.protocol_parameters = info.protocol.try_into()?;

            *LAST_SYNC.lock().unwrap() = Some(current_time + CACHE_NETWORK_INFO_TIMEOUT_IN_SECONDS);
        }

        Ok(self
            .inner
            .network_info
            .read()
            .map_err(|_| crate::client::Error::PoisonError)?
            .clone())
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
    pub async fn get_bech32_hrp(&self) -> Result<String> {
        Ok(self.get_network_info().await?.protocol_parameters.bech32_hrp().into())
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
    pub fn get_tips_interval(&self) -> u64 {
        self.inner
            .network_info
            .read()
            .map_or(DEFAULT_TIPS_INTERVAL, |info| info.tips_interval)
    }

    /// returns if local pow should be used or not
    pub fn get_local_pow(&self) -> bool {
        self.inner
            .network_info
            .read()
            .map_or(NetworkInfo::default().local_pow, |info| info.local_pow)
    }

    pub(crate) fn get_timeout(&self) -> Duration {
        self.inner.api_timeout
    }

    pub(crate) fn get_remote_pow_timeout(&self) -> Duration {
        self.inner.remote_pow_timeout
    }

    /// returns the fallback_to_local_pow
    pub fn get_fallback_to_local_pow(&self) -> bool {
        self.inner
            .network_info
            .read()
            .map_or(NetworkInfo::default().fallback_to_local_pow, |info| {
                info.fallback_to_local_pow
            })
    }

    /// Validates if a bech32 HRP matches the one from the connected network.
    pub async fn bech32_hrp_matches(&self, bech32_hrp: &str) -> Result<()> {
        let expected = self.get_bech32_hrp().await?;
        if bech32_hrp != expected {
            return Err(Error::InvalidBech32Hrp {
                provided: bech32_hrp.to_string(),
                expected,
            });
        };
        Ok(())
    }
}
