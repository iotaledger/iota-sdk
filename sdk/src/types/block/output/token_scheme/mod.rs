// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod simple;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

pub use self::simple::SimpleTokenScheme;
use crate::types::block::Error;

///
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, derive_more::From, packable::Packable)]
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

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for TokenScheme {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct TypedTokenScheme {
            #[serde(rename = "type")]
            kind: u8,
            data: serde_json::Value,
        }

        let value = TypedTokenScheme::deserialize(d)?;
        Ok(match value.kind {
            SimpleTokenScheme::KIND => SimpleTokenScheme::deserialize(value.data)
                .map_err(|e| serde::de::Error::custom(alloc::format!("cannot deserialize simple token scheme: {e}")))?
                .into(),
            _ => {
                return Err(serde::de::Error::custom(alloc::format!(
                    "invalid token scheme type: {}",
                    value.kind
                )));
            }
        })
    }
}

#[cfg(feature = "serde")]
impl Serialize for TokenScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum TokenSchemeDto<'a> {
            T1(&'a SimpleTokenScheme),
        }
        #[derive(Serialize)]
        struct TypedTokenScheme<'a> {
            #[serde(rename = "type")]
            kind: u8,
            data: TokenSchemeDto<'a>,
        }
        let data = match self {
            Self::Simple(data) => TokenSchemeDto::T1(data),
        };
        TypedTokenScheme {
            kind: self.kind(),
            data,
        }
        .serialize(serializer)
    }
}
