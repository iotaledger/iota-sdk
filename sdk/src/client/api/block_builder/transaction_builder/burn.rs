// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeMap;
use std::collections::{HashMap, HashSet};

use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::types::block::output::{AccountId, DelegationId, FoundryId, NativeToken, NftId, TokenId};

/// A type to specify what needs to be burned in a transaction.
/// Nothing will be burned that has not been explicitly set with this struct.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Burn {
    // Whether initial excess mana should be burned (only from inputs/outputs that have been specified manually).
    #[serde(default)]
    pub(crate) mana: bool,
    // Whether generated mana should be burned.
    #[serde(default)]
    pub(crate) generated_mana: bool,
    /// Accounts to burn.
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub(crate) accounts: HashSet<AccountId>,
    /// Foundries to burn.
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub(crate) foundries: HashSet<FoundryId>,
    /// NFTs to burn.
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub(crate) nfts: HashSet<NftId>,
    /// Delegations to burn.
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub(crate) delegations: HashSet<DelegationId>,
    /// Amounts of native tokens to burn.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub(crate) native_tokens: BTreeMap<TokenId, U256>,
}

impl Burn {
    /// Creates a new [`Burn`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the flag to [`Burn`] initial excess mana.
    pub fn set_mana(mut self, burn_mana: bool) -> Self {
        self.mana = burn_mana;
        self
    }

    /// Returns whether to [`Burn`] mana.
    pub fn mana(&self) -> bool {
        self.mana
    }

    /// Sets the flag to [`Burn`] generated mana.
    pub fn set_generated_mana(mut self, burn_generated_mana: bool) -> Self {
        self.generated_mana = burn_generated_mana;
        self
    }

    /// Returns whether to [`Burn`] generated mana.
    pub fn generated_mana(&self) -> bool {
        self.generated_mana
    }

    /// Adds an account to [`Burn`].
    pub fn add_account(mut self, account_id: AccountId) -> Self {
        self.accounts.insert(account_id);
        self
    }

    /// Sets the accounts to [`Burn`].
    pub fn set_accounts(mut self, accounts: HashSet<AccountId>) -> Self {
        self.accounts = accounts;
        self
    }

    /// Returns the accounts to [`Burn`].
    pub fn accounts(&self) -> &HashSet<AccountId> {
        &self.accounts
    }

    /// Adds a foundry to [`Burn`].
    pub fn add_foundry(mut self, foundry_id: FoundryId) -> Self {
        self.foundries.insert(foundry_id);
        self
    }

    /// Sets the foundries to [`Burn`].
    pub fn set_foundries(mut self, foundries: HashSet<FoundryId>) -> Self {
        self.foundries = foundries;
        self
    }

    /// Returns the foundries to [`Burn`].
    pub fn foundries(&self) -> &HashSet<FoundryId> {
        &self.foundries
    }

    /// Adds an NFT to [`Burn`].
    pub fn add_nft(mut self, nft_id: NftId) -> Self {
        self.nfts.insert(nft_id);
        self
    }

    /// Sets the NFTs to [`Burn`].
    pub fn set_nfts(mut self, nfts: HashSet<NftId>) -> Self {
        self.nfts = nfts;
        self
    }

    /// Returns the NFTs to [`Burn`].
    pub fn nfts(&self) -> &HashSet<NftId> {
        &self.nfts
    }

    /// Adds an delegation to [`Burn`].
    pub fn add_delegation(mut self, delegation_id: DelegationId) -> Self {
        self.delegations.insert(delegation_id);
        self
    }

    /// Sets the delegation to [`Burn`].
    pub fn set_delegation(mut self, delegations: HashSet<DelegationId>) -> Self {
        self.delegations = delegations;
        self
    }

    /// Returns the delegation to [`Burn`].
    pub fn delegations(&self) -> &HashSet<DelegationId> {
        &self.delegations
    }

    /// Adds an amount of native token to [`Burn`].
    pub fn add_native_token(mut self, token_id: TokenId, amount: impl Into<U256>) -> Self {
        self.native_tokens.insert(token_id, amount.into());
        self
    }

    /// Sets the amounts of native tokens to [`Burn`].
    pub fn set_native_tokens(mut self, native_tokens: HashMap<TokenId, impl Into<U256>>) -> Self {
        self.native_tokens = native_tokens
            .into_iter()
            .map(|(token_id, amount)| (token_id, amount.into()))
            .collect();
        self
    }

    /// Returns the native tokens to [`Burn`].
    pub fn native_tokens(&self) -> &BTreeMap<TokenId, U256> {
        &self.native_tokens
    }
}

impl From<AccountId> for Burn {
    fn from(id: AccountId) -> Self {
        Self::new().add_account(id)
    }
}

impl From<FoundryId> for Burn {
    fn from(id: FoundryId) -> Self {
        Self::new().add_foundry(id)
    }
}

impl From<NftId> for Burn {
    fn from(id: NftId) -> Self {
        Self::new().add_nft(id)
    }
}

impl From<DelegationId> for Burn {
    fn from(id: DelegationId) -> Self {
        Self::new().add_delegation(id)
    }
}

impl From<NativeToken> for Burn {
    fn from(native_token: NativeToken) -> Self {
        Self::new().add_native_token(*native_token.token_id(), native_token.amount())
    }
}
