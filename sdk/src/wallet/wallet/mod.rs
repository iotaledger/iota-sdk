// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod builder;
pub(crate) mod operations;

use std::sync::{
    atomic::{AtomicU32, AtomicUsize},
    Arc,
};

use tokio::sync::RwLock;

#[cfg(feature = "storage")]
use self::builder::StorageOptions;
pub use self::builder::WalletBuilder;
#[cfg(feature = "events")]
use crate::wallet::events::{
    types::{Event, WalletEventType},
    EventEmitter,
};
#[cfg(feature = "storage")]
use crate::wallet::storage::manager::StorageManager;
use crate::{
    client::{secret::SecretManager, verify_mnemonic, Client},
    wallet::account::{builder::AccountBuilder, operations::syncing::SyncOptions, types::Balance, Account},
};

/// The wallet, used to create and get accounts. One wallet can hold many accounts, but they should
/// all share the same secret_manager type with the same seed/mnemonic.
#[derive(Debug, Clone)]
pub struct Wallet {
    pub(crate) inner: Arc<WalletInner>,
    // TODO should we use a hashmap instead of a vec?
    pub(crate) accounts: Arc<RwLock<Vec<Account>>>,
}

impl core::ops::Deref for Wallet {
    type Target = WalletInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Wallet {
    /// Initialises the wallet builder.
    pub fn builder() -> WalletBuilder {
        WalletBuilder::new()
    }

    /// Create a new account
    pub fn create_account(&self) -> AccountBuilder {
        log::debug!("creating account");
        AccountBuilder::new(self.clone())
    }
}

#[derive(Debug)]
pub struct WalletInner {
    // 0 = not running, 1 = running, 2 = stopping
    pub(crate) background_syncing_status: AtomicUsize,
    pub(crate) client: Client,
    pub(crate) coin_type: AtomicU32,
    pub(crate) secret_manager: Arc<RwLock<SecretManager>>,
    #[cfg(feature = "events")]
    pub(crate) event_emitter: tokio::sync::RwLock<EventEmitter>,
    #[cfg(feature = "storage")]
    pub(crate) storage_options: StorageOptions,
    #[cfg(feature = "storage")]
    pub(crate) storage_manager: tokio::sync::RwLock<StorageManager>,
}

impl Wallet {
    /// Get all accounts
    pub async fn get_accounts(&self) -> crate::wallet::Result<Vec<Account>> {
        Ok(self.accounts.read().await.clone())
    }

    /// Get all account aliases
    pub async fn get_account_aliases(&self) -> crate::wallet::Result<Vec<String>> {
        let accounts = self.accounts.read().await;
        let mut aliases = Vec::with_capacity(accounts.len());
        for handle in accounts.iter() {
            aliases.push(handle.details().await.alias().clone());
        }
        Ok(aliases)
    }

    /// Removes the latest account (account with the largest account index).
    pub async fn remove_latest_account(&self) -> crate::wallet::Result<()> {
        let mut largest_account_index_opt = None;
        let mut accounts = self.accounts.write().await;

        for account in accounts.iter() {
            let account_index = *account.details().await.index();
            if let Some(largest_account_index) = largest_account_index_opt {
                if account_index > largest_account_index {
                    largest_account_index_opt = Some(account_index);
                }
            } else {
                largest_account_index_opt = Some(account_index)
            }
        }

        if let Some(largest_account_index) = largest_account_index_opt {
            for i in 0..accounts.len() {
                if let Some(account) = accounts.get(i) {
                    if *account.details().await.index() == largest_account_index {
                        let _ = accounts.remove(i);

                        #[cfg(feature = "storage")]
                        self.storage_manager
                            .write()
                            .await
                            .remove_account(largest_account_index)
                            .await?;

                        return Ok(());
                    }
                }
            }
        }

        Ok(())
    }

    /// Get the balance of all accounts added together
    pub async fn balance(&self) -> crate::wallet::Result<Balance> {
        let mut balance = Balance::default();
        let accounts = self.accounts.read().await;

        for account in accounts.iter() {
            balance += account.balance().await?;
        }

        Ok(balance)
    }

    /// Sync all accounts
    pub async fn sync(&self, options: Option<SyncOptions>) -> crate::wallet::Result<Balance> {
        let mut balance = Balance::default();

        for account in self.accounts.read().await.iter() {
            balance += account.sync(options.clone()).await?;
        }

        Ok(balance)
    }
}

impl WalletInner {
    /// Get the [SecretManager]
    pub fn get_secret_manager(&self) -> &Arc<RwLock<SecretManager>> {
        &self.secret_manager
    }

    /// Listen to wallet events, empty vec will listen to all events
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    pub async fn listen<F, I: IntoIterator<Item = WalletEventType> + Send>(&self, events: I, handler: F)
    where
        I::IntoIter: Send,
        F: Fn(&Event) + 'static + Clone + Send + Sync,
    {
        let mut emitter = self.event_emitter.write().await;
        emitter.on(events, handler);
    }

    /// Remove wallet event listeners, empty vec will remove all listeners
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    pub async fn clear_listeners<I: IntoIterator<Item = WalletEventType> + Send>(&self, events: I)
    where
        I::IntoIter: Send,
    {
        let mut emitter = self.event_emitter.write().await;
        emitter.clear(events);
    }

    /// Generates a new random mnemonic.
    pub fn generate_mnemonic(&self) -> crate::wallet::Result<String> {
        Ok(Client::generate_mnemonic()?)
    }

    /// Verify that a &str is a valid mnemonic.
    pub fn verify_mnemonic(&self, mnemonic: &str) -> crate::wallet::Result<()> {
        verify_mnemonic(mnemonic)?;
        Ok(())
    }

    #[cfg(feature = "events")]
    pub(crate) async fn emit(&self, account_index: u32, event: crate::wallet::events::types::WalletEvent) {
        self.event_emitter.read().await.emit(account_index, event);
    }

    /// Helper function to test events. Emits a provided event with account index 0.
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    pub async fn emit_test_event(&self, event: crate::wallet::events::types::WalletEvent) {
        self.emit(0, event).await
    }
}

impl Drop for Wallet {
    fn drop(&mut self) {
        log::debug!("drop Wallet");
    }
}
