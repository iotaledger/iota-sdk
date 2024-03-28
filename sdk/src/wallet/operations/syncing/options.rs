// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

const DEFAULT_FORCE_SYNCING: bool = false;
const DEFAULT_SYNC_INCOMING_TRANSACTIONS: bool = false;
const DEFAULT_SYNC_ONLY_MOST_BASIC_OUTPUTS: bool = false;
const DEFAULT_SYNC_PENDING_TRANSACTIONS: bool = true;
const DEFAULT_SYNC_NATIVE_TOKEN_FOUNDRIES: bool = false;
const DEFAULT_SYNC_IMPLICIT_ACCOUNTS: bool = false;

/// The synchronization options
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncOptions {
    /// Syncing is usually skipped if it's called repeatedly in a short amount of time as there can only be new changes
    /// every slot and calling it twice "at the same time" will not return new data.
    /// When this to true, we sync anyways, even if it's called 0ms after the last sync finished.
    #[serde(default)]
    pub force_syncing: bool,
    /// Try to sync transactions from incoming outputs with their inputs. Some data may not be obtained if it has been
    /// pruned.
    #[serde(default = "default_sync_incoming_transactions")]
    pub sync_incoming_transactions: bool,
    /// Checks pending transactions and reissues them if necessary.
    #[serde(default = "default_sync_pending_transactions")]
    pub sync_pending_transactions: bool,
    /// Specifies what outputs should be synced for the ed25519 address from the wallet.
    #[serde(default)]
    pub wallet: WalletSyncOptions,
    /// Specifies what outputs should be synced for the address of an account output.
    #[serde(default)]
    pub account: AccountSyncOptions,
    /// Specifies what outputs should be synced for the address of an nft output.
    #[serde(default)]
    pub nft: NftSyncOptions,
    /// Specifies if only basic outputs with an AddressUnlockCondition alone should be synced, will overwrite
    /// `wallet`, `account` and `nft` options.
    #[serde(default = "default_sync_only_most_basic_outputs")]
    pub sync_only_most_basic_outputs: bool,
    /// Sync native token foundries, so their metadata can be returned in the balance.
    #[serde(default = "default_sync_native_token_foundries")]
    pub sync_native_token_foundries: bool,
    /// Sync implicit accounts.
    #[serde(default = "default_sync_implicit_accounts")]
    pub sync_implicit_accounts: bool,
}

fn default_force_syncing() -> bool {
    DEFAULT_FORCE_SYNCING
}

fn default_sync_incoming_transactions() -> bool {
    DEFAULT_SYNC_INCOMING_TRANSACTIONS
}

fn default_sync_only_most_basic_outputs() -> bool {
    DEFAULT_SYNC_ONLY_MOST_BASIC_OUTPUTS
}

fn default_sync_pending_transactions() -> bool {
    DEFAULT_SYNC_PENDING_TRANSACTIONS
}

fn default_sync_native_token_foundries() -> bool {
    DEFAULT_SYNC_NATIVE_TOKEN_FOUNDRIES
}

fn default_sync_implicit_accounts() -> bool {
    DEFAULT_SYNC_IMPLICIT_ACCOUNTS
}

impl Default for SyncOptions {
    fn default() -> Self {
        Self {
            sync_incoming_transactions: default_sync_incoming_transactions(),
            sync_pending_transactions: default_sync_pending_transactions(),
            wallet: WalletSyncOptions::default(),
            account: AccountSyncOptions::default(),
            nft: NftSyncOptions::default(),
            sync_only_most_basic_outputs: default_sync_only_most_basic_outputs(),
            sync_native_token_foundries: default_sync_native_token_foundries(),
            force_syncing: default_force_syncing(),
            sync_implicit_accounts: default_sync_implicit_accounts(),
        }
    }
}

/// Sync options for Ed25519 addresses from the wallet
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct WalletSyncOptions {
    pub basic_outputs: bool,
    pub account_outputs: bool,
    pub nft_outputs: bool,
    pub delegation_outputs: bool,
}

impl Default for WalletSyncOptions {
    fn default() -> Self {
        Self {
            basic_outputs: true,
            account_outputs: true,
            nft_outputs: true,
            delegation_outputs: true,
        }
    }
}

impl WalletSyncOptions {
    pub(crate) fn all_outputs(&self) -> bool {
        self.basic_outputs && self.account_outputs && self.nft_outputs && self.delegation_outputs
    }
}

/// Sync options for addresses from account outputs
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AccountSyncOptions {
    pub basic_outputs: bool,
    pub account_outputs: bool,
    pub foundry_outputs: bool,
    pub nft_outputs: bool,
    pub delegation_outputs: bool,
}

impl Default for AccountSyncOptions {
    // Sync only foundries
    fn default() -> Self {
        Self {
            basic_outputs: false,
            account_outputs: false,
            foundry_outputs: true,
            nft_outputs: false,
            delegation_outputs: false,
        }
    }
}

impl AccountSyncOptions {
    pub(crate) fn all_outputs(&self) -> bool {
        self.basic_outputs
            && self.account_outputs
            && self.foundry_outputs
            && self.nft_outputs
            && self.delegation_outputs
    }
}

/// Sync options for addresses from NFT outputs
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct NftSyncOptions {
    pub basic_outputs: bool,
    pub account_outputs: bool,
    pub nft_outputs: bool,
    pub delegation_outputs: bool,
}

impl NftSyncOptions {
    pub(crate) fn all_outputs(&self) -> bool {
        self.basic_outputs && self.account_outputs && self.nft_outputs && self.delegation_outputs
    }
}
