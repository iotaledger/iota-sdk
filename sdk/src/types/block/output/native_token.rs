// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};

use derive_more::{Deref, DerefMut, From};
use packable::{bounded::BoundedU8, prefix::BTreeSetPrefix, set::UnpackSetError, Packable};
use primitive_types::U256;

use crate::types::block::{output::TokenId, Error};

///
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = Error)]
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
    pub fn new(token_id: TokenId, amount: impl Into<U256>) -> Result<Self, Error> {
        let amount = amount.into();

        verify_amount::<true>(&amount, &())?;

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

impl core::borrow::Borrow<TokenId> for NativeToken {
    fn borrow(&self) -> &TokenId {
        &self.token_id
    }
}

#[inline]
fn verify_amount<const VERIFY: bool>(amount: &U256, _: &()) -> Result<(), Error> {
    if VERIFY && amount.is_zero() {
        Err(Error::NativeTokensNullAmount)
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
    pub fn add_native_token(&mut self, native_token: NativeToken) -> Result<(), Error> {
        let entry = self.0.entry(*native_token.token_id()).or_default();
        *entry = entry
            .checked_add(native_token.amount())
            .ok_or(Error::NativeTokensOverflow)?;

        Ok(())
    }

    /// Adds the given [`NativeTokens`].
    pub fn add_native_tokens(&mut self, native_tokens: NativeTokens) -> Result<(), Error> {
        for native_token in native_tokens {
            self.add_native_token(native_token)?;
        }

        Ok(())
    }

    /// Merges another [`NativeTokensBuilder`] into this one.
    pub fn merge(&mut self, other: Self) -> Result<(), Error> {
        for (token_id, amount) in other.0.into_iter() {
            self.add_native_token(NativeToken::new(token_id, amount)?)?;
        }

        Ok(())
    }

    /// Finishes the [`NativeTokensBuilder`] into [`NativeTokens`].
    pub fn finish(self) -> Result<NativeTokens, Error> {
        NativeTokens::try_from(
            self.0
                .into_iter()
                .map(|(token_id, amount)| NativeToken::new(token_id, amount))
                .collect::<Result<BTreeSet<_>, _>>()?,
        )
    }

    /// Finishes the [`NativeTokensBuilder`] into a [`Vec<NativeToken>`].
    pub fn finish_vec(self) -> Result<Vec<NativeToken>, Error> {
        self.0
            .into_iter()
            .map(|(token_id, amount)| NativeToken::new(token_id, amount))
            .collect::<Result<_, _>>()
    }

    /// Finishes the [`NativeTokensBuilder`] into a [`BTreeSet<NativeToken>`].
    pub fn finish_set(self) -> Result<BTreeSet<NativeToken>, Error> {
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

pub(crate) type NativeTokenCount = BoundedU8<0, { NativeTokens::COUNT_MAX }>;

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = Error, with = map_native_tokens_set_error)]
pub struct NativeTokens(BTreeSetPrefix<NativeToken, NativeTokenCount>);

fn map_native_tokens_set_error<T, P>(error: UnpackSetError<T, Error, P>) -> Error
where
    <NativeTokenCount as TryFrom<usize>>::Error: From<P>,
{
    match error {
        UnpackSetError::DuplicateItem(_) => Error::NativeTokensNotUniqueSorted,
        UnpackSetError::Item(e) => e,
        UnpackSetError::Prefix(p) => Error::InvalidNativeTokenCount(p.into()),
    }
}

impl TryFrom<Vec<NativeToken>> for NativeTokens {
    type Error = Error;

    #[inline(always)]
    fn try_from(native_tokens: Vec<NativeToken>) -> Result<Self, Self::Error> {
        Self::from_vec(native_tokens)
    }
}

impl TryFrom<BTreeSet<NativeToken>> for NativeTokens {
    type Error = Error;

    #[inline(always)]
    fn try_from(native_tokens: BTreeSet<NativeToken>) -> Result<Self, Self::Error> {
        Self::from_set(native_tokens)
    }
}

impl IntoIterator for NativeTokens {
    type Item = NativeToken;
    type IntoIter = alloc::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        BTreeSet::from(self.0).into_iter()
    }
}

impl NativeTokens {
    /// Maximum number of different native tokens that can be referenced in one transaction.
    pub const COUNT_MAX: u8 = 64;

    /// Creates a new [`NativeTokens`] from a vec.
    pub fn from_vec(native_tokens: Vec<NativeToken>) -> Result<Self, Error> {
        Ok(Self(
            native_tokens
                .into_iter()
                .collect::<BTreeSet<_>>()
                .try_into()
                .map_err(Error::InvalidNativeTokenCount)?,
        ))
    }

    /// Creates a new [`NativeTokens`] from an ordered set.
    pub fn from_set(native_tokens: BTreeSet<NativeToken>) -> Result<Self, Error> {
        Ok(Self(native_tokens.try_into().map_err(Error::InvalidNativeTokenCount)?))
    }

    /// Gets the underlying set.
    pub fn as_set(&self) -> &BTreeSet<NativeToken> {
        &self.0
    }

    /// Creates a new [`NativeTokensBuilder`].
    #[inline(always)]
    pub fn build() -> NativeTokensBuilder {
        NativeTokensBuilder::new()
    }
}
