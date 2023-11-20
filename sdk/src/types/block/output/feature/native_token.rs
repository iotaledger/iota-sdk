// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, From};

use crate::types::block::output::{NativeToken, StorageScore};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, From, packable::Packable)]
pub struct NativeTokenFeature(NativeToken);

impl NativeTokenFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of [`NativeTokenFeature`].
    pub const KIND: u8 = 4;

    /// Creates a new [`NativeTokenFeature`].
    pub fn new(native_token: NativeToken) -> Self {
        Self(native_token)
    }

    /// Returns the inner native token.
    pub fn native_token(&self) -> &NativeToken {
        &self.0
    }
}

impl StorageScore for NativeTokenFeature {}

#[cfg(feature = "serde")]
mod dto {
    use primitive_types::U256;
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::{output::TokenId, Error};

    #[derive(Serialize, Deserialize)]
    struct NativeTokenFeatureDto {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(rename = "id")]
        token_id: TokenId,
        amount: U256,
    }

    impl From<&NativeTokenFeature> for NativeTokenFeatureDto {
        fn from(value: &NativeTokenFeature) -> Self {
            Self {
                kind: NativeTokenFeature::KIND,
                token_id: *value.token_id(),
                amount: value.amount(),
            }
        }
    }

    impl TryFrom<NativeTokenFeatureDto> for NativeTokenFeature {
        type Error = Error;

        fn try_from(value: NativeTokenFeatureDto) -> Result<Self, Error> {
            Ok(Self::new(NativeToken::new(value.token_id, value.amount)?))
        }
    }

    crate::impl_serde_typed_dto!(NativeTokenFeature, NativeTokenFeatureDto, "native token feature");
}
