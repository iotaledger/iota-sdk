// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::address::Address;

/// Identifies the validated sender of an output.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
pub struct SenderFeature(pub(crate) Address);

impl SenderFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of a [`SenderFeature`].
    pub const KIND: u8 = 0;

    /// Creates a new [`SenderFeature`].
    #[inline(always)]
    pub fn new(address: impl Into<Address>) -> Self {
        Self(address.into())
    }

    /// Returns the sender [`Address`].
    #[inline(always)]
    pub fn address(&self) -> &Address {
        &self.0
    }
}

pub(super) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct SenderFeatureDto {
        #[serde(rename = "type")]
        kind: u8,
        address: Address,
    }

    impl From<&SenderFeature> for SenderFeatureDto {
        fn from(value: &SenderFeature) -> Self {
            Self {
                kind: SenderFeature::KIND,
                address: value.0,
            }
        }
    }

    impl From<SenderFeatureDto> for SenderFeature {
        fn from(value: SenderFeatureDto) -> Self {
            Self(value.address)
        }
    }

    impl<'de> Deserialize<'de> for SenderFeature {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let dto = SenderFeatureDto::deserialize(d)?;
            if dto.kind != Self::KIND {
                return Err(serde::de::Error::custom(format!(
                    "invalid sender feature type: expected {}, found {}",
                    Self::KIND,
                    dto.kind
                )));
            }
            Ok(dto.into())
        }
    }

    impl Serialize for SenderFeature {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            SenderFeatureDto::from(self).serialize(s)
        }
    }
}
