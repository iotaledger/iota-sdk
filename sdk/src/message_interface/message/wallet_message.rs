// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "stronghold")]
use std::path::PathBuf;

use derivative::Derivative;
use serde::{Deserialize, Serialize};

use super::{account_method::AccountMethod, OmmittedDebug};
#[cfg(feature = "events")]
use crate::wallet::events::types::{WalletEvent, WalletEventType};
use crate::{
    client::{node_manager::node::NodeAuth, secret::GenerateAddressOptions, Url},
    wallet::{
        account::{operations::syncing::SyncOptions, types::AccountIdentifier},
        ClientOptions,
    },
};

/// The messages that can be sent to the actor.
#[derive(Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
#[serde(tag = "cmd", content = "payload", rename_all = "camelCase")]
#[allow(clippy::large_enum_variant)]
pub enum WalletMessage {
    /// Creates an account.
    /// Expected response: [`Account`](crate::message_interface::Response::Account)
    CreateAccount {
        /// The account alias.
        alias: Option<String>,
        /// The bech32 HRP.
        #[serde(rename = "bech32Hrp")]
        bech32_hrp: Option<String>,
    },
    /// Read account.
    /// Expected response: [`Account`](crate::message_interface::Response::Account)
    GetAccount {
        #[serde(rename = "accountId")]
        account_id: AccountIdentifier,
    },
    /// Return the account indexes.
    /// Expected response: [`AccountIndexes`](crate::message_interface::Response::AccountIndexes)
    GetAccountIndexes,
    /// Read accounts.
    /// Expected response: [`Accounts`](crate::message_interface::Response::Accounts)
    GetAccounts,
    /// Consume an account method.
    /// Returns [`Response`](crate::message_interface::Response)
    CallAccountMethod {
        /// The account identifier.
        #[serde(rename = "accountId")]
        account_id: AccountIdentifier,
        /// The account method to call.
        method: AccountMethod,
    },
    /// Backup storage. Password must be the current one, when Stronghold is used as SecretManager.
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    Backup {
        /// The backup destination.
        destination: PathBuf,
        /// Stronghold file password.
        #[derivative(Debug(format_with = "OmmittedDebug::omitted_fmt"))]
        password: String,
    },
    /// Change the Stronghold password to another one and also re-encrypt the values in the loaded snapshot with it.
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    ChangeStrongholdPassword {
        #[derivative(Debug(format_with = "OmmittedDebug::omitted_fmt"))]
        #[serde(rename = "currentPassword")]
        current_password: String,
        #[derivative(Debug(format_with = "OmmittedDebug::omitted_fmt"))]
        #[serde(rename = "newPassword")]
        new_password: String,
    },
    /// Clears the Stronghold password from memory.
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    ClearStrongholdPassword,
    /// Checks if the Stronghold password is available.
    /// Expected response:
    /// [`StrongholdPasswordIsAvailable`](crate::message_interface::Response::StrongholdPasswordIsAvailable)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    IsStrongholdPasswordAvailable,
    /// Find accounts with unspent outputs
    /// Expected response: [`Accounts`](crate::message_interface::Response::Accounts)
    RecoverAccounts {
        #[serde(rename = "accountStartIndex")]
        /// The index of the first account to search for.
        account_start_index: u32,
        #[serde(rename = "accountGapLimit")]
        /// The number of accounts to search for, after the last account with unspent outputs.
        account_gap_limit: u32,
        #[serde(rename = "addressGapLimit")]
        /// The number of addresses to search for, after the last address with unspent outputs, in
        /// each account.
        address_gap_limit: u32,
        #[serde(rename = "syncOptions")]
        /// Optional parameter to specify the sync options. The `address_start_index` and `force_syncing`
        /// fields will be overwritten to skip existing addresses.
        sync_options: Option<SyncOptions>,
    },
    /// Restore a backup from a Stronghold file
    /// Replaces client_options, coin_type, secret_manager and accounts. Returns an error if accounts were already
    /// created If Stronghold is used as secret_manager, the existing Stronghold file will be overwritten. If a
    /// mnemonic was stored, it will be gone.
    /// if ignore_if_coin_type_mismatch.is_some(), client options will not be restored
    /// if ignore_if_coin_type_mismatch == Some(true), client options coin type and accounts will not be restored if
    /// the cointype doesn't match Expected response: [`Ok`](crate::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    RestoreBackup {
        /// The path to the backed up Stronghold.
        source: PathBuf,
        /// Stronghold file password.
        #[derivative(Debug(format_with = "OmmittedDebug::omitted_fmt"))]
        password: String,
        #[serde(rename = "ignoreIfCoinTypeMismatch")]
        ignore_if_coin_type_mismatch: Option<bool>,
    },
    /// Removes the latest account (account with the largest account index).
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    RemoveLatestAccount,
    /// Generates a new mnemonic.
    /// Expected response: [`GeneratedMnemonic`](crate::message_interface::Response::GeneratedMnemonic)
    GenerateMnemonic,
    /// Checks if the given mnemonic is valid.
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    VerifyMnemonic {
        #[derivative(Debug(format_with = "OmmittedDebug::omitted_fmt"))]
        mnemonic: String,
    },
    /// Updates the client options for all accounts.
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    SetClientOptions {
        #[serde(rename = "clientOptions")]
        client_options: Box<ClientOptions>,
    },
    /// Generate an address without storing it
    /// Expected response: [`Bech32Address`](crate::message_interface::Response::Bech32Address)
    GenerateAddress {
        /// Account index
        #[serde(rename = "accountIndex")]
        account_index: u32,
        /// Internal address
        internal: bool,
        /// Account index
        #[serde(rename = "addressIndex")]
        address_index: u32,
        /// Options
        options: Option<GenerateAddressOptions>,
        /// Bech32 HRP
        #[serde(rename = "bech32Hrp")]
        bech32_hrp: Option<String>,
    },
    /// Get the ledger nano status
    /// Expected response: [`LedgerNanoStatus`](crate::message_interface::Response::LedgerNanoStatus)
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    GetLedgerNanoStatus,
    /// Get the node information
    /// Expected response: [`NodeInfo`](crate::message_interface::Response::NodeInfo)
    GetNodeInfo {
        /// Url
        url: Option<String>,
        /// Node authentication
        auth: Option<NodeAuth>,
    },
    /// Set the stronghold password.
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    SetStrongholdPassword {
        #[derivative(Debug(format_with = "OmmittedDebug::omitted_fmt"))]
        password: String,
    },
    /// Set the stronghold password clear interval.
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    SetStrongholdPasswordClearInterval {
        #[serde(rename = "intervalInMilliseconds")]
        interval_in_milliseconds: Option<u64>,
    },
    /// Store a mnemonic into the Stronghold vault.
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    StoreMnemonic {
        #[derivative(Debug(format_with = "OmmittedDebug::omitted_fmt"))]
        mnemonic: String,
    },
    /// Start background syncing.
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    StartBackgroundSync {
        /// Sync options
        options: Option<SyncOptions>,
        /// Interval in milliseconds
        #[serde(rename = "intervalInMilliseconds")]
        interval_in_milliseconds: Option<u64>,
    },
    /// Stop background syncing.
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    StopBackgroundSync,
    /// Emits an event for testing if the event system is working
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    EmitTestEvent { event: WalletEvent },
    /// Transforms a bech32 encoded address to hex
    /// Expected response: [`HexAddress`](crate::message_interface::Response::HexAddress)
    Bech32ToHex {
        #[serde(rename = "bech32Address")]
        bech32_address: String,
    },
    /// Transforms a hex encoded address to a bech32 encoded address
    /// Expected response: [`Bech32Address`](crate::message_interface::Response::Bech32Address)
    HexToBech32 {
        /// Hex encoded bech32 address
        hex: String,
        /// Human readable part
        #[serde(rename = "bech32Hrp")]
        bech32_hrp: Option<String>,
    },
    // Remove all listeners of this type. Empty vec clears all listeners
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    ClearListeners {
        #[serde(rename = "eventTypes")]
        event_types: Vec<WalletEventType>,
    },
    /// Update the authentication for the provided node.
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    UpdateNodeAuth {
        /// Node url
        url: Url,
        /// Authentication options
        auth: Option<NodeAuth>,
    },
}
