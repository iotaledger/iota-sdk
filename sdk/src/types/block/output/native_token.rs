// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};

use derive_more::{Deref, DerefMut, From};
use packable::{bounded::BoundedU8, Packable};
use primitive_types::U256;

use crate::types::block::{
    output::FoundryId,
    protocol::{WorkScore, WorkScoreParameters},
    Error,
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
fn verify_amount(amount: &U256) -> Result<(), Error> {
    if amount.is_zero() {
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

    /// Merges another [`NativeTokensBuilder`] into this one.
    pub fn merge(&mut self, other: Self) -> Result<(), Error> {
        for (token_id, amount) in other.0.into_iter() {
            self.add_native_token(NativeToken::new(token_id, amount)?)?;
        }

        Ok(())
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

pub(crate) type NativeTokenCount = BoundedU8<0, 255>;
