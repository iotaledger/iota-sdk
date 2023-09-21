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
use tokio::sync::RwLock;

use super::operations::storage::SaveLoadWallet;
#[cfg(feature = "events")]
use crate::wallet::events::EventEmitter;
#[cfg(all(feature = "storage", not(feature = "rocksdb")))]
use crate::wallet::storage::adapter::memory::Memory;
#[cfg(feature = "storage")]
use crate::wallet::{
    account::AccountDetails,
    storage::{StorageManager, StorageOptions},
};
use crate::{
    client::secret::{SecretManage, SecretManager},
    wallet::{
        core::{WalletData, WalletInner},
        Account, ClientOptions, Wallet,
    }, types::block::{address::{Address, AccountAddress}, output::AccountId},
};

/// Builder for the wallet inner.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBuilder<S: SecretManage = SecretManager> {
    pub(crate) alias: Option<String>,
    pub(crate) address: Option<Address>,
    pub(crate) bip_path: Option<Bip44>,
    pub(crate) client_options: Option<ClientOptions>,
    #[cfg(feature = "storage")]
    pub(crate) storage_options: Option<StorageOptions>,
    #[serde(skip)]
    pub(crate) secret_manager: Option<Arc<RwLock<S>>>,
}

impl<S: SecretManage> Default for WalletBuilder<S> {
    fn default() -> Self {
        Self {
            alias: Default::default(),
            address: Default::default(),
            bip_path: Default::default(),
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

    /// Set the alias of the wallet.
    pub fn with_alias(mut self, alias: impl Into<Option<String>>) -> Self {
        self.alias = alias.into();
        self
    }

    /// Set the address of the wallet.
    pub fn with_address(mut self, address: impl Into<Option<Address>>) -> Self {
        self.address = address.into();
        self
    }

    /// Set the BIP44 path of the wallet.
    pub fn with_bip44(mut self, bip_path: impl Into<Option<Bip44>>) -> Self {
        self.bip_path = bip_path.into();
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
            if self.client_options.is_none() {
                return Err(crate::wallet::Error::MissingParameter("client_options"));
            }
            todo!("move this to wallet builder");
            // if self.coin_type.is_none() {
            //     return Err(crate::wallet::Error::MissingParameter("coin_type"));
            // }
            if self.secret_manager.is_none() {
                return Err(crate::wallet::Error::MissingParameter("secret_manager"));
            }
        }

        #[cfg(all(feature = "rocksdb", feature = "storage"))]
        let storage =
            crate::wallet::storage::adapter::rocksdb::RocksdbStorageAdapter::new(storage_options.path.clone())?;
        #[cfg(all(not(feature = "rocksdb"), feature = "storage"))]
        let storage = Memory::default();

        #[cfg(feature = "storage")]
        let mut storage_manager = StorageManager::new(storage, storage_options.encryption_key.clone()).await?;

        #[cfg(feature = "storage")]
        let read_manager_builder = Self::load(&storage_manager).await?;
        #[cfg(not(feature = "storage"))]
        let read_manager_builder: Option<Self> = None;

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

        todo!("access bip44 path");
        // if self.coin_type.is_none() {
        //     self.coin_type = read_manager_builder.and_then(|builder| builder.coin_type);
        // }
        // let coin_type = self.coin_type.ok_or(crate::wallet::Error::MissingParameter(
        //     "coin_type (IOTA: 4218, Shimmer: 4219)",
        // ))?;

        todo!("get account from storage");
        // #[cfg(feature = "storage")]
        // let mut accounts = storage_manager.get_accounts().await?;

        // // Check against potential account coin type before saving the wallet data
        // #[cfg(feature = "storage")]
        // if let Some(account) = accounts.first() {
        //     if *account.coin_type() != coin_type {
        //         return Err(crate::wallet::Error::InvalidCoinType {
        //             new_coin_type: coin_type,
        //             existing_coin_type: *account.coin_type(),
        //         });
        //     }
        // }

        // Store wallet data in storage
        #[cfg(feature = "storage")]
        self.save(&storage_manager).await?;

        #[cfg(feature = "events")]
        let event_emitter = tokio::sync::RwLock::new(EventEmitter::new());

        // It happened that inputs got locked, the transaction failed, but they weren't unlocked again, so we do this
        // here
        todo!("single account");
        // #[cfg(feature = "storage")]
        // unlock_unused_inputs(&mut accounts)?;

        todo!("single account");
        // #[cfg(not(feature = "storage"))]
        // let accounts = Vec::new();

        let wallet_inner = Arc::new(WalletInner {
            background_syncing_status: AtomicUsize::new(0),
            client: self
                .client_options
                .clone()
                .ok_or(crate::wallet::Error::MissingParameter("client_options"))?
                .finish()
                .await?,
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

        todo!("single account");
        // let mut accounts: Vec<Account<S>> = try_join_all(
        //     accounts
        //         .into_iter()
        //         .map(|a| Account::new(a, wallet_inner.clone()).boxed()),
        // )
        // .await?;

        // If the wallet builder is not set, it means the user provided it and we need to update the addresses.
        // In the other case it was loaded from the database and addresses are up to date.
        if new_provided_client_options {
            todo!("single account");
            // for account in accounts.iter_mut() {
            //     // Safe to unwrap because we create the client if accounts aren't empty
            //     account.update_account_bech32_hrp().await?;
            // }
        }

        todo!("remove unwraps");
        let wallet_data = Arc::new(RwLock::new(WalletData::new(self.alias.unwrap(), self.bip_path.unwrap(), self.address.unwrap())));

        Ok(Wallet {
            inner: wallet_inner,
            data: wallet_data,
        })
    }

    #[cfg(feature = "storage")]
    pub(crate) async fn from_wallet(wallet: &Wallet<S>) -> Self {
        Self {
            alias: Some(wallet.data.read().await.alias.clone()),
            bip_path: Some(wallet.data.read().await.bip_path.clone()),
            address: Some(wallet.data.read().await.address.clone()),
            client_options: Some(wallet.client_options().await),
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
                    used_inputs.insert(*input.metadata.output_id());
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

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::Deserialize;

    use super::*;
    #[cfg(feature = "storage")]
    use crate::{client::secret::SecretManage, wallet::storage::StorageOptions};

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WalletBuilderDto {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) alias: Option<String>,
        pub(crate) bip_path: String,
        pub(crate) address: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) client_options: Option<ClientOptions>,
        #[cfg(feature = "storage")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) storage_options: Option<StorageOptions>,
    }

    impl<S: SecretManage> From<WalletBuilderDto> for WalletBuilder<S> {
        fn from(value: WalletBuilderDto) -> Self {
            todo!("make this TryFrom");
            // Self {
            //     alias: value.alias.unwrap_or_else("0".to_string()),
            //     bip_path: value.bip_path,
            //     address: value.address,
            //     client_options: value.client_options,
            //     #[cfg(feature = "storage")]
            //     storage_options: value.storage_options,
            //     secret_manager: None,
            // }
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
