// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};
use core::convert::Infallible;

use derive_more::{Deref, DerefMut, From};
use iterator_sorted::is_unique_sorted;
use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix, Packable};
use primitive_types::U256;

use crate::types::block::{
    output::FoundryId,
    protocol::{WorkScore, WorkScoreParameters},
};

crate::impl_id!(
    /// Unique identifier of a [`NativeToken`](crate::types::block::output::NativeToken).
    /// The TokenId of native tokens minted by a specific foundry is the same as the
    /// [`FoundryId`](crate::types::block::output::FoundryId).
    pub TokenId {
        pub const LENGTH: usize = 38;
    }
);

impl From<FoundryId> for TokenId {
    fn from(foundry_id: FoundryId) -> Self {
        Self::new(*foundry_id)
    }
}

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
#[allow(missing_docs)]
pub enum NativeTokenError {
    #[display(fmt = "invalid native token count: {_0}")]
    Count(<NativeTokenCount as TryFrom<usize>>::Error),
    #[display(fmt = "native tokens are not unique and/or sorted")]
    NotUniqueSorted,
    #[display(fmt = "native tokens null amount")]
    NullAmount,
    #[display(fmt = "native tokens overflow")]
    Overflow,
    #[display(fmt = "consumed native tokens amount overflow")]
    ConsumedAmountOverflow,
    #[display(fmt = "created native tokens amount overflow")]
    CreatedAmountOverflow,
}

#[cfg(feature = "std")]
impl std::error::Error for NativeTokenError {}

impl From<Infallible> for NativeTokenError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}

///
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = NativeTokenError)]
pub struct NativeToken {
    // Identifier of the native token.
    #[cfg_attr(feature = "serde", serde(rename = "id"))]
    token_id: TokenId,
    // Amount of native tokens.
    #[packable(verify_with = verify_amount)]
    amount: U256,
}

impl NativeToken {
    /// Creates a new [`NativeToken`].
    #[inline(always)]
    pub fn new(token_id: TokenId, amount: impl Into<U256>) -> Result<Self, NativeTokenError> {
        let amount = amount.into();

        verify_amount(&amount)?;

        Ok(Self { token_id, amount })
    }

    /// Returns the token ID of the [`NativeToken`].
    #[inline(always)]
    pub fn token_id(&self) -> &TokenId {
        &self.token_id
    }

    /// Returns the amount of the [`NativeToken`].
    #[inline(always)]
    pub fn amount(&self) -> U256 {
        self.amount
    }
}

impl WorkScore for NativeToken {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.native_token()
    }
}

impl PartialOrd for NativeToken {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for NativeToken {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.token_id.cmp(&other.token_id)
    }
}

#[inline]
fn verify_amount(amount: &U256) -> Result<(), NativeTokenError> {
    if amount.is_zero() {
        Err(NativeTokenError::NullAmount)
    } else {
        Ok(())
    }
}

/// A builder for [`NativeTokens`].
#[derive(Clone, Default, Debug, Deref, DerefMut, From)]
#[must_use]
pub struct NativeTokensBuilder(BTreeMap<TokenId, U256>);

impl NativeTokensBuilder {
    /// Creates a new [`NativeTokensBuilder`].
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds the given [`NativeToken`].
    pub fn add_native_token(&mut self, native_token: NativeToken) -> Result<(), NativeTokenError> {
        let entry = self.0.entry(*native_token.token_id()).or_default();
        *entry = entry
            .checked_add(native_token.amount())
            .ok_or(NativeTokenError::Overflow)?;

        Ok(())
    }

    /// Adds the given [`NativeTokens`].
    pub fn add_native_tokens(&mut self, native_tokens: NativeTokens) -> Result<(), NativeTokenError> {
        for native_token in native_tokens {
            self.add_native_token(native_token)?;
        }

        Ok(())
    }

    /// Merges another [`NativeTokensBuilder`] into this one.
    pub fn merge(&mut self, other: Self) -> Result<(), NativeTokenError> {
        for (token_id, amount) in other.0.into_iter() {
            self.add_native_token(NativeToken::new(token_id, amount)?)?;
        }

        Ok(())
    }

    /// Finishes the [`NativeTokensBuilder`] into [`NativeTokens`].
    pub fn finish(self) -> Result<NativeTokens, NativeTokenError> {
        NativeTokens::try_from(
            self.0
                .into_iter()
                .map(|(token_id, amount)| NativeToken::new(token_id, amount))
                .collect::<Result<BTreeSet<_>, _>>()?,
        )
    }

    /// Finishes the [`NativeTokensBuilder`] into a [`Vec<NativeToken>`].
    pub fn finish_vec(self) -> Result<Vec<NativeToken>, NativeTokenError> {
        self.0
            .into_iter()
            .map(|(token_id, amount)| NativeToken::new(token_id, amount))
            .collect::<Result<_, _>>()
    }

    /// Finishes the [`NativeTokensBuilder`] into a [`BTreeSet<NativeToken>`].
    pub fn finish_set(self) -> Result<BTreeSet<NativeToken>, NativeTokenError> {
        self.0
            .into_iter()
            .map(|(token_id, amount)| NativeToken::new(token_id, amount))
            .collect::<Result<_, _>>()
    }
}

impl From<NativeTokens> for NativeTokensBuilder {
    fn from(native_tokens: NativeTokens) -> Self {
        let mut builder = Self::new();
        for native_token in native_tokens {
            *builder.0.entry(*native_token.token_id()).or_default() += native_token.amount();
        }
        builder
    }
}

pub(crate) type NativeTokenCount = BoundedU8<0, 255>;

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = NativeTokenError, with = |e| e.unwrap_item_err_or_else(|p| NativeTokenError::Count(p.into())))]
pub struct NativeTokens(
    #[packable(verify_with = verify_unique_sorted)] BoxedSlicePrefix<NativeToken, NativeTokenCount>,
);

impl TryFrom<Vec<NativeToken>> for NativeTokens {
    type Error = NativeTokenError;

    #[inline(always)]
    fn try_from(native_tokens: Vec<NativeToken>) -> Result<Self, Self::Error> {
        Self::from_vec(native_tokens)
    }
}

impl TryFrom<BTreeSet<NativeToken>> for NativeTokens {
    type Error = NativeTokenError;

    #[inline(always)]
    fn try_from(native_tokens: BTreeSet<NativeToken>) -> Result<Self, Self::Error> {
        Self::from_set(native_tokens)
    }
}

impl IntoIterator for NativeTokens {
    type Item = NativeToken;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(Into::<Box<[NativeToken]>>::into(self.0)).into_iter()
    }
}

impl NativeTokens {
    /// Creates a new [`NativeTokens`] from a vec.
    pub fn from_vec(native_tokens: Vec<NativeToken>) -> Result<Self, NativeTokenError> {
        let mut native_tokens =
            BoxedSlicePrefix::<NativeToken, NativeTokenCount>::try_from(native_tokens.into_boxed_slice())
                .map_err(NativeTokenError::Count)?;

        native_tokens.sort_by(|a, b| a.token_id().cmp(b.token_id()));
        // Sort is obviously fine now but uniqueness still needs to be checked.
        verify_unique_sorted(&native_tokens)?;

        Ok(Self(native_tokens))
    }

    /// Creates a new [`NativeTokens`] from an ordered set.
    pub fn from_set(native_tokens: BTreeSet<NativeToken>) -> Result<Self, NativeTokenError> {
        Ok(Self(
            native_tokens
                .into_iter()
                .collect::<Box<[_]>>()
                .try_into()
                .map_err(NativeTokenError::Count)?,
        ))
    }

    /// Creates a new [`NativeTokensBuilder`].
    #[inline(always)]
    pub fn build() -> NativeTokensBuilder {
        NativeTokensBuilder::new()
    }

    /// Checks whether the provided token ID is contained in the native tokens.
    pub fn contains(&self, token_id: &TokenId) -> bool {
        // Binary search is possible because native tokens are always ordered by token ID.
        self.0
            .binary_search_by_key(token_id, |native_token| native_token.token_id)
            .is_ok()
    }

    /// Gets the native token associated with the provided token ID if contained.
    pub fn get(&self, token_id: &TokenId) -> Option<&NativeToken> {
        // Binary search is possible because native tokens are always ordered by token ID.
        self.0
            .binary_search_by_key(token_id, |native_token| native_token.token_id)
            .map_or(None, |index| Some(&self.0[index]))
    }
}

#[inline]
fn verify_unique_sorted(native_tokens: &[NativeToken]) -> Result<(), NativeTokenError> {
    if !is_unique_sorted(native_tokens.iter().map(NativeToken::token_id)) {
        Err(NativeTokenError::NotUniqueSorted)
    } else {
        Ok(())
    }
}
