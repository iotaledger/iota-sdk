// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// The synchronization options
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncOptions {
    /// Syncing is usually skipped if it's called repeatedly in a short amount of time as there can only be new changes
    /// every slot and calling it twice "at the same time" will not return new data.
    /// When this to true, we sync anyways, even if it's called 0ms after the last sync finished.
    #[serde(default = "no")]
    pub force_syncing: bool,
    /// Try to sync transactions from incoming outputs with their inputs. Some data may not be obtained if it has been
    /// pruned.
    #[serde(default = "no")]
    pub sync_incoming_transactions: bool,
    /// Checks pending transactions and reissues them if necessary.
    #[serde(default = "yes")]
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
    #[serde(default = "no")]
    pub sync_only_most_basic_outputs: bool,
    /// Sync native token foundries, so their metadata can be returned in the balance.
    #[serde(default = "no")]
    pub sync_native_token_foundries: bool,
    /// Sync implicit accounts.
    #[serde(default = "no")]
    pub sync_implicit_accounts: bool,
}

impl Default for SyncOptions {
    fn default() -> Self {
        Self {
            force_syncing: no(),
            sync_incoming_transactions: no(),
            sync_pending_transactions: yes(),
            wallet: WalletSyncOptions::default(),
            account: AccountSyncOptions::default(),
            nft: NftSyncOptions::default(),
            sync_only_most_basic_outputs: no(),
            sync_native_token_foundries: no(),
            sync_implicit_accounts: no(),
        }
    }
}

/// Sync options for Ed25519 addresses from the wallet
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct WalletSyncOptions {
    #[serde(default = "yes")]
    pub basic_outputs: bool,
    #[serde(default = "yes")]
    pub account_outputs: bool,
    #[serde(default = "yes")]
    pub nft_outputs: bool,
    #[serde(default = "yes")]
    pub delegation_outputs: bool,
}

impl Default for WalletSyncOptions {
    fn default() -> Self {
        Self {
            basic_outputs: yes(),
            account_outputs: yes(),
            nft_outputs: yes(),
            delegation_outputs: yes(),
        }
    }
}

impl WalletSyncOptions {
    pub(crate) fn all_outputs(&self) -> bool {
        self.basic_outputs && self.account_outputs && self.nft_outputs && self.delegation_outputs
    }
}

/// Sync options for addresses from account outputs
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AccountSyncOptions {
    #[serde(default = "no")]
    pub basic_outputs: bool,
    #[serde(default = "no")]
    pub account_outputs: bool,
    #[serde(default = "yes")]
    pub foundry_outputs: bool,
    #[serde(default = "no")]
    pub nft_outputs: bool,
    #[serde(default = "no")]
    pub delegation_outputs: bool,
}

impl Default for AccountSyncOptions {
    // Sync only foundries
    fn default() -> Self {
        Self {
            basic_outputs: no(),
            account_outputs: no(),
            foundry_outputs: yes(),
            nft_outputs: no(),
            delegation_outputs: no(),
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct NftSyncOptions {
    #[serde(default = "no")]
    pub basic_outputs: bool,
    #[serde(default = "no")]
    pub account_outputs: bool,
    #[serde(default = "no")]
    pub nft_outputs: bool,
    #[serde(default = "no")]
    pub delegation_outputs: bool,
}

impl Default for NftSyncOptions {
    fn default() -> Self {
        Self {
            basic_outputs: no(),
            account_outputs: no(),
            nft_outputs: no(),
            delegation_outputs: no(),
        }
    }
}
impl NftSyncOptions {
    pub(crate) fn all_outputs(&self) -> bool {
        self.basic_outputs && self.account_outputs && self.nft_outputs && self.delegation_outputs
    }
}

const fn yes() -> bool {
    true
}

const fn no() -> bool {
    false
}
