// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "storage")]
use std::collections::HashSet;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};

use super::operations::storage::SaveLoadWallet;
#[cfg(feature = "events")]
use crate::wallet::events::EventEmitter;
#[cfg(all(feature = "storage", not(feature = "rocksdb")))]
use crate::wallet::storage::adapter::memory::Memory;
#[cfg(feature = "storage")]
use crate::wallet::storage::{StorageManager, StorageOptions};
use crate::{
    client::secret::{SecretManage, SecretManager},
    types::block::address::Bech32Address,
    wallet::{
        core::{operations::background_syncing::BackgroundSyncStatus, Bip44, WalletInner, WalletLedger},
        operations::syncing::SyncOptions,
        ClientOptions, Wallet,
    },
};

/// An address provider for the wallet builder.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AddressProvider {
    /// The address will be provided manually. If `bip_path` is set, then address must be an Ed25519 address. If
    /// `address` is anything other than an Ed25519 address, `bip_path` must be `None`.
    Manual {
        address: Bech32Address,
        bip_path: Option<Bip44>,
    },
    /// The address will be generated using a configured secret manager.
    Chain(Bip44),
}

impl From<Bech32Address> for AddressProvider {
    fn from(value: Bech32Address) -> Self {
        Self::Manual {
            address: value,
            bip_path: None,
        }
    }
}

impl From<Bip44> for AddressProvider {
    fn from(value: Bip44) -> Self {
        Self::Chain(value)
    }
}

impl From<(Bech32Address, Option<Bip44>)> for AddressProvider {
    fn from(value: (Bech32Address, Option<Bip44>)) -> Self {
        Self::Manual {
            address: value.0,
            bip_path: value.1,
        }
    }
}

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
    crate::wallet::Error: From<S::Error>,
{
    /// Initialises a new instance of the wallet builder with the default storage adapter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the address provider of the wallet.
    pub fn with_address(mut self, provider: impl Into<AddressProvider>) -> Self {
        match provider.into() {
            AddressProvider::Manual { address, bip_path } => {
                self.address = Some(address);
                self.bip_path = bip_path;
            }
            AddressProvider::Chain(bip_path) => self.bip_path = Some(bip_path),
        }
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
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
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

        // May use a previously stored wallet alias if it wasn't provided
        if self.alias.is_none() {
            self.alias = loaded_wallet_builder.as_ref().and_then(|builder| builder.alias.clone());
        }

        if self.address.is_none() {
            self.address = loaded_wallet_builder
                .as_ref()
                .and_then(|builder| builder.address.clone());
        }

        // Create the node client.
        let client = self
            .client_options
            .clone()
            .ok_or(crate::wallet::Error::MissingParameter("client_options"))?
            .finish()
            .await?;

        let hrp = client.get_bech32_hrp().await?;

        let (wallet_address, wallet_bip_path) = match (self.address.as_ref(), self.bip_path.as_ref()) {
            (Some(address), bip_path) => {
                if let Some(bip_path) = bip_path {
                    if let Some(backing_ed25519_address) = address.inner.backing_ed25519() {
                        // we need to verify that the address is derived from the provided bip path
                        if let Some(ref secret_manager) = self.secret_manager {
                            let secret_manager = &*secret_manager.read().await;
                            let generated_ed25519_address = secret_manager
                                .generate_ed25519_addresses(
                                    bip_path.coin_type,
                                    bip_path.account,
                                    bip_path.address_index..bip_path.address_index + 1,
                                    None,
                                )
                                .await?[0];
                            if backing_ed25519_address == &generated_ed25519_address {
                                (address.clone(), Some(*bip_path))
                            } else {
                                return Err(crate::wallet::Error::InvalidParameter("address/bip-path mismatch"));
                            }
                        } else {
                            return Err(crate::wallet::Error::MissingParameter("secret manager"));
                        }
                    } else {
                        return Err(crate::wallet::Error::InvalidParameter("address/bip path mismatch"));
                    }
                } else {
                    // the wallet only provides a view into that address
                    (address.clone(), None)
                }
            }
            (None, Some(bip_path)) => {
                if let Some(ref secret_manager) = self.secret_manager {
                    let secret_manager = &*secret_manager.read().await;
                    let generated_ed25519_address = secret_manager
                        .generate_ed25519_addresses(
                            bip_path.coin_type,
                            bip_path.account,
                            bip_path.address_index..bip_path.address_index + 1,
                            None,
                        )
                        .await?[0];

                    // provided_client_options.
                    (Bech32Address::new(hrp, generated_ed25519_address), Some(*bip_path))
                } else {
                    return Err(crate::wallet::Error::MissingParameter("secret manager"));
                }
            }
            (None, None) => {
                return Err(crate::wallet::Error::MissingParameter("address/bip_path"));
            }
        };

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

        let background_syncing_status = tokio::sync::watch::channel(BackgroundSyncStatus::Stopped);
        let background_syncing_status = (Arc::new(background_syncing_status.0), background_syncing_status.1);

        // Build the wallet.
        let wallet_inner = WalletInner {
            default_sync_options: Mutex::new(SyncOptions::default()),
            last_synced: Mutex::new(0),
            background_syncing_status,
            client,
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
            address: Arc::new(RwLock::new(wallet_address)),
            bip_path: Arc::new(RwLock::new(wallet_bip_path)),
            alias: Arc::new(RwLock::new(self.alias)),
            inner: Arc::new(wallet_inner),
            ledger: Arc::new(RwLock::new(wallet_ledger)),
        };

        // If the wallet builder is not set, it means the user provided it and we need to update the addresses.
        // In the other case it was loaded from the database and addresses are up to date.
        if provided_client_options {
            wallet.update_wallet_address_hrp().await?;
        }

        Ok(wallet)
    }

    // TODO #1941: remove!
    // /// Generate the wallet address.
    // pub(crate) async fn create_default_wallet_address(&self) -> crate::wallet::Result<Bech32Address> {
    //     let bech32_hrp = self
    //         .client_options
    //         .as_ref()
    //         .unwrap()
    //         .network_info
    //         .protocol_parameters
    //         .bech32_hrp;
    //     let bip_path = self.bip_path.as_ref().unwrap();
    //
    //     Ok(Bech32Address::new(
    //         bech32_hrp,
    //         Address::Ed25519(
    //             self.secret_manager
    //                 .as_ref()
    //                 .unwrap()
    //                 .read()
    //                 .await
    //                 .generate_ed25519_addresses(
    //                     bip_path.coin_type,
    //                     bip_path.account,
    //                     bip_path.address_index..bip_path.address_index + 1,
    //                     GenerateAddressOptions {
    //                         internal: bip_path.change != 0,
    //                         ledger_nano_prompt: false,
    //                     },
    //                 )
    //                 .await?[0],
    //         ),
    //     ))
    // }

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
}

// Check if any of the locked inputs is not used in a transaction and unlock them, so they get available for new
// transactions
#[cfg(feature = "storage")]
fn unlock_unused_inputs(wallet_ledger: &mut WalletLedger) -> crate::wallet::Result<()> {
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
