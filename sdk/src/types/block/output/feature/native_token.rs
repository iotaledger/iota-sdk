// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, From};

use crate::types::block::output::NativeToken;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, From, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
