// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "stronghold")]
use std::path::PathBuf;

use derivative::Derivative;
#[cfg(feature = "events")]
use iota_sdk::wallet::events::types::{WalletEvent, WalletEventType};
use iota_sdk::{
    client::{node_manager::node::NodeAuth, secret::GenerateAddressOptions},
    types::block::address::{Bech32Address, Hrp},
    wallet::{ClientOptions, SyncOptions},
};
use serde::{Deserialize, Serialize};
use url::Url;

#[cfg(feature = "stronghold")]
use crate::OmittedDebug;

/// The methods that can be sent to the actor.
#[derive(Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
#[non_exhaustive]
pub enum WalletMethod {
    /// Consume an account method.
    /// Returns [`Response`](crate::Response)
    #[serde(rename_all = "camelCase")]
    CallMethod {
        /// The wallet operation method to call.
        method: super::WalletOperationMethod,
    },
    /// Backup storage. Password must be the current one, when Stronghold is used as SecretManager.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    Backup {
        /// The backup destination.
        destination: PathBuf,
        /// Stronghold file password.
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        password: String,
    },
    /// Change the Stronghold password to another one and also re-encrypt the values in the loaded snapshot with it.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[serde(rename_all = "camelCase")]
    ChangeStrongholdPassword {
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        current_password: String,
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        new_password: String,
    },
    /// Clears the Stronghold password from memory.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    ClearStrongholdPassword,
    /// Checks if the Stronghold password is available.
    /// Expected response:
    /// [`Bool`](crate::Response::Bool)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    IsStrongholdPasswordAvailable,
    /// Restore a backup from a Stronghold file
    /// Replaces client_options, coin_type, secret_manager and accounts. Returns an error if accounts were already
    /// created If Stronghold is used as secret_manager, the existing Stronghold file will be overwritten. If a
    /// mnemonic was stored, it will be gone.
    /// if ignore_if_coin_type_mismatch.is_some(), client options will not be restored
    /// if ignore_if_coin_type_mismatch == Some(true), client options coin type and accounts will not be restored if
    /// the cointype doesn't match
    /// if ignore_if_bech32_hrp_mismatch == Some("rms"), but addresses have something different like "smr", no accounts
    /// will be restored.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[serde(rename_all = "camelCase")]
    RestoreBackup {
        /// The path to the backed up Stronghold.
        source: PathBuf,
        /// Stronghold file password.
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        password: String,
        /// If ignore_if_coin_type_mismatch.is_some(), client options will not be restored.
        /// If ignore_if_coin_type_mismatch == Some(true), client options coin type and accounts will not be restored
        /// if the cointype doesn't match.
        ignore_if_coin_type_mismatch: Option<bool>,
        /// If ignore_if_bech32_hrp_mismatch == Some("rms"), but addresses have something different like "smr", no
        /// accounts will be restored.
        ignore_if_bech32_mismatch: Option<Hrp>,
    },
    /// Updates the client options for all accounts.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[serde(rename_all = "camelCase")]
    SetClientOptions { client_options: Box<ClientOptions> },
    /// Generate an address without storing it
    /// Expected response: [`Bech32Address`](crate::Response::Bech32Address)
    #[serde(rename_all = "camelCase")]
    GenerateEd25519Address {
        /// Account index
        account_index: u32,
        /// Account index
        address_index: u32,
        /// Options
        options: Option<GenerateAddressOptions>,
        /// Bech32 HRP
        bech32_hrp: Option<Hrp>,
    },
    /// Get the ledger nano status
    /// Expected response: [`LedgerNanoStatus`](crate::Response::LedgerNanoStatus)
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    GetLedgerNanoStatus,
    /// Set the stronghold password.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    SetStrongholdPassword {
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        password: String,
    },
    /// Set the stronghold password clear interval.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[serde(rename_all = "camelCase")]
    SetStrongholdPasswordClearInterval { interval_in_milliseconds: Option<u64> },
    /// Store a mnemonic into the Stronghold vault.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    StoreMnemonic {
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        mnemonic: String,
    },
    /// Start background syncing.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[serde(rename_all = "camelCase")]
    StartBackgroundSync {
        /// Sync options
        options: Option<SyncOptions>,
        /// Interval in milliseconds
        interval_in_milliseconds: Option<u64>,
    },
    /// Stop background syncing.
    /// Expected response: [`Ok`](crate::Response::Ok)
    StopBackgroundSync,
    /// Emits an event for testing if the event system is working
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    EmitTestEvent { event: WalletEvent },
    // Remove all listeners of this type. Empty vec clears all listeners
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    #[serde(rename_all = "camelCase")]
    ClearListeners { event_types: Vec<WalletEventType> },
    /// Update the authentication for the provided node.
    /// Expected response: [`Ok`](crate::Response::Ok)
    UpdateNodeAuth {
        /// Node url
        url: Url,
        /// Authentication options
        auth: Option<NodeAuth>,
    },
}
