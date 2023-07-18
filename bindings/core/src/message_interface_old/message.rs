// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::{Debug, Formatter, Result};
#[cfg(feature = "stronghold")]
use std::path::PathBuf;

#[cfg(feature = "events")]
use iota_sdk::wallet::events::types::{WalletEvent, WalletEventType};
use iota_sdk::{
    client::{node_manager::node::NodeAuth, secret::GenerateAddressOptions},
    types::block::address::{Bech32Address, Hrp},
    wallet::{
        account::{types::AccountIdentifier, SyncOptions},
        ClientOptions,
    },
    Url,
};
use serde::{Deserialize, Serialize};

use super::account_method::AccountMethod;

/// The messages that can be sent to the actor.
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "cmd", content = "payload", rename_all = "camelCase")]
pub enum Message {
    /// Creates an account.
    /// Expected response: [`Account`](crate::wallet::message_interface::Response::Account)
    #[serde(rename_all = "camelCase")]
    CreateAccount {
        /// The account alias.
        alias: Option<String>,
        /// The bech32 HRP.
        bech32_hrp: Option<Hrp>,
    },
    /// Read account.
    /// Expected response: [`Account`](crate::wallet::message_interface::Response::Account)
    #[serde(rename_all = "camelCase")]
    GetAccount { account_id: AccountIdentifier },
    /// Return the account indexes.
    /// Expected response: [`AccountIndexes`](crate::wallet::message_interface::Response::AccountIndexes)
    GetAccountIndexes,
    /// Read accounts.
    /// Expected response: [`Accounts`](crate::wallet::message_interface::Response::Accounts)
    GetAccounts,
    /// Consume an account method.
    /// Returns [`Response`](crate::wallet::message_interface::Response)
    #[serde(rename_all = "camelCase")]
    CallAccountMethod {
        /// The account identifier.
        account_id: AccountIdentifier,
        /// The account method to call.
        method: AccountMethod,
    },
    /// Backup storage. Password must be the current one, when Stronghold is used as SecretManager.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    Backup {
        /// The backup destination.
        destination: PathBuf,
        /// Stronghold file password.
        password: String,
    },
    /// Change the Stronghold password to another one and also re-encrypt the values in the loaded snapshot with it.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[serde(rename_all = "camelCase")]
    ChangeStrongholdPassword {
        current_password: String,
        new_password: String,
    },
    /// Clears the Stronghold password from memory.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    ClearStrongholdPassword,
    /// Checks if the Stronghold password is available.
    /// Expected response:
    /// [`Bool`](crate::wallet::message_interface::Response::Bool)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    IsStrongholdPasswordAvailable,
    /// Find accounts with unspent outputs
    /// Expected response: [`Accounts`](crate::wallet::message_interface::Response::Accounts)
    #[serde(rename_all = "camelCase")]
    RecoverAccounts {
        /// The index of the first account to search for.
        account_start_index: u32,
        /// The number of accounts to search for, after the last account with unspent outputs.
        account_gap_limit: u32,
        /// The number of addresses to search for, after the last address with unspent outputs, in
        /// each account.
        address_gap_limit: u32,
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
    /// the cointype doesn't match
    /// if ignore_if_bech32_hrp_mismatch == Some("rms"), but addresses have something different like "smr", no accounts
    /// will be restored.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[serde(rename_all = "camelCase")]
    RestoreBackup {
        /// The path to the backed up Stronghold.
        source: PathBuf,
        /// Stronghold file password.
        password: String,
        /// If ignore_if_coin_type_mismatch.is_some(), client options will not be restored.
        /// If ignore_if_coin_type_mismatch == Some(true), client options coin type and accounts will not be restored
        /// if the cointype doesn't match.
        ignore_if_coin_type_mismatch: Option<bool>,
        /// If ignore_if_bech32_hrp_mismatch == Some("rms"), but addresses have something different like "smr", no
        /// accounts will be restored.
        ignore_if_bech32_mismatch: Option<Hrp>,
    },
    /// Removes the latest account (account with the largest account index).
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    RemoveLatestAccount,
    /// Generates a new mnemonic.
    /// Expected response: [`GeneratedMnemonic`](crate::wallet::message_interface::Response::GeneratedMnemonic)
    GenerateMnemonic,
    /// Checks if the given mnemonic is valid.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    VerifyMnemonic { mnemonic: String },
    /// Updates the client options for all accounts.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    #[serde(rename_all = "camelCase")]
    SetClientOptions { client_options: Box<ClientOptions> },
    /// Generate an address without storing it
    /// Expected response: [`Bech32Address`](crate::wallet::message_interface::Response::Bech32Address)
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
    /// Expected response: [`LedgerNanoStatus`](crate::wallet::message_interface::Response::LedgerNanoStatus)
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    GetLedgerNanoStatus,
    /// Get the node information
    /// Expected response: [`NodeInfo`](crate::wallet::message_interface::Response::NodeInfo)
    GetNodeInfo {
        /// Url
        url: Option<String>,
        /// Node authentication
        auth: Option<NodeAuth>,
    },
    /// Set the stronghold password.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    SetStrongholdPassword { password: String },
    /// Set the stronghold password clear interval.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[serde(rename_all = "camelCase")]
    SetStrongholdPasswordClearInterval { interval_in_milliseconds: Option<u64> },
    /// Store a mnemonic into the Stronghold vault.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    StoreMnemonic { mnemonic: String },
    /// Start background syncing.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    #[serde(rename_all = "camelCase")]
    StartBackgroundSync {
        /// Sync options
        options: Option<SyncOptions>,
        /// Interval in milliseconds
        interval_in_milliseconds: Option<u64>,
    },
    /// Stop background syncing.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    StopBackgroundSync,
    /// Emits an event for testing if the event system is working
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    EmitTestEvent { event: WalletEvent },
    /// Transforms a bech32 encoded address to hex
    /// Expected response: [`HexAddress`](crate::wallet::message_interface::Response::HexAddress)
    #[serde(rename_all = "camelCase")]
    Bech32ToHex { bech32_address: Bech32Address },
    /// Transforms a hex encoded address to a bech32 encoded address
    /// Expected response: [`Bech32Address`](crate::wallet::message_interface::Response::Bech32Address)
    #[serde(rename_all = "camelCase")]
    HexToBech32 {
        /// Hex encoded bech32 address
        hex: String,
        /// Human readable part
        bech32_hrp: Option<Hrp>,
    },
    // Remove all listeners of this type. Empty vec clears all listeners
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    #[serde(rename_all = "camelCase")]
    ClearListeners { event_types: Vec<WalletEventType> },
    /// Update the authentication for the provided node.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    UpdateNodeAuth {
        /// Node url
        url: Url,
        /// Authentication options
        auth: Option<NodeAuth>,
    },
}

// Custom Debug implementation to not log secrets
impl Debug for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::CreateAccount { alias, bech32_hrp } => {
                write!(f, "CreateAccount{{ alias: {alias:?}, bech32_hrp: {bech32_hrp:?} }}")
            }
            Self::GetAccountIndexes => write!(f, "GetAccountIndexes"),
            Self::GetAccount { account_id } => write!(f, "GetAccount{{ account_id: {account_id:?} }}"),
            Self::GetAccounts => write!(f, "GetAccounts"),
            Self::CallAccountMethod { account_id, method } => write!(
                f,
                "CallAccountMethod{{ account_id: {account_id:?}, method: {method:?} }}"
            ),
            #[cfg(feature = "stronghold")]
            Self::ChangeStrongholdPassword {
                current_password: _,
                new_password: _,
            } => write!(
                f,
                "ChangeStrongholdPassword{{ current_password: <omitted>, new_password: <omitted> }}"
            ),
            #[cfg(feature = "stronghold")]
            Self::ClearStrongholdPassword => write!(f, "ClearStrongholdPassword"),
            #[cfg(feature = "stronghold")]
            Self::IsStrongholdPasswordAvailable => write!(f, "IsStrongholdPasswordAvailable"),
            #[cfg(feature = "stronghold")]
            Self::Backup {
                destination,
                password: _,
            } => write!(f, "Backup{{ destination: {destination:?} }}"),
            Self::RecoverAccounts {
                account_start_index,
                account_gap_limit,
                address_gap_limit,
                sync_options,
            } => write!(
                f,
                "RecoverAccounts{{ account_start_index: {account_start_index:?}, account_gap_limit: {account_gap_limit:?}, address_gap_limit: {address_gap_limit:?}, sync_options: {sync_options:?} }}"
            ),
            Self::RemoveLatestAccount => write!(f, "RemoveLatestAccount"),
            #[cfg(feature = "stronghold")]
            Self::RestoreBackup {
                source,
                password: _,
                ignore_if_coin_type_mismatch,
                ignore_if_bech32_mismatch,
            } => write!(
                f,
                "RestoreBackup{{ source: {source:?}, password: <ommited>, ignore_if_coin_type_mismatch: {ignore_if_coin_type_mismatch:?}, ignore_if_bech32_mismatch: {ignore_if_bech32_mismatch:?} }}"
            ),
            Self::GenerateMnemonic => write!(f, "GenerateMnemonic"),
            Self::VerifyMnemonic { mnemonic: _ } => write!(f, "VerifyMnemonic{{ mnemonic: <omitted> }}"),
            Self::SetClientOptions { client_options } => {
                write!(f, "SetClientOptions{{ client_options: {client_options:?} }}")
            }
            #[cfg(feature = "ledger_nano")]
            Self::GetLedgerNanoStatus => write!(f, "GetLedgerNanoStatus"),
            Self::GenerateEd25519Address {
                account_index,
                address_index,
                options,
                bech32_hrp,
            } => write!(
                f,
                "GenerateEd25519Address{{ account_index: {account_index:?}, address_index: {address_index:?}, options: {options:?}, bech32_hrp: {bech32_hrp:?} }}"
            ),
            Self::GetNodeInfo { url, auth: _ } => write!(f, "GetNodeInfo{{ url: {url:?} }}"),
            #[cfg(feature = "stronghold")]
            Self::SetStrongholdPassword { password: _ } => {
                write!(f, "SetStrongholdPassword{{  password: <omitted> }}")
            }
            #[cfg(feature = "stronghold")]
            Self::SetStrongholdPasswordClearInterval {
                interval_in_milliseconds,
            } => {
                write!(
                    f,
                    "SetStrongholdPasswordClearInterval{{ interval_in_milliseconds: {interval_in_milliseconds:?} }}"
                )
            }
            #[cfg(feature = "stronghold")]
            Self::StoreMnemonic { mnemonic: _ } => write!(f, "StoreMnemonic{{ mnemonic: <omitted> }}"),
            Self::StartBackgroundSync {
                options,
                interval_in_milliseconds,
            } => write!(
                f,
                "StartBackgroundSync{{ options: {options:?}, interval: {interval_in_milliseconds:?} }}"
            ),
            Self::StopBackgroundSync => write!(f, "StopBackgroundSync"),
            #[cfg(feature = "events")]
            Self::EmitTestEvent { event } => write!(f, "EmitTestEvent{{ event: {event:?} }}"),
            Self::Bech32ToHex { bech32_address } => write!(f, "Bech32ToHex{{ bech32_address: {bech32_address:?} }}"),
            Self::HexToBech32 { hex, bech32_hrp } => {
                write!(f, "HexToBech32{{ hex: {hex:?}, bech32_hrp: {bech32_hrp:?} }}")
            }
            #[cfg(feature = "events")]
            Self::ClearListeners { event_types } => write!(f, "ClearListeners{{ event_types: {event_types:?} }}"),
            Self::UpdateNodeAuth { url, auth: _ } => write!(f, "UpdateNodeAuth{{ url: {url}, auth: <omitted> }}"),
        }
    }
}
