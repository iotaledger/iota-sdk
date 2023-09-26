// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::{
    atomic::{AtomicU32, AtomicUsize},
    Arc,
};
#[cfg(feature = "storage")]
use std::{collections::HashSet, sync::atomic::Ordering};

use crypto::keys::bip44::Bip44;
use futures::{future::try_join_all, FutureExt};
use serde::Serialize;
use tokio::sync::{Mutex, RwLock};

use super::operations::storage::SaveLoadWallet;
#[cfg(feature = "events")]
use crate::wallet::events::EventEmitter;
#[cfg(all(feature = "storage", not(feature = "rocksdb")))]
use crate::wallet::storage::adapter::memory::Memory;
#[cfg(feature = "storage")]
use crate::wallet::storage::{StorageManager, StorageOptions};
use crate::{
    client::secret::{GenerateAddressOptions, SecretManage, SecretManager},
    types::block::{
        address::{AccountAddress, Address, Bech32Address, Hrp, ToBech32Ext},
        output::AccountId,
    },
    wallet::{
        account::SyncOptions,
        core::{WalletData, WalletInner},
        ClientOptions, Wallet,
    },
};

/// Builder for the wallet inner.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBuilder<S: SecretManage = SecretManager> {
    pub(crate) bip_path: Option<Bip44>,
    pub(crate) address: Option<Bech32Address>,
    pub(crate) alias: Option<String>,
    pub(crate) client_options: Option<ClientOptions>,
    #[cfg(feature = "storage")]
    pub(crate) storage_options: Option<StorageOptions>,
    #[serde(skip)]
    pub(crate) secret_manager: Option<Arc<RwLock<S>>>,
}

impl<S: SecretManage> Default for WalletBuilder<S> {
    fn default() -> Self {
        Self {
            bip_path: Default::default(),
            address: Default::default(),
            alias: Default::default(),
            client_options: Default::default(),
            #[cfg(feature = "storage")]
            storage_options: Default::default(),
            secret_manager: Default::default(),
        }
    }
}

impl<S: 'static + SecretManage> WalletBuilder<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Initialises a new instance of the wallet builder with the default storage adapter.
    pub fn new() -> Self {
        Self {
            secret_manager: None,
            ..Default::default()
        }
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
    pub fn with_alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
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

    /// Set the secret_manager to be used.
    pub fn with_secret_manager(mut self, secret_manager: impl Into<Option<S>>) -> Self {
        self.secret_manager = secret_manager.into().map(|sm| Arc::new(RwLock::new(sm)));
        self
    }

    /// Set the secret_manager to be used wrapped in an Arc<RwLock<>> so it can be cloned and mutated also outside of
    /// the Wallet.
    pub fn with_secret_manager_arc(mut self, secret_manager: impl Into<Option<Arc<RwLock<S>>>>) -> Self {
        self.secret_manager = secret_manager.into();
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

impl<S: 'static + SecretManage> WalletBuilder<S>
where
    crate::wallet::Error: From<S::Error>,
    Self: SaveLoadWallet,
{
    /// Builds the wallet.
    pub async fn finish(mut self) -> crate::wallet::Result<Wallet<S>> {
        log::debug!("[WalletBuilder]");

        #[cfg(feature = "storage")]
        let storage_options = self.storage_options.clone().unwrap_or_default();
        // Check if the db exists and if not, return an error if one parameter is missing, because otherwise the db
        // would be created with an empty parameter which just leads to errors later
        #[cfg(feature = "storage")]
        if !storage_options.path.is_dir() {
            if self.bip_path.is_none() {
                return Err(crate::wallet::Error::MissingParameter("bip_path"));
            }
            if self.alias.is_none() {
                return Err(crate::wallet::Error::MissingParameter("alias"));
            }
            if self.client_options.is_none() {
                return Err(crate::wallet::Error::MissingParameter("client_options"));
            }

            // // TODO: consider not requiring a secret manager
            // if self.secret_manager.is_none() {
            //     return Err(crate::wallet::Error::MissingParameter("secret_manager"));
            // }
        }

        #[cfg(all(feature = "rocksdb", feature = "storage"))]
        let storage =
            crate::wallet::storage::adapter::rocksdb::RocksdbStorageAdapter::new(storage_options.path.clone())?;
        #[cfg(all(not(feature = "rocksdb"), feature = "storage"))]
        let storage = Memory::default();

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

        // May use a previously stored secret manager if it wasn't provided
        if self.secret_manager.is_none() {
            let secret_manager = loaded_wallet_builder
                .as_ref()
                .and_then(|builder| builder.secret_manager.clone());

            self.secret_manager = secret_manager;
        }

        // May use a previously stored BIP path if it wasn't provided
        if self.bip_path.is_none() {
            let bip_path = loaded_wallet_builder
                .as_ref()
                .and_then(|builder| builder.bip_path)
                .ok_or(crate::wallet::Error::MissingParameter("bip_path"))?;

            self.bip_path = Some(bip_path);
        }
        // Panic: can be safely unwrapped now
        let bip_path = self.bip_path.as_ref().unwrap().clone();

        // May use a previously stored wallet alias if it wasn't provided
        if self.alias.is_none() {
            let alias = loaded_wallet_builder
                .as_ref()
                .and_then(|builder| builder.alias.clone())
                .ok_or(crate::wallet::Error::MissingParameter("alias"))?;

            self.alias = Some(alias);
        }
        // Panic: can be safely unwrapped now
        let alias = self.alias.as_ref().unwrap().clone();

        let bech32_hrp = self
            .client_options
            .as_ref()
            .unwrap()
            .network_info
            .protocol_parameters
            .bech32_hrp;
        println!("HRP from protocol parameters: {bech32_hrp}");

        // May use a previously stored wallet address if it wasn't provided
        if self.address.is_none() {
            let address = loaded_wallet_builder.as_ref().and_then(|builder| builder.address);

            self.address = address;
        }

        // May create a default Ed25519 wallet address if there's a secret manager.
        if self.address.is_none() {
            if self.secret_manager.is_some() {
                let secret_manager = &**self.secret_manager.as_ref().unwrap();
                let bip_path = *self.bip_path.as_ref().unwrap();
                let coin_type = bip_path.coin_type;
                let account_index = bip_path.account;
                let address = self.create_default_wallet_address(bech32_hrp, bip_path).await?;

                self.address = Some(address);
            } else {
                return Err(crate::wallet::Error::MissingParameter("address"));
            }
        }
        // Panic: can be safely unwrapped now
        let address = self.address.as_ref().unwrap().clone();

        #[cfg(feature = "storage")]
        let mut wallet_data = storage_manager.load_wallet_data().await?;

        // The coin type must not change.
        #[cfg(feature = "storage")]
        if let Some(wallet_data) = &wallet_data {
            let new_coin_type = bip_path.coin_type;
            let old_coin_type = wallet_data.bip_path.coin_type;
            if bip_path != wallet_data.bip_path {
                if new_coin_type != old_coin_type {
                    return Err(crate::wallet::Error::CoinTypeMismatch {
                        new_coin_type,
                        old_coin_type,
                    });
                }
            }
        }

        // TODO: check address against the BIP account index and address index

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

        // Create wallet inner and wallet data.
        let wallet_inner = WalletInner {
            default_sync_options: Mutex::new(SyncOptions::default()),
            last_synced: Mutex::new(0),
            background_syncing_status: AtomicUsize::new(0),
            client,
            secret_manager: self.secret_manager.expect("make WalletInner::secret_manager optional?"),
            #[cfg(feature = "events")]
            event_emitter,
            #[cfg(feature = "storage")]
            storage_options,
            #[cfg(feature = "storage")]
            storage_manager: tokio::sync::RwLock::new(storage_manager),
        };
        let wallet_data = WalletData::new(bip_path, address, alias);

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
    pub(crate) async fn create_default_wallet_address(
        &self,
        hrp: Hrp,
        bip_path: Bip44,
    ) -> crate::wallet::Result<Bech32Address> {
        Ok(Bech32Address::new(
            hrp,
            Address::Ed25519(
                self.secret_manager
                    .as_ref()
                    .unwrap()
                    .read()
                    .await
                    .generate_ed25519_addresses(
                        bip_path.coin_type,
                        bip_path.account,
                        0..1,
                        GenerateAddressOptions {
                            internal: false,
                            ledger_nano_prompt: false,
                        },
                    )
                    .await?[0],
            ),
        ))
    }

    #[cfg(feature = "storage")]
    pub(crate) async fn from_wallet(wallet: &Wallet<S>) -> Self {
        Self {
            bip_path: Some(wallet.data().await.bip_path.clone()),
            address: Some(wallet.data().await.address.clone()),
            alias: Some(wallet.data().await.alias.clone()),
            client_options: Some(wallet.client_options().await),
            storage_options: Some(wallet.storage_options.clone()),
            secret_manager: Some(wallet.secret_manager.clone()),
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
    pub struct WalletBuilderDto {
        pub(crate) bip_path: Bip44,
        pub(crate) address: Bech32Address,
        pub(crate) alias: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) client_options: Option<ClientOptions>,
        #[cfg(feature = "storage")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) storage_options: Option<StorageOptions>,
    }

    impl<S: SecretManage> From<WalletBuilderDto> for WalletBuilder<S> {
        fn from(value: WalletBuilderDto) -> Self {
            Self {
                bip_path: Some(value.bip_path),
                address: Some(value.address),
                alias: Some(value.alias),
                client_options: value.client_options,
                #[cfg(feature = "storage")]
                storage_options: value.storage_options,
                secret_manager: None,
            }
        }
    }

    impl<'de, S: SecretManage> Deserialize<'de> for WalletBuilder<S> {
        fn deserialize<D>(d: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            WalletBuilderDto::deserialize(d).map(Into::into)
        }
    }
}
