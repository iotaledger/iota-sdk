// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix};

use crate::types::block::{public_key::PublicKey, slot::SlotIndex, Error};

const KEY_COUNT_MIN: u8 = 1;
const KEY_COUNT_MAX: u8 = 128;

pub(crate) type PublicKeyCount = BoundedU8<KEY_COUNT_MIN, KEY_COUNT_MAX>;

/// This feature defines the public keys with which a signature to burn Mana from
/// the containing account's Block Issuance Credit can be verified.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
#[packable(unpack_error = Error)]
pub struct BlockIssuerFeature {
    /// The slot index at which the Block Issuer Feature expires and can be removed.
    expiry_slot: SlotIndex,
    /// The Block Issuer Keys.
    #[packable(unpack_error_with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidPublicKeyCount(p.into())))]
    keys: BoxedSlicePrefix<PublicKey, PublicKeyCount>,
}

impl BlockIssuerFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of an [`BlockIssuerFeature`].
    pub const KIND: u8 = 4;

    /// Creates a new [`BlockIssuerFeature`].
    #[inline(always)]
    pub fn new(expiry_slot: impl Into<SlotIndex>, keys: impl Into<Box<[PublicKey]>>) -> Result<Self, Error> {
        let keys: Box<[PublicKey]> = keys.into();

        Ok(Self {
            expiry_slot: expiry_slot.into(),
            keys: keys.try_into().map_err(Error::InvalidPublicKeyCount)?,
        })
    }

    /// Returns the Slot Index at which the Block Issuer Feature expires and can be removed.
    pub fn expiry_slot(&self) -> SlotIndex {
        self.expiry_slot
    }

    /// Returns the Block Issuer Keys.
    pub fn keys(&self) -> &[PublicKey] {
        &self.keys
    }
}

pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::BlockIssuerFeature;
    use crate::{
        types::block::{
            public_key::{dto::PublicKeyDto, PublicKey},
            Error,
        },
        utils::serde::string,
    };

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct BlockIssuerFeatureDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(with = "string")]
        pub expiry_slot: u64,
        #[serde(skip_serializing_if = "alloc::vec::Vec::is_empty", default)]
        pub keys: alloc::vec::Vec<PublicKeyDto>,
    }

    impl From<&BlockIssuerFeature> for BlockIssuerFeatureDto {
        fn from(value: &BlockIssuerFeature) -> Self {
            Self {
                kind: BlockIssuerFeature::KIND,
                expiry_slot: value.expiry_slot.into(),
                keys: value.keys.iter().map(|key| key.into()).collect(),
            }
        }
    }

    impl TryFrom<BlockIssuerFeatureDto> for BlockIssuerFeature {
        type Error = Error;

        fn try_from(value: BlockIssuerFeatureDto) -> Result<Self, Self::Error> {
            let keys = value
                .keys
                .into_iter()
                .map(PublicKey::try_from)
                .collect::<Result<alloc::vec::Vec<PublicKey>, Error>>()?;

            Self::new(value.expiry_slot, keys)
        }
    }

    impl_serde_typed_dto!(BlockIssuerFeature, BlockIssuerFeatureDto, "block issuer feature");
}
