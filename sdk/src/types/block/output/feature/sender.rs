// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::{
    address::Address,
    output::{storage_score::StorageScoreParameters, StorageScore},
    protocol::WorkScore,
};

/// Identifies the validated sender of an output.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
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

impl StorageScore for SenderFeature {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        self.address().storage_score(params)
    }
}

impl WorkScore for SenderFeature {}

#[cfg(feature = "serde")]
mod dto {
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
                address: value.0.clone(),
            }
        }
    }

    impl From<SenderFeatureDto> for SenderFeature {
        fn from(value: SenderFeatureDto) -> Self {
            Self(value.address)
        }
    }

    crate::impl_serde_typed_dto!(SenderFeature, SenderFeatureDto, "sender feature");
}
