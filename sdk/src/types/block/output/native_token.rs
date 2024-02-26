// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::Packable;
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
