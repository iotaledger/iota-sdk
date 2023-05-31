// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::{
    atomic::{AtomicU32, AtomicUsize},
    Arc,
};
#[cfg(feature = "storage")]
use std::{collections::HashSet, path::PathBuf, sync::atomic::Ordering};

use futures::{future::try_join_all, FutureExt};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[cfg(feature = "events")]
use crate::wallet::events::EventEmitter;
#[cfg(all(feature = "storage", not(feature = "rocksdb")))]
use crate::wallet::storage::adapter::memory::Memory;
#[cfg(feature = "storage")]
use crate::wallet::{
    account::AccountDetails,
    storage::{
        constants::default_storage_path,
        manager::{ManagerStorage, StorageManager},
    },
};
use crate::{
    client::secret::SecretManager,
    wallet::{wallet::WalletInner, Account, ClientOptions, Wallet},
};

#[derive(Debug, Serialize, Deserialize, Default)]
/// Builder for the wallet.
pub struct WalletBuilder {
    client_options: Option<ClientOptions>,
    coin_type: Option<u32>,
    #[cfg(feature = "storage")]
    storage_options: Option<StorageOptions>,
    #[serde(default, skip)]
    pub(crate) secret_manager: Option<Arc<RwLock<SecretManager>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg(feature = "storage")]
#[cfg_attr(docsrs, doc(cfg(feature = "storage")))]
pub(crate) struct StorageOptions {
    pub(crate) storage_path: PathBuf,
    pub(crate) storage_file_name: Option<String>,
    pub(crate) storage_encryption_key: Option<[u8; 32]>,
    pub(crate) manager_store: ManagerStorage,
}

#[cfg(feature = "storage")]
impl Default for StorageOptions {
    fn default() -> Self {
        Self {
            storage_path: default_storage_path().into(),
            storage_file_name: None,
            storage_encryption_key: None,
            manager_store: ManagerStorage::default(),
        }
    }
}

impl WalletBuilder {
    /// Initialises a new instance of the wallet builder with the default storage adapter.
    pub fn new() -> Self {
        Self {
            secret_manager: None,
            ..Default::default()
        }
    }

    /// Set the client options for the core nodes.
    pub fn with_client_options(mut self, client_options: impl Into<Option<ClientOptions>>) -> Self {
        self.client_options = client_options.into();
        self
    }

    /// Set the coin type for the wallet. Registered coin types can be found at <https://github.com/satoshilabs/slips/blob/master/slip-0044.md>.
    pub fn with_coin_type(mut self, coin_type: impl Into<Option<u32>>) -> Self {
        self.coin_type = coin_type.into();
        self
    }

    /// Set the secret_manager to be used.
    pub fn with_secret_manager(mut self, secret_manager: impl Into<Option<SecretManager>>) -> Self {
        self.secret_manager = secret_manager.into().map(|sm| Arc::new(RwLock::new(sm)));
        self
    }

    /// Set the secret_manager to be used wrapped in an Arc<RwLock<>> so it can be cloned and mutated also outside of
    /// the Wallet.
    pub fn with_secret_manager_arc(mut self, secret_manager: impl Into<Option<Arc<RwLock<SecretManager>>>>) -> Self {
        self.secret_manager = secret_manager.into();
        self
    }

    /// Set the storage path to be used.
    #[cfg(feature = "storage")]
    #[cfg_attr(docsrs, doc(cfg(feature = "storage")))]
    pub fn with_storage_path(mut self, path: &str) -> Self {
        self.storage_options = Some(StorageOptions {
            storage_path: path.into(),
            ..Default::default()
        });
        self
    }

    /// Builds the wallet
    #[allow(unreachable_code, unused_mut)]
    pub async fn finish(mut self) -> crate::wallet::Result<Wallet> {
        log::debug!("[WalletBuilder]");

        #[cfg(feature = "storage")]
        let storage_options = self.storage_options.clone().unwrap_or_default();
        #[cfg(feature = "storage")]
        // Check if the db exists and if not, return an error if one parameter is missing, because otherwise the db
        // would be created with an empty parameter which just leads to errors later
        if !storage_options.storage_path.is_dir() {
            if self.client_options.is_none() {
                return Err(crate::wallet::Error::MissingParameter("client_options"));
            }
            if self.coin_type.is_none() {
                return Err(crate::wallet::Error::MissingParameter("coin_type"));
            }
            if self.secret_manager.is_none() {
                return Err(crate::wallet::Error::MissingParameter("secret_manager"));
            }
        }
        #[cfg(all(feature = "rocksdb", feature = "storage"))]
        let storage =
            crate::wallet::storage::adapter::rocksdb::RocksdbStorageAdapter::new(storage_options.storage_path.clone())?;
        #[cfg(all(not(feature = "rocksdb"), feature = "storage"))]
        let storage = Memory::default();

        #[cfg(feature = "storage")]
        let mut storage_manager = StorageManager::new(storage, None).await?;

        #[cfg(feature = "storage")]
        let mut read_manager_builder = storage_manager.get_wallet_data().await?;
        #[cfg(not(feature = "storage"))]
        let mut read_manager_builder: Option<Self> = None;

        // Prioritize provided client_options and secret_manager over stored ones
        let new_provided_client_options = if self.client_options.is_none() {
            let loaded_client_options = read_manager_builder
                .as_ref()
                .and_then(|data| data.client_options.clone())
                .ok_or(crate::wallet::Error::MissingParameter("client_options"))?;

            // Update self so it gets used and stored again
            self.client_options.replace(loaded_client_options);
            false
        } else {
            true
        };

        if self.secret_manager.is_none() {
            let secret_manager = read_manager_builder
                .as_ref()
                .and_then(|data| data.secret_manager.clone())
                .ok_or(crate::wallet::Error::MissingParameter("secret_manager"))?;

            // Update self so it gets used and stored again
            self.secret_manager.replace(secret_manager);
        }

        if self.coin_type.is_none() {
            self.coin_type = read_manager_builder.and_then(|builder| builder.coin_type);
        }
        let coin_type = self.coin_type.ok_or(crate::wallet::Error::MissingParameter(
            "coin_type (IOTA: 4218, Shimmer: 4219)",
        ))?;

        #[cfg(feature = "storage")]
        let mut accounts = storage_manager.get_accounts().await?;
        // Check against potential account coin type before saving the wallet data
        #[cfg(feature = "storage")]
        if let Some(account) = accounts.first() {
            if *account.coin_type() != coin_type {
                return Err(crate::wallet::Error::InvalidCoinType {
                    new_coin_type: coin_type,
                    existing_coin_type: *account.coin_type(),
                });
            }
        }

        // Store wallet data in storage
        #[cfg(feature = "storage")]
        storage_manager.save_wallet_data(&self).await?;

        #[cfg(feature = "events")]
        let event_emitter = tokio::sync::RwLock::new(EventEmitter::new());

        // It happened that inputs got locked, the transaction failed, but they weren't unlocked again, so we do this
        // here
        #[cfg(feature = "storage")]
        unlock_unused_inputs(&mut accounts)?;
        #[cfg(not(feature = "storage"))]
        let accounts = Vec::new();
        let wallet_inner = Arc::new(WalletInner {
            background_syncing_status: AtomicUsize::new(0),
            client: self
                .client_options
                .clone()
                .ok_or(crate::wallet::Error::MissingParameter("client_options"))?
                .finish()
                .await?,
            coin_type: AtomicU32::new(coin_type),
            secret_manager: self
                .secret_manager
                .ok_or(crate::wallet::Error::MissingParameter("secret_manager"))?,
            #[cfg(feature = "events")]
            event_emitter,
            #[cfg(feature = "storage")]
            storage_options,
            #[cfg(feature = "storage")]
            storage_manager: tokio::sync::RwLock::new(storage_manager),
        });

        let mut accounts: Vec<Account> = try_join_all(
            accounts
                .into_iter()
                .map(|a| Account::new(a, wallet_inner.clone()).boxed()),
        )
        .await?;

        // If the wallet builder is not set, it means the user provided it and we need to update the addresses.
        // In the other case it was loaded from the database and addresses are up to date.
        if new_provided_client_options {
            for account in accounts.iter_mut() {
                // Safe to unwrap because we create the client if accounts aren't empty
                account.update_account_bech32_hrp().await?;
            }
        }

        Ok(Wallet {
            inner: wallet_inner,
            accounts: Arc::new(RwLock::new(accounts)),
        })
    }

    #[cfg(feature = "storage")]
    pub(crate) async fn from_wallet(wallet: &Wallet) -> Self {
        Self {
            client_options: Some(ClientOptions::from_client(wallet.client()).await),
            coin_type: Some(wallet.coin_type.load(Ordering::Relaxed)),
            storage_options: Some(wallet.storage_options.clone()),
            secret_manager: Some(wallet.secret_manager.clone()),
        }
    }
}

// Check if any of the locked inputs is not used in a transaction and unlock them, so they get available for new
// transactions
#[cfg(feature = "storage")]
fn unlock_unused_inputs(accounts: &mut [AccountDetails]) -> crate::wallet::Result<()> {
    log::debug!("[unlock_unused_inputs]");
    for account in accounts.iter_mut() {
        let mut used_inputs = HashSet::new();
        for transaction_id in account.pending_transactions() {
            if let Some(tx) = account.transactions().get(transaction_id) {
                for input in &tx.inputs {
                    used_inputs.insert(input.metadata.output_id()?);
                }
            }
        }
        account.locked_outputs.retain(|input| {
            let used = used_inputs.contains(input);
            if !used {
                log::debug!("unlocking unused input {input}");
            }
            used
        })
    }
    Ok(())
}
