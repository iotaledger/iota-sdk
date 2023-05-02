// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod simple;

pub use self::simple::SimpleTokenScheme;
use crate::types::block::Error;

///
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, derive_more::From, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidTokenSchemeKind)]
pub enum TokenScheme {
    ///
    #[packable(tag = SimpleTokenScheme::KIND)]
    Simple(SimpleTokenScheme),
}

impl core::fmt::Debug for TokenScheme {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Simple(scheme) => scheme.fmt(f),
        }
    }
}

impl TokenScheme {
    /// Returns the token scheme kind of a [`TokenScheme`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Simple(_) => SimpleTokenScheme::KIND,
        }
    }
}

#[allow(missing_docs)]
pub mod dto {
    use derive_more::From;
    use serde::{Deserialize, Serialize};

    pub use super::simple::dto::SimpleTokenSchemeDto;
    use super::*;
    use crate::types::block::Error;

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub enum TokenSchemeDto {
        /// A simple token scheme.
        Simple(SimpleTokenSchemeDto),
    }

    impl From<&TokenScheme> for TokenSchemeDto {
        fn from(value: &TokenScheme) -> Self {
            match value {
                TokenScheme::Simple(v) => Self::Simple(v.into()),
            }
        }
    }

    impl TryFrom<&TokenSchemeDto> for TokenScheme {
        type Error = Error;

        fn try_from(value: &TokenSchemeDto) -> Result<Self, Self::Error> {
            Ok(match value {
                TokenSchemeDto::Simple(v) => Self::Simple(v.try_into()?),
            })
        }
    }
}
