// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeMap;
use std::collections::{HashMap, HashSet};

use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::types::block::{
    output::{AccountId, FoundryId, NativeToken, NftId, TokenId},
    Error,
};

/// A type to specify what needs to be burned during input selection.
/// Nothing will be burned that has not been explicitly set with this struct.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Burn {
    /// Aliases to burn.
    pub(crate) aliases: HashSet<AccountId>,
    /// NFTs to burn.
    pub(crate) nfts: HashSet<NftId>,
    /// Foundries to burn.
    pub(crate) foundries: HashSet<FoundryId>,
    /// Amounts of native tokens to burn.
    pub(crate) native_tokens: BTreeMap<TokenId, U256>,
}

impl Burn {
    /// Creates a new [`Burn`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an alias to [`Burn`].
    pub fn add_alias(mut self, account_id: AccountId) -> Self {
        self.aliases.insert(account_id);
        self
    }

    /// Sets the aliases to [`Burn`].
    pub fn set_aliases(mut self, aliases: HashSet<AccountId>) -> Self {
        self.aliases = aliases;
        self
    }

    /// Returns the aliases to [`Burn`].
    pub fn aliases(&self) -> &HashSet<AccountId> {
        &self.aliases
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

impl From<FoundryId> for Burn {
    fn from(id: FoundryId) -> Self {
        Self::new().add_foundry(id)
    }
}

impl From<AccountId> for Burn {
    fn from(id: AccountId) -> Self {
        Self::new().add_alias(id)
    }
}

impl From<NftId> for Burn {
    fn from(id: NftId) -> Self {
        Self::new().add_nft(id)
    }
}

impl From<NativeToken> for Burn {
    fn from(native_token: NativeToken) -> Self {
        Self::new().add_native_token(*native_token.token_id(), native_token.amount())
    }
}

/// A DTO for [`Burn`].
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BurnDto {
    /// Aliases to burn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) aliases: Option<HashSet<AccountId>>,
    /// NFTs to burn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) nfts: Option<HashSet<NftId>>,
    /// Foundries to burn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) foundries: Option<HashSet<FoundryId>>,
    /// Amounts of native tokens to burn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) native_tokens: Option<BTreeMap<TokenId, U256>>,
}

impl From<&Burn> for BurnDto {
    fn from(value: &Burn) -> Self {
        Self {
            aliases: (!value.aliases.is_empty()).then_some(value.aliases.clone()),
            nfts: (!value.nfts.is_empty()).then_some(value.nfts.clone()),
            foundries: (!value.foundries.is_empty()).then_some(value.foundries.clone()),
            native_tokens: (!value.native_tokens.is_empty()).then_some(BTreeMap::from_iter(
                value
                    .native_tokens
                    .iter()
                    .map(|(token_id, amount)| (*token_id, *amount)),
            )),
        }
    }
}

impl TryFrom<BurnDto> for Burn {
    type Error = Error;

    fn try_from(value: BurnDto) -> Result<Self, Self::Error> {
        Ok(Self {
            aliases: value.aliases.unwrap_or_default(),
            nfts: value.nfts.unwrap_or_default(),
            foundries: value.foundries.unwrap_or_default(),
            native_tokens: value.native_tokens.unwrap_or_default(),
        })
    }
}
