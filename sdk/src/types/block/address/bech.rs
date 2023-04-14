// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::types::block::address::Address;

/// An address and its network type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Bech32 {
    pub(crate) inner: Address,
    #[serde(rename = "bech32Hrp")]
    pub(crate) bech32_hrp: String,
}

impl AsRef<Address> for Bech32 {
    fn as_ref(&self) -> &Address {
        &self.inner
    }
}

impl Bech32 {
    /// Create a new address wrapper.
    pub fn new(address: Address, bech32_hrp: String) -> Self {
        Self {
            inner: address,
            bech32_hrp,
        }
    }

    /// Encodes the address as bech32.
    pub fn to_bech32(&self) -> String {
        self.inner.to_bech32(&self.bech32_hrp)
    }

    /// Get the bech32 human readable part
    pub fn bech32_hrp(&self) -> &str {
        &self.bech32_hrp
    }

    /// Parses a bech32 address string.
    pub fn try_from_bech32<A: AsRef<str>>(address: A) -> crate::wallet::Result<Self> {
        let (address, bech32_hrp) = Address::try_from_bech32_with_hrp(address)?;

        Ok(Self::new(address, bech32_hrp))
    }
}
