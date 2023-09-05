// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use crate::types::block::{
    output::{rent::RentBuilder, Rent},
    public_key::{PublicKey, PublicKeys},
    slot::SlotIndex,
    Error,
};

/// This feature defines the public keys with which a signature from the containing
/// account's Block Issuance Credit can be verified in order to burn Mana.
#[derive(Clone, Debug, Eq, PartialEq, Hash, packable::Packable)]
#[packable(unpack_error = Error)]
pub struct BlockIssuerFeature {
    /// The slot index at which the Block Issuer Feature expires and can be removed.
    expiry_slot: SlotIndex,
    /// The Block Issuer Keys.
    public_keys: PublicKeys,
}

impl BlockIssuerFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of a [`BlockIssuerFeature`].
    pub const KIND: u8 = 4;

    /// Creates a new [`BlockIssuerFeature`].
    #[inline(always)]
    pub fn new(
        expiry_slot: impl Into<SlotIndex>,
        public_keys: impl IntoIterator<Item = PublicKey>,
    ) -> Result<Self, Error> {
        let public_keys = PublicKeys::from_vec(public_keys.into_iter().collect::<Vec<PublicKey>>())?;
        Ok(Self {
            expiry_slot: expiry_slot.into(),
            public_keys,
        })
    }

    /// Returns the Slot Index at which the Block Issuer Feature expires and can be removed.
    pub fn expiry_slot(&self) -> SlotIndex {
        self.expiry_slot
    }

    /// Returns the Block Issuer Keys.
    pub fn public_keys(&self) -> &[PublicKey] {
        &self.public_keys
    }
}

impl Rent for BlockIssuerFeature {
    fn build_weighted_bytes(&self, builder: RentBuilder) -> RentBuilder {
        builder
            // Feature Type
            .data_field::<u8>()
            // Expiry Slot
            .data_field::<SlotIndex>()
            // Public Keys
            .packable_block_issuer_key_field(&self.public_keys)
    }
}

#[cfg(feature = "serde")]
mod dto {
    use alloc::vec::Vec;

    use serde::{Deserialize, Serialize};

    use super::BlockIssuerFeature;
    use crate::types::block::{
        public_key::{dto::PublicKeyDto, PublicKey},
        slot::SlotIndex,
        Error,
    };

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct BlockIssuerFeatureDto {
        #[serde(rename = "type")]
        kind: u8,
        expiry_slot: SlotIndex,
        keys: Vec<PublicKeyDto>,
    }

    impl From<&BlockIssuerFeature> for BlockIssuerFeatureDto {
        fn from(value: &BlockIssuerFeature) -> Self {
            Self {
                kind: BlockIssuerFeature::KIND,
                expiry_slot: value.expiry_slot,
                keys: value.public_keys.iter().map(|key| key.into()).collect(),
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
                .collect::<Result<Vec<PublicKey>, Error>>()?;

            Self::new(value.expiry_slot, keys)
        }
    }

    impl_serde_typed_dto!(BlockIssuerFeature, BlockIssuerFeatureDto, "block issuer feature");
}
