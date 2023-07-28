// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::{address::Address, slot::SlotIndex};

type PublicKey = Vec<u8>;

/// Identifies the validated issuer of the UTXO state machine.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockIssuerFeature {
    /// The slot index at which the Block Issuer Feature expires and can be removed.
    expiry_slot: SlotIndex,
    /// The number of Block Issuer Keys.
    block_issuer_keys_count: u8,
    /// The Block Issuer Keys.
    block_issuer_keys: Vec<PublicKey>,
}

impl BlockIssuerFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of an [`IssuerFeature`].
    pub const KIND: u8 = 4;

    /// Creates a new [`IssuerFeature`].
    #[inline(always)]
    pub fn new(address: impl Into<Address>) -> Self {
        todo!()
        // Self(address.into())
    }

    /// Returns the Slot Index at which the Block Issuer Feature expires and can be removed.
    pub fn expiry_slot(&self) -> SlotIndex {
        self.expiry_slot
    }

    /// Returns the number of Block Issuer Keys.
    pub fn block_issuer_keys_count(&self) -> u8 {
        self.block_issuer_keys_count
    }

    /// Returns the Block Issuer Keys.
    fn block_issuer_keys(&self) -> &Vec<PublicKey> {
        &self.block_issuer_keys
    }
}

pub(crate) mod dto {
    use crate::utils::serde::string;
    use serde::{Deserialize, Serialize};

    // use crate::types::block::address::dto::AddressDto;
    use super::PublicKey;

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct BlockIssuerFeatureDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(with = "string")]
        expiry_slot: u64,
        block_issuer_keys_count: u8,
        block_issuer_keys: Vec<PublicKey>,
    }
}
