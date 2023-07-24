// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::address::Address;

/// Identifies the validated issuer of the UTXO state machine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
pub struct IssuerFeature(pub(crate) Address);

impl IssuerFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of an [`IssuerFeature`].
    pub const KIND: u8 = 1;

    /// Creates a new [`IssuerFeature`].
    #[inline(always)]
    pub fn new(address: impl Into<Address>) -> Self {
        Self(address.into())
    }

    /// Returns the issuer [`Address`].
    #[inline(always)]
    pub fn address(&self) -> &Address {
        &self.0
    }
}

mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct IssuerFeatureDto {
        #[serde(rename = "type")]
        kind: u8,
        address: Address,
    }

    impl From<&IssuerFeature> for IssuerFeatureDto {
        fn from(value: &IssuerFeature) -> Self {
            Self {
                kind: IssuerFeature::KIND,
                address: value.0,
            }
        }
    }

    impl From<IssuerFeatureDto> for IssuerFeature {
        fn from(value: IssuerFeatureDto) -> Self {
            Self(value.address)
        }
    }

    impl<'de> Deserialize<'de> for IssuerFeature {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let dto = IssuerFeatureDto::deserialize(d)?;
            if dto.kind != Self::KIND {
                return Err(serde::de::Error::custom(format!(
                    "invalid issuer feature type: expected {}, found {}",
                    Self::KIND,
                    dto.kind
                )));
            }
            Ok(dto.into())
        }
    }

    impl Serialize for IssuerFeature {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            IssuerFeatureDto::from(self).serialize(s)
        }
    }
}
