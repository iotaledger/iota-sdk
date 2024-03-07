// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "storage")]
use std::collections::HashSet;
use std::sync::Arc;

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
    client::{
        secret::{GenerateAddressOptions, SecretManage, SecretManager},
        ClientError,
    },
    types::block::address::{Bech32Address, Ed25519Address},
    wallet::{
        core::{operations::background_syncing::BackgroundSyncStatus, Bip44, WalletInner, WalletLedger},
        ClientOptions, Wallet, WalletError,
    },
};

/// Builder for the wallet.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBuilder<S: SecretManage = SecretManager> {
    pub(crate) address: Option<Bech32Address>,
    pub(crate) bip_path: Option<Bip44>,
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
            address: Default::default(),
            bip_path: Default::default(),
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
    WalletError: From<S::Error>,
{
    /// Initialises a new instance of the wallet builder with the default storage adapter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the address of the wallet.
    pub fn with_address(mut self, address: impl Into<Option<Bech32Address>>) -> Self {
        self.address = address.into();
        self
    }

    /// Set the BIP44 path of the wallet.
    pub fn with_bip_path(mut self, bip_path: impl Into<Option<Bip44>>) -> Self {
        self.bip_path = bip_path.into();
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
    pub fn with_storage_path(mut self, path: impl Into<std::path::PathBuf>) -> Self {
        self.storage_options = Some(StorageOptions {
            path: path.into(),
            ..Default::default()
        });
        self
    }
}

impl<S: 'static + SecretManage> WalletBuilder<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
    Self: SaveLoadWallet,
{
    /// Builds the wallet.
    pub async fn finish(mut self) -> Result<Wallet<S>, WalletError> {
        log::debug!("[WalletBuilder]");

        #[cfg(feature = "storage")]
        let storage_options = self.storage_options.clone().unwrap_or_default();
        // Check if the db exists and if not, return an error if one parameter is missing, because otherwise the db
        // would be created with an empty parameter which just leads to errors later
        #[cfg(feature = "storage")]
        if !storage_options.path.is_dir() && self.client_options.is_none() {
            return Err(WalletError::MissingParameter("client_options"));
        }

        #[cfg(all(feature = "rocksdb", feature = "storage"))]
        let storage =
            crate::wallet::storage::adapter::rocksdb::RocksdbStorageAdapter::new(storage_options.path.clone())?;
        #[cfg(all(not(feature = "rocksdb"), feature = "storage"))]
        let storage = Memory::default();

        #[cfg(feature = "storage")]
        let storage_manager = StorageManager::new(storage, storage_options.encryption_key.clone()).await?;

        #[cfg(feature = "storage")]
        let loaded_wallet_builder = Self::load(&storage_manager).await?;
        #[cfg(not(feature = "storage"))]
        let loaded_wallet_builder: Option<Self> = None;

        // May use a previously stored client options if those weren't provided
        let provided_client_options = if self.client_options.is_none() {
            let loaded_client_options = loaded_wallet_builder
                .as_ref()
                .and_then(|data| data.client_options.clone())
                .ok_or(WalletError::MissingParameter("client_options"))?;

            // Update self so it gets used and stored again
            self.client_options = Some(loaded_client_options);
            false
        } else {
            true
        };

        // May use a previously stored secret manager if it wasn't provided
        if self.secret_manager.is_none() {
            self.secret_manager.replace(
                loaded_wallet_builder
                    .as_ref()
                    .and_then(|builder| builder.secret_manager.clone())
                    .ok_or(WalletError::MissingParameter("secret_manager"))?,
            );
        }

        let mut verify_address = false;
        let loaded_address = loaded_wallet_builder
            .as_ref()
            .and_then(|builder| builder.address.clone());

        // May use a previously stored address if it wasn't provided
        if let Some(address) = &self.address {
            if let Some(loaded_address) = &loaded_address {
                if address != loaded_address {
                    return Err(WalletError::WalletAddressMismatch(address.clone()));
                }
            } else {
                verify_address = true;
            }
        } else {
            self.address = loaded_address;
        }

        let loaded_bip_path = loaded_wallet_builder.as_ref().and_then(|builder| builder.bip_path);

        // May use a previously stored BIP path if it wasn't provided
        if let Some(bip_path) = self.bip_path {
            if let Some(loaded_bip_path) = loaded_bip_path {
                if bip_path != loaded_bip_path {
                    return Err(WalletError::BipPathMismatch {
                        new_bip_path: Some(bip_path),
                        old_bip_path: Some(loaded_bip_path),
                    });
                }
            } else {
                verify_address = true;
            }
        } else {
            self.bip_path = loaded_bip_path;
        }

        // Create the node client.
        let client = self
            .client_options
            .clone()
            .ok_or(WalletError::MissingParameter("client_options"))?
            .finish()
            .await?;

        match (self.address.as_ref(), self.bip_path.as_ref()) {
            (Some(address), Some(bip_path)) => {
                if verify_address {
                    // verify that the address is derived from the provided bip path.
                    if let Some(backing_ed25519_address) = address.inner.backing_ed25519() {
                        self.verify_ed25519_address(backing_ed25519_address, bip_path).await?;
                    } else {
                        return Err(WalletError::InvalidParameter("address/bip_path mismatch"));
                    }
                }
            }
            (Some(_address), None) => {}
            (None, Some(bip_path)) => {
                self.address.replace(Bech32Address::new(
                    client.get_bech32_hrp().await?,
                    self.generate_ed25519_address(bip_path).await?,
                ));
            }
            (None, None) => {
                return Err(WalletError::MissingParameter("address or bip_path"));
            }
        };

        // May use a previously stored wallet alias if it wasn't provided
        if self.alias.is_none() {
            self.alias = loaded_wallet_builder.as_ref().and_then(|builder| builder.alias.clone());
        }

        #[cfg(feature = "storage")]
        let mut wallet_ledger = storage_manager.load_wallet_ledger().await?;

        // Store the wallet builder (for convenience reasons)
        #[cfg(feature = "storage")]
        self.save(&storage_manager).await?;

        // It happened that inputs got locked, the transaction failed, but they weren't unlocked again, so we do this
        // here
        #[cfg(feature = "storage")]
        if let Some(wallet_ledger) = &mut wallet_ledger {
            unlock_unused_inputs(wallet_ledger)?;
        }

        #[cfg(feature = "storage")]
        let default_sync_options = storage_manager.get_default_sync_options().await?.unwrap_or_default();
        #[cfg(not(feature = "storage"))]
        let default_sync_options = crate::wallet::SyncOptions::default();

        let background_syncing_status = tokio::sync::watch::channel(BackgroundSyncStatus::Stopped);
        let background_syncing_status = (Arc::new(background_syncing_status.0), background_syncing_status.1);

        // Build the wallet.
        let wallet_inner = WalletInner {
            default_sync_options: Mutex::new(default_sync_options),
            last_synced: Mutex::new(0),
            background_syncing_status,
            client,
            // TODO: make secret manager optional
            secret_manager: self.secret_manager.expect("make WalletInner::secret_manager optional?"),
            #[cfg(feature = "events")]
            event_emitter: tokio::sync::RwLock::new(EventEmitter::new()),
            #[cfg(feature = "storage")]
            storage_options,
            #[cfg(feature = "storage")]
            storage_manager,
        };
        #[cfg(feature = "storage")]
        let wallet_ledger = wallet_ledger.unwrap_or_default();
        #[cfg(not(feature = "storage"))]
        let wallet_ledger = WalletLedger::default();

        let wallet = Wallet {
            // Unwrap: The address is always set above (or we already returned)
            address: Arc::new(RwLock::new(self.address.unwrap())),
            bip_path: Arc::new(RwLock::new(self.bip_path)),
            alias: Arc::new(RwLock::new(self.alias)),
            inner: Arc::new(wallet_inner),
            ledger: Arc::new(RwLock::new(wallet_ledger)),
        };

        // If the wallet builder is not set, it means the user provided it and we need to update the addresses.
        // In the other case it was loaded from the database and addresses are up to date.
        if provided_client_options {
            wallet.update_address_hrp().await?;
        }

        Ok(wallet)
    }

    #[cfg(feature = "storage")]
    pub(crate) async fn from_wallet(wallet: &Wallet<S>) -> Self {
        Self {
            address: Some(wallet.address().await),
            bip_path: wallet.bip_path().await,
            alias: wallet.alias().await,
            client_options: Some(wallet.client_options().await),
            storage_options: Some(wallet.storage_options.clone()),
            secret_manager: Some(wallet.secret_manager.clone()),
        }
    }

    #[inline(always)]
    async fn verify_ed25519_address(
        &self,
        ed25519_address: &Ed25519Address,
        bip_path: &Bip44,
    ) -> Result<(), WalletError> {
        (ed25519_address == &self.generate_ed25519_address(bip_path).await?)
            .then_some(())
            .ok_or(WalletError::InvalidParameter("address/bip_path mismatch"))
    }

    async fn generate_ed25519_address(&self, bip_path: &Bip44) -> Result<Ed25519Address, WalletError> {
        if let Some(secret_manager) = &self.secret_manager {
            let secret_manager = &*secret_manager.read().await;
            Ok(secret_manager
                .generate_ed25519_addresses(
                    bip_path.coin_type,
                    bip_path.account,
                    bip_path.address_index..bip_path.address_index + 1,
                    GenerateAddressOptions {
                        internal: bip_path.change != 0,
                        ledger_nano_prompt: false,
                    },
                )
                // Panic: if it didn't return an Err, then there must be at least one address
                .await?[0])
        } else {
            Err(WalletError::MissingParameter("secret_manager"))
        }
    }
}

// Check if any of the locked inputs is not used in a transaction and unlock them, so they get available for new
// transactions
#[cfg(feature = "storage")]
fn unlock_unused_inputs(wallet_ledger: &mut WalletLedger) -> Result<(), WalletError> {
    log::debug!("[unlock_unused_inputs]");
    let mut used_inputs = HashSet::new();
    for transaction_id in &wallet_ledger.pending_transactions {
        if let Some(tx) = wallet_ledger.transactions.get(transaction_id) {
            for input in &tx.inputs {
                used_inputs.insert(*input.metadata.output_id());
            }
        }
    }
    wallet_ledger.locked_outputs.retain(|input| {
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) address: Option<Bech32Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) bip_path: Option<Bip44>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) alias: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) client_options: Option<ClientOptions>,
        #[cfg(feature = "storage")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) storage_options: Option<StorageOptions>,
    }

    impl<S: SecretManage> From<WalletBuilderDto> for WalletBuilder<S> {
        fn from(value: WalletBuilderDto) -> Self {
            Self {
                address: value.address,
                bip_path: value.bip_path,
                alias: value.alias,
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
