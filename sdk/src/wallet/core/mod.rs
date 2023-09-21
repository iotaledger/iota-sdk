// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod builder;
pub(crate) mod operations;

use std::sync::{
    atomic::{AtomicU32, AtomicUsize},
    Arc,
};

use crypto::keys::bip39::{Mnemonic, MnemonicRef};
use tokio::sync::RwLock;

pub use self::builder::WalletBuilder;
#[cfg(feature = "events")]
use crate::wallet::events::{
    types::{Event, WalletEventType},
    EventEmitter,
};
#[cfg(feature = "storage")]
use crate::wallet::storage::{StorageManager, StorageOptions};
use crate::{
    client::{
        secret::{SecretManage, SecretManager},
        verify_mnemonic, Client,
    },
    wallet::account::{builder::AccountBuilder, operations::syncing::SyncOptions, types::Balance, Account},
};

/// The wallet, used to ... TODO
#[derive(Debug)]
pub struct Wallet<S: SecretManage = SecretManager> {
    pub(crate) inner: Arc<WalletInner<S>>,
    pub(crate) data: Arc<RwLock<WalletData>>,
}

impl<S: SecretManage> Clone for Wallet<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            // data: self.data.clone(),
        }
    }
}

impl<S: SecretManage> core::ops::Deref for Wallet<S> {
    type Target = WalletInner<S>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Initialises the wallet builder.
    pub fn builder() -> WalletBuilder<S> {
        WalletBuilder::<S>::new()
    }

    /// Create a new account
    pub fn create_account(&self) -> AccountBuilder<S> {
        log::debug!("creating account");
        AccountBuilder::<S>::new(self.clone())
    }
}

#[derive(Debug)]
pub struct WalletInner<S: SecretManage = SecretManager> {
    // 0 = not running, 1 = running, 2 = stopping
    pub(crate) background_syncing_status: AtomicUsize,
    pub(crate) client: Client,
    // TODO: remove in favor of WalletData::bip_path
    pub(crate) coin_type: AtomicU32,
    pub(crate) secret_manager: Arc<RwLock<S>>,
    #[cfg(feature = "events")]
    pub(crate) event_emitter: tokio::sync::RwLock<EventEmitter>,
    #[cfg(feature = "storage")]
    pub(crate) storage_options: StorageOptions,
    #[cfg(feature = "storage")]
    pub(crate) storage_manager: tokio::sync::RwLock<StorageManager>,
}

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Get all accounts
    pub async fn get_accounts(&self) -> crate::wallet::Result<Vec<Account<S>>> {
        // Ok(self.data.read().await.clone())
        todo!("get_data")
    }

    /// Get all account aliases
    pub async fn get_account_aliases(&self) -> crate::wallet::Result<Vec<String>> {
        // let accounts = self.data.read().await;
        // let mut account_aliases = Vec::with_capacity(accounts.len());
        // for handle in accounts.iter() {
        //     account_aliases.push(handle.details().await.alias().clone());
        // }
        // Ok(account_aliases)
        todo!("get_alias")
    }

    /// Removes the latest account (account with the largest account index).
    pub async fn remove_latest_account(&self) -> crate::wallet::Result<()> {
        // let mut largest_account_index_opt = None;
        // let mut accounts = self.data.write().await;

        // for account in accounts.iter() {
        //     let account_index = *account.details().await.index();
        //     if let Some(largest_account_index) = largest_account_index_opt {
        //         if account_index > largest_account_index {
        //             largest_account_index_opt = Some(account_index);
        //         }
        //     } else {
        //         largest_account_index_opt = Some(account_index)
        //     }
        // }

        // if let Some(largest_account_index) = largest_account_index_opt {
        //     for i in 0..accounts.len() {
        //         if let Some(account) = accounts.get(i) {
        //             if *account.details().await.index() == largest_account_index {
        //                 let _ = accounts.remove(i);

        //                 #[cfg(feature = "storage")]
        //                 self.storage_manager
        //                     .write()
        //                     .await
        //                     .remove_account(largest_account_index)
        //                     .await?;

        //                 return Ok(());
        //             }
        //         }
        //     }
        // }

        // Ok(())
        todo!("refactor to remove account");
    }

    /// Get the balance of all accounts added together
    pub async fn balance(&self) -> crate::wallet::Result<Balance> {
        let mut balance = Balance::default();
        todo!("just return the balance of the single account");
        // let accounts = self.data.read().await;

        // for account in accounts.iter() {
        //     balance += account.balance().await?;
        // }

        Ok(balance)
    }

    /// Sync all accounts
    pub async fn sync(&self, options: Option<SyncOptions>) -> crate::wallet::Result<Balance> {
        let mut balance = Balance::default();

        todo!("just sync the one account and return its balance");
        // for account in self.data.read().await.iter() {
        //     balance += account.sync(options.clone()).await?;
        // }

        Ok(balance)

        fn address() -> Ed25519Address {
            ...
        }
        
        fn implicit_account_address() -> ImplicitAccountAddress {
            // Based on Self::address()
            ...
        }
        
        fn implicit_accounts() -> Vec<ImplicitAccount> {
            let output = self.unspent_outputs.find(ImplcitType);
            ImplicitAccount {
                output,
                wallet: self
            }
        }
        
        fn issuer_accounts() -> Vec<Account> {
            
        }
    }
}

impl<S: SecretManage> WalletInner<S> {
    /// Get the [SecretManager]
    pub fn get_secret_manager(&self) -> &Arc<RwLock<S>> {
        &self.secret_manager
    }

    /// Listen to wallet events, empty vec will listen to all events
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    pub async fn listen<F, I: IntoIterator<Item = WalletEventType> + Send>(&self, events: I, handler: F)
    where
        I::IntoIter: Send,
        F: Fn(&Event) + 'static + Send + Sync,
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
    pub fn generate_mnemonic(&self) -> crate::wallet::Result<Mnemonic> {
        Ok(Client::generate_mnemonic()?)
    }

    /// Verify that a &str is a valid mnemonic.
    pub fn verify_mnemonic(&self, mnemonic: &MnemonicRef) -> crate::wallet::Result<()> {
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

impl<S: SecretManage> Drop for Wallet<S> {
    fn drop(&mut self) {
        log::debug!("drop Wallet");
    }
}

pub struct WalletData {
    pub(crate) bip_path: BIP44,
    pub(crate) alias: String,
    pub(crate) address: Address,
    pub(crate) outputs: HashMap<OutputId, OutputData>,
    pub(crate) locked_outputs: HashSet<OutputId>,
    pub(crate) unspent_outputs: HashMap<OutputId, OutputData>,
    transactions: HashMap<TransactionId, Transaction>,
    pending_transactions: HashSet<TransactionId>,
    incoming_transactions: HashMap<TransactionId, Transaction>,
    inaccessible_incoming_transactions: HashSet<TransactionId>,
    native_token_foundries: HashMap<FoundryId, FoundryOutput>,
}
