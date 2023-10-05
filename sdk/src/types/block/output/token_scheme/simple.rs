// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};
use primitive_types::U256;

use crate::types::block::Error;

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
    ) -> Result<Self, Error> {
        let minted_tokens = minted_tokens.into();
        let melted_tokens = melted_tokens.into();
        let maximum_supply = maximum_supply.into();

        verify_supply(&minted_tokens, &melted_tokens, &maximum_supply)?;

        Ok(Self {
            minted_tokens,
            melted_tokens,
            maximum_supply,
        })
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

impl Packable for SimpleTokenScheme {
    type UnpackError = Error;
    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.minted_tokens.pack(packer)?;
        self.melted_tokens.pack(packer)?;
        self.maximum_supply.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let minted_tokens = U256::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;
        let melted_tokens = U256::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;
        let maximum_supply = U256::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;

        if VERIFY {
            verify_supply(&minted_tokens, &melted_tokens, &maximum_supply).map_err(UnpackError::Packable)?;
        }

        Ok(Self {
            minted_tokens,
            melted_tokens,
            maximum_supply,
        })
    }
}

#[inline]
fn verify_supply(minted_tokens: &U256, melted_tokens: &U256, maximum_supply: &U256) -> Result<(), Error> {
    if maximum_supply.is_zero() || melted_tokens > minted_tokens || minted_tokens - melted_tokens > *maximum_supply {
        return Err(Error::InvalidFoundryOutputSupply {
            minted: *minted_tokens,
            melted: *melted_tokens,
            max: *maximum_supply,
        });
    }

    Ok(())
}

#[cfg(feature = "serde_types")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

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
        type Error = Error;

        fn try_from(value: SimpleTokenSchemeDto) -> Result<Self, Self::Error> {
            Self::new(value.minted_tokens, value.melted_tokens, value.maximum_supply)
        }
    }

    impl_serde_typed_dto!(SimpleTokenScheme, SimpleTokenSchemeDto, "simple token scheme");
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::{
        types::block::Error,
        utils::json::{FromJson, JsonExt, ToJson, Value},
    };

    impl ToJson for SimpleTokenScheme {
        fn to_json(&self) -> Value {
            crate::json! ({
                "type": Self::KIND,
                "mintedTokens": self.minted_tokens,
                "meltedTokens": self.melted_tokens,
                "maximumSupply": self.maximum_supply,
            })
        }
    }

    impl FromJson for SimpleTokenScheme {
        type Error = Error;

        fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            if value["type"] != Self::KIND {
                return Err(Error::invalid_type::<Self>(Self::KIND, &value["type"]));
            }
            Self::new(
                value["mintedTokens"].take_value::<U256>()?,
                value["meltedTokens"].take_value::<U256>()?,
                value["maximumSupply"].take_value::<U256>()?,
            )
        }
    }
}
