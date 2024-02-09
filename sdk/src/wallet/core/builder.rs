// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "storage")]
use std::collections::HashSet;
use std::sync::Arc;

use serde::Serialize;
use tokio::sync::{Mutex, RwLock};

#[cfg(feature = "events")]
use crate::wallet::events::EventEmitter;
#[cfg(all(feature = "storage", not(feature = "rocksdb")))]
use crate::wallet::storage::adapter::memory::Memory;
#[cfg(feature = "storage")]
use crate::wallet::storage::{StorageManager, StorageOptions};
use crate::{
    client::secret::{SecretManage, SecretManagerConfig},
    types::block::address::{Bech32Address, Ed25519Address, ToBech32Ext},
    wallet::{
        core::{operations::background_syncing::BackgroundSyncStatus, SecretData, WalletData, WalletInner},
        operations::syncing::SyncOptions,
        ClientOptions, Wallet,
    },
};

/// Builder for the wallet.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WalletBuilder<T = ()> {
    #[serde(flatten)]
    pub(crate) secret_data: T,
    pub(crate) address: Option<Bech32Address>,
    pub(crate) alias: Option<String>,
    pub(crate) client_options: Option<ClientOptions>,
    #[cfg(feature = "storage")]
    pub(crate) storage_options: Option<StorageOptions>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretDataBuilder<S: SecretManage> {
    pub(crate) public_key_options: Option<S::GenerationOptions>,
    pub(crate) signing_options: Option<S::SigningOptions>,
    #[serde(skip)]
    pub(crate) secret_manager: Option<Arc<RwLock<S>>>,
}

impl<S: SecretManage> Default for SecretDataBuilder<S> {
    fn default() -> Self {
        Self {
            public_key_options: Default::default(),
            signing_options: Default::default(),
            secret_manager: Default::default(),
        }
    }
}

impl<T: Default> WalletBuilder<T> {
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
    pub fn with_storage_path(mut self, path: impl Into<std::path::PathBuf>) -> Self {
        self.storage_options = Some(StorageOptions {
            path: path.into(),
            ..Default::default()
        });
        self
    }
}

impl WalletBuilder {
    /// Initialises a new instance of the wallet builder with the default storage adapter.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_secret_type<S: SecretManage>(self) -> WalletBuilder<SecretDataBuilder<S>> {
        WalletBuilder {
            secret_data: Default::default(),
            address: self.address,
            alias: self.alias,
            client_options: self.client_options,
            #[cfg(feature = "storage")]
            storage_options: self.storage_options,
        }
    }

    /// Set the public key options.
    pub fn with_public_key_options<S: SecretManage>(
        self,
        public_key_options: impl Into<Option<S::GenerationOptions>>,
    ) -> WalletBuilder<SecretDataBuilder<S>> {
        self.with_secret_type::<S>().with_public_key_options(public_key_options)
    }

    /// Set the signing options.
    pub fn with_signing_options<S: SecretManage>(
        self,
        signing_options: impl Into<Option<S::SigningOptions>>,
    ) -> WalletBuilder<SecretDataBuilder<S>> {
        self.with_secret_type::<S>().with_signing_options(signing_options)
    }

    /// Set the secret_manager to be used.
    pub fn with_secret_manager<S: SecretManage>(
        self,
        secret_manager: impl Into<Option<S>>,
    ) -> WalletBuilder<SecretDataBuilder<S>> {
        self.with_secret_type::<S>().with_secret_manager(secret_manager)
    }

    /// Set the secret_manager to be used wrapped in an Arc<RwLock<>> so it can be cloned and mutated also outside of
    /// the Wallet.
    pub fn with_secret_manager_arc<S: SecretManage>(
        self,
        secret_manager: impl Into<Option<Arc<RwLock<S>>>>,
    ) -> WalletBuilder<SecretDataBuilder<S>> {
        self.with_secret_type::<S>().with_secret_manager_arc(secret_manager)
    }
}

impl<S: SecretManage> WalletBuilder<SecretDataBuilder<S>> {
    /// Set the public key options.
    pub fn with_public_key_options(mut self, public_key_options: impl Into<Option<S::GenerationOptions>>) -> Self {
        self.secret_data.public_key_options = public_key_options.into();
        self
    }

    /// Set the signing options.
    pub fn with_signing_options(mut self, signing_options: impl Into<Option<S::SigningOptions>>) -> Self {
        self.secret_data.signing_options = signing_options.into();
        self
    }

    /// Set the secret_manager to be used.
    pub fn with_secret_manager(mut self, secret_manager: impl Into<Option<S>>) -> Self {
        self.secret_data.secret_manager = secret_manager.into().map(|sm| Arc::new(RwLock::new(sm)));
        self
    }

    /// Set the secret_manager to be used wrapped in an Arc<RwLock<>> so it can be cloned and mutated also outside of
    /// the Wallet.
    pub fn with_secret_manager_arc(mut self, secret_manager: impl Into<Option<Arc<RwLock<S>>>>) -> Self {
        self.secret_data.secret_manager = secret_manager.into();
        self
    }
}

impl WalletBuilder {
    /// Builds the wallet.
    pub async fn finish(mut self) -> crate::wallet::Result<Wallet> {
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

        #[cfg(feature = "storage")]
        let storage_manager = StorageManager::new(storage, storage_options.encryption_key.clone()).await?;

        #[cfg(feature = "storage")]
        let loaded_wallet_builder = Self::load::<()>(&storage_manager).await?;
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

        let address = self
            .address
            .as_ref()
            .ok_or(crate::wallet::Error::MissingParameter("address"))?
            .clone();

        #[cfg(feature = "storage")]
        let mut wallet_data = storage_manager.load_wallet_data().await?;

        // Store the wallet builder (for convenience reasons)
        #[cfg(feature = "storage")]
        self.save(&storage_manager).await?;

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

        let background_syncing_status = tokio::sync::watch::channel(BackgroundSyncStatus::Stopped);
        let background_syncing_status = (Arc::new(background_syncing_status.0), background_syncing_status.1);

        // Build the wallet.
        let wallet_inner = WalletInner {
            default_sync_options: Mutex::new(SyncOptions::default()),
            last_synced: Mutex::new(0),
            background_syncing_status,
            client,
            #[cfg(feature = "events")]
            event_emitter: tokio::sync::RwLock::new(EventEmitter::new()),
            #[cfg(feature = "storage")]
            storage_options,
            #[cfg(feature = "storage")]
            storage_manager,
        };
        #[cfg(feature = "storage")]
        let wallet_data = match wallet_data {
            Some(d) => d,
            None => WalletData::new(address, self.alias.clone()),
        };
        #[cfg(not(feature = "storage"))]
        let wallet_data = WalletData::new(address, self.alias.clone());
        let wallet = Wallet {
            inner: Arc::new(wallet_inner),
            data: Arc::new(RwLock::new(wallet_data)),
            secret_data: (),
        };

        // If the wallet builder is not set, it means the user provided it and we need to update the addresses.
        // In the other case it was loaded from the database and addresses are up to date.
        if provided_client_options {
            wallet.update_bech32_hrp().await?;
        }

        Ok(wallet)
    }
}

impl<S: 'static + SecretManagerConfig + core::fmt::Debug> WalletBuilder<SecretDataBuilder<S>>
where
    for<'a> &'a S::GenerationOptions: PartialEq,
{
    /// Builds the wallet.
    pub async fn finish(mut self) -> crate::wallet::Result<Wallet<SecretData<S>>> {
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

        #[cfg(feature = "storage")]
        let storage_manager = StorageManager::new(storage, storage_options.encryption_key.clone()).await?;

        #[cfg(feature = "storage")]
        let loaded_wallet_builder = Self::load::<SecretDataBuilder<S>>(&storage_manager).await?;
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

        // May use previously stored options if they weren't provided
        if self.secret_data.public_key_options.is_none() {
            self.secret_data.public_key_options = loaded_wallet_builder
                .as_ref()
                .and_then(|builder| builder.secret_data.public_key_options.clone());
        }

        // The public key options must not change.
        #[cfg(feature = "storage")]
        if let Some(secret_data) = loaded_wallet_builder.as_ref().map(|b| &b.secret_data) {
            if self.secret_data.public_key_options.is_some() {
                if self.secret_data.public_key_options != secret_data.public_key_options {
                    return Err(crate::wallet::Error::PublicKeyOptionsMismatch {
                        new: serde_json::to_value(&self.secret_data.public_key_options)?,
                        old: serde_json::to_value(&secret_data.public_key_options)?,
                    });
                }
            } else {
                self.secret_data.public_key_options = secret_data.public_key_options.clone();
            }
        }

        // May use a previously stored secret manager if it wasn't provided
        if self.secret_data.secret_manager.is_none() {
            let secret_manager = loaded_wallet_builder
                .as_ref()
                .and_then(|builder| builder.secret_data.secret_manager.clone());

            self.secret_data.secret_manager = secret_manager;
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
            if self.secret_data.secret_manager.is_some() {
                let address = self.create_default_wallet_address().await?;
                self.address = Some(address);
            } else {
                return Err(crate::wallet::Error::MissingParameter("address"));
            }
        }
        // Panic: can be safely unwrapped now
        let address = self.address.as_ref().unwrap().clone();

        #[cfg(feature = "storage")]
        let mut wallet_data = storage_manager.load_wallet_data().await?;

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

        let background_syncing_status = tokio::sync::watch::channel(BackgroundSyncStatus::Stopped);
        let background_syncing_status = (Arc::new(background_syncing_status.0), background_syncing_status.1);

        // Build the wallet.
        let wallet_inner = WalletInner {
            default_sync_options: Mutex::new(SyncOptions::default()),
            last_synced: Mutex::new(0),
            background_syncing_status,
            client,
            #[cfg(feature = "events")]
            event_emitter,
            #[cfg(feature = "storage")]
            storage_options,
            #[cfg(feature = "storage")]
            storage_manager,
        };
        #[cfg(feature = "storage")]
        let wallet_data = match wallet_data {
            Some(d) => d,
            None => WalletData::new(address, self.alias.clone()),
        };
        #[cfg(not(feature = "storage"))]
        let wallet_data = WalletData::new(address, self.alias.clone());
        let wallet = Wallet {
            inner: Arc::new(wallet_inner),
            data: Arc::new(RwLock::new(wallet_data)),
            secret_data: SecretData {
                public_key_options: Arc::new(RwLock::new(
                    self.secret_data
                        .public_key_options
                        .ok_or(crate::wallet::Error::MissingParameter("public_key_options"))?,
                )),
                signing_options: self
                    .secret_data
                    .signing_options
                    .ok_or(crate::wallet::Error::MissingParameter("signing_options"))?,
                secret_manager: self
                    .secret_data
                    .secret_manager
                    .ok_or(crate::wallet::Error::MissingParameter("secret_manager"))?,
            },
        };

        // If the wallet builder is not set, it means the user provided it and we need to update the addresses.
        // In the other case it was loaded from the database and addresses are up to date.
        if provided_client_options {
            wallet.update_bech32_hrp().await?;
        }

        Ok(wallet)
    }

    /// Generate the wallet address.
    pub(crate) async fn create_default_wallet_address(&self) -> crate::wallet::Result<Bech32Address> {
        let bech32_hrp = self
            .client_options
            .as_ref()
            .unwrap()
            .network_info
            .protocol_parameters
            .bech32_hrp;
        let options = self.secret_data.public_key_options.as_ref().unwrap();

        Ok(Ed25519Address::from_public_key_bytes(
            self.secret_data
                .secret_manager
                .as_ref()
                .unwrap()
                .read()
                .await
                .generate(options)
                .await?
                .to_bytes(),
        )
        .to_bech32(bech32_hrp))
    }
}

impl<T> WalletBuilder<T> {
    #[cfg(feature = "storage")]
    pub(crate) async fn from_wallet<U: Sync>(wallet: &Wallet<U>) -> Self
    where
        Wallet<U>: BuilderFrom<Builder = Self>,
    {
        BuilderFrom::from(wallet).await
    }
}

#[async_trait::async_trait]
pub trait BuilderFrom {
    type Builder;

    async fn from(&self) -> Self::Builder;
}

mod builder_from {
    use async_trait::async_trait;

    use super::BuilderFrom;
    use crate::{
        client::secret::SecretManage,
        wallet::{
            core::{builder::SecretDataBuilder, SecretData},
            Wallet, WalletBuilder,
        },
    };

    #[async_trait]
    impl<S: SecretManage> BuilderFrom for SecretData<S> {
        type Builder = SecretDataBuilder<S>;

        async fn from(&self) -> Self::Builder {
            Self::Builder {
                public_key_options: Some(self.public_key_options.read().await.clone()),
                signing_options: Some(self.signing_options.clone()),
                secret_manager: Some(self.secret_manager.clone()),
            }
        }
    }

    #[async_trait]
    impl BuilderFrom for () {
        type Builder = ();

        async fn from(&self) -> Self::Builder {
            ()
        }
    }

    #[async_trait]
    impl<T: BuilderFrom + Sync> BuilderFrom for Wallet<T> {
        type Builder = WalletBuilder<T::Builder>;

        async fn from(&self) -> Self::Builder {
            Self::Builder {
                address: Some(self.address().await),
                alias: self.alias().await,
                client_options: Some(self.client_options().await),
                #[cfg(feature = "storage")]
                storage_options: Some(self.storage_options.clone()),
                secret_data: BuilderFrom::from(&self.secret_data).await,
            }
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
    use crate::{client::secret::SecretManage, wallet::storage::StorageOptions};

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WalletBuilderDto<T> {
        #[serde(flatten)]
        secret_data: T,
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

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SecretDataDto<G, S> {
        #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
        pub(crate) public_key_options: Option<G>,
        #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
        pub(crate) signing_options: Option<S>,
    }

    impl<S: SecretManage> From<SecretDataDto<S::GenerationOptions, S::SigningOptions>> for SecretDataBuilder<S> {
        fn from(value: SecretDataDto<S::GenerationOptions, S::SigningOptions>) -> Self {
            Self {
                public_key_options: value.public_key_options,
                signing_options: value.signing_options,
                secret_manager: None,
            }
        }
    }

    impl<T1: Into<T2>, T2> From<WalletBuilderDto<T1>> for WalletBuilder<T2> {
        fn from(value: WalletBuilderDto<T1>) -> Self {
            Self {
                secret_data: value.secret_data.into(),
                address: value.address,
                alias: value.alias,
                client_options: value.client_options,
                #[cfg(feature = "storage")]
                storage_options: value.storage_options,
            }
        }
    }

    impl<'de, S: SecretManage> Deserialize<'de> for SecretDataBuilder<S>
    where
        S::GenerationOptions: Deserialize<'de>,
        S::SigningOptions: Deserialize<'de>,
    {
        fn deserialize<D>(d: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            SecretDataDto::<S::GenerationOptions, S::SigningOptions>::deserialize(d).map(Into::into)
        }
    }

    impl<'de, T: Deserialize<'de>> Deserialize<'de> for WalletBuilder<T> {
        fn deserialize<D>(d: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            WalletBuilderDto::<T>::deserialize(d).map(Into::into)
        }
    }
}
