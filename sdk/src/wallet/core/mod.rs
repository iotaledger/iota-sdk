// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod builder;
pub(crate) mod operations;

use std::{sync::{
    atomic::{AtomicU32, AtomicUsize},
    Arc,
}, collections::{HashMap, HashSet}};

use crypto::keys::{bip39::{Mnemonic, MnemonicRef}, bip44::Bip44};
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
    wallet::account::{builder::AccountBuilder, operations::syncing::SyncOptions, types::Balance, Account}, types::block::{address::Address, output::{OutputId, FoundryOutput, FoundryId}, payload::transaction::TransactionId},
};

use super::account::types::{OutputData, Transaction};

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
            data: self.data.clone(),
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

    // TODO: remove
    // pub fn create_account(&self) -> AccountBuilder<S> {
    //     log::debug!("creating account");
    //     AccountBuilder::<S>::new(self.clone())
    // }
}

#[derive(Debug)]
pub struct WalletInner<S: SecretManage = SecretManager> {
    // 0 = not running, 1 = running, 2 = stopping
    pub(crate) background_syncing_status: AtomicUsize,
    pub(crate) client: Client,
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
    }

    // fn address() -> Ed25519Address {
    // }

    // fn implicit_account_address() -> ImplicitAccountAddress {
    //     // Based on Self::address()
    //     ...
    // }

    // fn implicit_accounts() -> Vec<ImplicitAccount> {
    //     let output = self.unspent_outputs.find(ImplcitType);
    //     ImplicitAccount {
    //         output,
    //         wallet: self
    //     }
    // }

    // fn issuer_accounts() -> Vec<Account> {

    // }
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

#[derive(Debug, Clone)]
pub struct WalletData {
    pub(crate) alias: String,
    pub(crate) bip_path: Bip44,
    pub(crate) address: Address,
    pub(crate) outputs: HashMap<OutputId, OutputData>,
    pub(crate) locked_outputs: HashSet<OutputId>,
    pub(crate) unspent_outputs: HashMap<OutputId, OutputData>,
    pub(crate) transactions: HashMap<TransactionId, Transaction>,
    pub(crate) pending_transactions: HashSet<TransactionId>,
    pub(crate) incoming_transactions: HashMap<TransactionId, Transaction>,
    pub(crate) inaccessible_incoming_transactions: HashSet<TransactionId>,
    pub(crate) native_token_foundries: HashMap<FoundryId, FoundryOutput>,
}

impl WalletData {
    pub(crate) fn new(alias: String, bip_path: Bip44, address: Address) -> Self {
        Self {
            alias,
            bip_path,
            address,
            outputs: HashMap::new(),
            locked_outputs: HashSet::new(),
            unspent_outputs: HashMap::new(),
            transactions: HashMap::new(),
            pending_transactions: HashSet::new(),
            incoming_transactions: HashMap::new(),
            inaccessible_incoming_transactions: HashSet::new(),
            native_token_foundries: HashMap::new(),
        }
    }

    pub(crate) fn coin_type(&self) -> u32 {
        self.bip_path.coin_type
    }
}
