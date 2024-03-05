// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::Packable;
use primitive_types::U256;

use crate::types::block::{
    output::token_scheme::TokenSchemeError,
    protocol::{WorkScore, WorkScoreParameters},
};

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Packable)]
#[packable(unpack_error = TokenSchemeError)]
#[packable(verify_with = verify_simple_token_scheme)]
pub struct SimpleTokenScheme {
    // Amount of tokens minted by a foundry.
    minted_tokens: U256,
    // Amount of tokens melted by a foundry.
    melted_tokens: U256,
    // Maximum supply of tokens controlled by a foundry.
    maximum_supply: U256,
}

impl SimpleTokenScheme {
    /// The [`TokenScheme`](crate::types::block::output::TokenScheme) kind of a [`SimpleTokenScheme`].
    pub const KIND: u8 = 0;

    /// Creates a new [`SimpleTokenScheme`].
    #[inline(always)]
    pub fn new(
        minted_tokens: impl Into<U256>,
        melted_tokens: impl Into<U256>,
        maximum_supply: impl Into<U256>,
    ) -> Result<Self, TokenSchemeError> {
        let minted_tokens = minted_tokens.into();
        let melted_tokens = melted_tokens.into();
        let maximum_supply = maximum_supply.into();

        let token_scheme = Self {
            minted_tokens,
            melted_tokens,
            maximum_supply,
        };

        verify_simple_token_scheme(&token_scheme)?;

        Ok(token_scheme)
    }

    /// Returns the number of minted tokens of the [`SimpleTokenScheme`].
    #[inline(always)]
    pub fn minted_tokens(&self) -> U256 {
        self.minted_tokens
    }

    /// Returns the number of melted tokens of the [`SimpleTokenScheme`].
    #[inline(always)]
    pub fn melted_tokens(&self) -> U256 {
        self.melted_tokens
    }

    /// Returns the maximum supply of the [`SimpleTokenScheme`].
    #[inline(always)]
    pub fn maximum_supply(&self) -> U256 {
        self.maximum_supply
    }

    /// Returns the circulating supply of the [`SimpleTokenScheme`].
    #[inline(always)]
    pub fn circulating_supply(&self) -> U256 {
        self.minted_tokens - self.melted_tokens
    }
}

impl WorkScore for SimpleTokenScheme {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.native_token()
    }
}

#[inline]
fn verify_simple_token_scheme(token_scheme: &SimpleTokenScheme) -> Result<(), TokenSchemeError> {
    if token_scheme.maximum_supply.is_zero()
        || token_scheme.melted_tokens > token_scheme.minted_tokens
        || token_scheme.minted_tokens - token_scheme.melted_tokens > token_scheme.maximum_supply
    {
        return Err(TokenSchemeError::Supply {
            minted: token_scheme.minted_tokens,
            melted: token_scheme.melted_tokens,
            max: token_scheme.maximum_supply,
        });
    }

    Ok(())
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    /// Describes a foundry output that is controlled by an account.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SimpleTokenSchemeDto {
        #[serde(rename = "type")]
        kind: u8,
        // Amount of tokens minted by a foundry.
        minted_tokens: U256,
        // Amount of tokens melted by a foundry.
        melted_tokens: U256,
        // Maximum supply of tokens controlled by a foundry.
        maximum_supply: U256,
    }

    impl From<&SimpleTokenScheme> for SimpleTokenSchemeDto {
        fn from(value: &SimpleTokenScheme) -> Self {
            Self {
                kind: SimpleTokenScheme::KIND,
                minted_tokens: (&value.minted_tokens()).into(),
                melted_tokens: (&value.melted_tokens()).into(),
                maximum_supply: (&value.maximum_supply()).into(),
            }
        }
    }

    impl TryFrom<SimpleTokenSchemeDto> for SimpleTokenScheme {
        type Error = TokenSchemeError;

        fn try_from(value: SimpleTokenSchemeDto) -> Result<Self, Self::Error> {
            Self::new(value.minted_tokens, value.melted_tokens, value.maximum_supply)
        }
    }

    crate::impl_serde_typed_dto!(SimpleTokenScheme, SimpleTokenSchemeDto, "simple token scheme");
}
