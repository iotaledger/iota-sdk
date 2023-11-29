// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "storage")]
use std::collections::HashSet;
use std::sync::{atomic::AtomicUsize, Arc};

use serde::Serialize;
use tokio::sync::{Mutex, RwLock};

#[cfg(feature = "events")]
use crate::wallet::events::EventEmitter;
#[cfg(all(feature = "storage", not(feature = "rocksdb")))]
use crate::wallet::storage::adapter::memory::Memory;
#[cfg(feature = "storage")]
use crate::wallet::storage::{StorageManager, StorageOptions};
use crate::{
    client::secret::{GenerateAddressOptions, SecretManage},
    types::block::address::{Address, Bech32Address},
    wallet::{
        core::{Bip44, WalletData, WalletInner},
        operations::syncing::SyncOptions,
        ClientOptions, Wallet,
    },
};

/// Builder for the wallet.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBuilder {
    pub(crate) bip_path: Option<Bip44>,
    pub(crate) address: Option<Bech32Address>,
    pub(crate) alias: Option<String>,
    pub(crate) client_options: Option<ClientOptions>,
    #[cfg(feature = "storage")]
    pub(crate) storage_options: Option<StorageOptions>,
}

impl Default for WalletBuilder {
    fn default() -> Self {
        Self {
            bip_path: Default::default(),
            address: Default::default(),
            alias: Default::default(),
            client_options: Default::default(),
            #[cfg(feature = "storage")]
            storage_options: Default::default(),
        }
    }
}

impl WalletBuilder {
    /// Initialises a new instance of the wallet builder with the default storage adapter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the BIP44 path of the wallet.
    pub fn with_bip_path(mut self, bip_path: impl Into<Option<Bip44>>) -> Self {
        self.bip_path = bip_path.into();
        self
    }

    /// Set the wallet address.
    pub fn with_address(mut self, address: impl Into<Option<Bech32Address>>) -> Self {
        self.address = address.into();
        self
    }

    /// Set the alias of the wallet.
    pub fn with_alias(mut self, alias: impl Into<Option<String>>) -> Self {
        self.alias = alias.into();
        self
    }

    /// Set the client options for the core nodes.
    pub fn with_client_options(mut self, client_options: impl Into<Option<ClientOptions>>) -> Self {
        self.client_options = client_options.into();
        self
    }

    /// Set the storage options to be used.
    #[cfg(feature = "storage")]
    #[cfg_attr(docsrs, doc(cfg(feature = "storage")))]
    pub fn with_storage_options(mut self, storage_options: impl Into<Option<StorageOptions>>) -> Self {
        self.storage_options = storage_options.into();
        self
    }

    /// Set the storage path to be used.
    #[cfg(feature = "storage")]
    #[cfg_attr(docsrs, doc(cfg(feature = "storage")))]
    pub fn with_storage_path(mut self, path: &str) -> Self {
        self.storage_options = Some(StorageOptions {
            path: path.into(),
            ..Default::default()
        });
        self
    }
}

impl WalletBuilder {
    /// Builds the wallet.
    pub async fn finish<S: SecretManage>(mut self, secret_manager: &S) -> crate::wallet::Result<Wallet>
    where
        crate::client::Error: From<S::Error>,
    {
        log::debug!("[WalletBuilder]");

        #[cfg(feature = "storage")]
        let storage_options = self.storage_options.clone().unwrap_or_default();

        // Check if the db exists and if not, return an error if one parameter is missing, because otherwise the db
        // would be created with an empty parameter which just leads to errors later
        #[cfg(feature = "storage")]
        if !storage_options.path.is_dir() && self.client_options.is_none() {
            return Err(crate::wallet::Error::MissingParameter("client_options"));
        }

        #[cfg(all(feature = "rocksdb", feature = "storage"))]
        let storage =
            crate::wallet::storage::adapter::rocksdb::RocksdbStorageAdapter::new(storage_options.path.clone())?;
        #[cfg(all(not(feature = "rocksdb"), feature = "storage"))]
        let storage = Memory::default();

        // Check if the db exists and if not, return an error if one parameter is missing, because otherwise the db
        // would be created with an empty parameter which just leads to errors later
        #[cfg(feature = "storage")]
        if !storage_options.path.is_dir() {
            if self.client_options.is_none() {
                return Err(crate::wallet::Error::MissingParameter("client_options"));
            }
        }

        #[cfg(feature = "storage")]
        let mut storage_manager = StorageManager::new(storage, storage_options.encryption_key.clone()).await?;

        #[cfg(feature = "storage")]
        let loaded_wallet_builder = Self::load(&storage_manager).await?;
        #[cfg(not(feature = "storage"))]
        let loaded_wallet_builder: Option<Self> = None;

        // May use a previously stored client options if those weren't provided
        let provided_client_options = if self.client_options.is_none() {
            let loaded_client_options = loaded_wallet_builder
                .as_ref()
                .and_then(|data| data.client_options.clone())
                .ok_or(crate::wallet::Error::MissingParameter("client_options"))?;

            // Update self so it gets used and stored again
            self.client_options = Some(loaded_client_options);
            false
        } else {
            true
        };

        // May use a previously stored BIP path if it wasn't provided
        if self.bip_path.is_none() {
            self.bip_path = loaded_wallet_builder.as_ref().and_then(|builder| builder.bip_path);
        }

        // May use a previously stored wallet alias if it wasn't provided
        if self.alias.is_none() {
            self.alias = loaded_wallet_builder.as_ref().and_then(|builder| builder.alias.clone());
        }

        // May use a previously stored wallet address if it wasn't provided
        if self.address.is_none() {
            self.address = loaded_wallet_builder
                .as_ref()
                .and_then(|builder| builder.address.clone());
        }

        // May create a default Ed25519 wallet address if there's a secret manager.
        if self.address.is_none() {
            let address = self.create_default_wallet_address(secret_manager).await?;
            self.address = Some(address);
        }
        // Panic: can be safely unwrapped now
        let address = self.address.as_ref().unwrap().clone();

        #[cfg(feature = "storage")]
        let mut wallet_data = storage_manager.load_wallet_data().await?;

        // The bip path must not change.
        #[cfg(feature = "storage")]
        if let Some(wallet_data) = &wallet_data {
            let new_bip_path = self.bip_path;
            let old_bip_path = wallet_data.bip_path;
            if new_bip_path != old_bip_path {
                return Err(crate::wallet::Error::BipPathMismatch {
                    new_bip_path,
                    old_bip_path,
                });
            }
        }

        // Store the wallet builder (for convenience reasons)
        #[cfg(feature = "storage")]
        self.save(&storage_manager).await?;

        #[cfg(feature = "events")]
        let event_emitter = tokio::sync::RwLock::new(EventEmitter::new());

        // It happened that inputs got locked, the transaction failed, but they weren't unlocked again, so we do this
        // here
        #[cfg(feature = "storage")]
        if let Some(wallet_data) = &mut wallet_data {
            unlock_unused_inputs(wallet_data)?;
        }

        // Create the node client.
        let client = self
            .client_options
            .clone()
            .ok_or(crate::wallet::Error::MissingParameter("client_options"))?
            .finish()
            .await?;

        // Build the wallet.
        let wallet_inner = WalletInner {
            default_sync_options: Mutex::new(SyncOptions::default()),
            last_synced: Mutex::new(0),
            background_syncing_status: AtomicUsize::new(0),
            client,
            #[cfg(feature = "events")]
            event_emitter,
            #[cfg(feature = "storage")]
            storage_options,
            #[cfg(feature = "storage")]
            storage_manager: tokio::sync::RwLock::new(storage_manager),
        };
        let wallet_data = WalletData::new(self.bip_path, address, self.alias.clone());
        let wallet = Wallet {
            inner: Arc::new(wallet_inner),
            data: Arc::new(RwLock::new(wallet_data)),
        };

        // If the wallet builder is not set, it means the user provided it and we need to update the addresses.
        // In the other case it was loaded from the database and addresses are up to date.
        if provided_client_options {
            wallet.update_bech32_hrp().await?;
        }

        Ok(wallet)
    }

    /// Generate the wallet address.
    pub(crate) async fn create_default_wallet_address<S: SecretManage>(
        &self,
        secret_manager: &S,
    ) -> crate::wallet::Result<Bech32Address>
    where
        crate::client::Error: From<S::Error>,
    {
        let bech32_hrp = self
            .client_options
            .as_ref()
            .unwrap()
            .network_info
            .protocol_parameters
            .bech32_hrp;
        let bip_path = self.bip_path.as_ref().unwrap();

        Ok(Bech32Address::new(
            bech32_hrp,
            Address::Ed25519(
                secret_manager
                    .generate_ed25519_addresses(
                        bip_path.coin_type,
                        bip_path.account,
                        bip_path.address_index..bip_path.address_index + 1,
                        GenerateAddressOptions {
                            internal: bip_path.change != 0,
                            ledger_nano_prompt: false,
                        },
                    )
                    .await
                    .map_err(crate::client::Error::from)?[0],
            ),
        ))
    }

    #[cfg(feature = "storage")]
    pub(crate) async fn from_wallet(wallet: &Wallet) -> Self {
        Self {
            bip_path: wallet.bip_path().await,
            address: Some(wallet.address().await),
            alias: wallet.alias().await,
            client_options: Some(wallet.client_options().await),
            storage_options: Some(wallet.storage_options.clone()),
        }
    }
}

// Check if any of the locked inputs is not used in a transaction and unlock them, so they get available for new
// transactions
#[cfg(feature = "storage")]
fn unlock_unused_inputs(wallet_data: &mut WalletData) -> crate::wallet::Result<()> {
    log::debug!("[unlock_unused_inputs]");
    let mut used_inputs = HashSet::new();
    for transaction_id in &wallet_data.pending_transactions {
        if let Some(tx) = wallet_data.transactions.get(transaction_id) {
            for input in &tx.inputs {
                used_inputs.insert(*input.metadata.output_id());
            }
        }
    }
    wallet_data.locked_outputs.retain(|input| {
        let used = used_inputs.contains(input);
        if !used {
            log::debug!("unlocking unused input {input}");
        }
        used
    });
    Ok(())
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::Deserialize;

    use super::*;
    #[cfg(feature = "storage")]
    use crate::wallet::storage::StorageOptions;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WalletBuilderDto {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) bip_path: Option<Bip44>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) address: Option<Bech32Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) alias: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) client_options: Option<ClientOptions>,
        #[cfg(feature = "storage")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) storage_options: Option<StorageOptions>,
    }

    impl From<WalletBuilderDto> for WalletBuilder {
        fn from(value: WalletBuilderDto) -> Self {
            Self {
                bip_path: value.bip_path,
                address: value.address,
                alias: value.alias,
                client_options: value.client_options,
                #[cfg(feature = "storage")]
                storage_options: value.storage_options,
            }
        }
    }

    impl<'de> Deserialize<'de> for WalletBuilder {
        fn deserialize<D>(d: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            WalletBuilderDto::deserialize(d).map(Into::into)
        }
    }
}
