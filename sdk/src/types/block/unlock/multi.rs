// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;

use derive_more::Deref;
use packable::{prefix::BoxedSlicePrefix, Packable};

use crate::types::block::{address::WeightedAddressCount, unlock::Unlock, Error};

pub(crate) type UnlocksCount = WeightedAddressCount;

/// Unlocks a [`MultiAddress`](crate::types::block::address::MultiAddress) with a list of other unlocks.
#[derive(Clone, Debug, Deref, Eq, PartialEq, Hash, Packable)]
#[packable(unpack_error = Error, with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidMultiUnlockCount(p.into())))]
pub struct MultiUnlock(#[packable(verify_with = verify_unlocks)] BoxedSlicePrefix<Unlock, UnlocksCount>);

impl MultiUnlock {
    /// The [`Unlock`](crate::types::block::unlock::Unlock) kind of an [`MultiUnlock`].
    pub const KIND: u8 = 5;

    /// Creates a new [`MultiUnlock`].
    #[inline(always)]
    pub fn new(unlocks: impl IntoIterator<Item = Unlock>) -> Result<Self, Error> {
        let unlocks = unlocks.into_iter().collect::<Box<[_]>>();

        verify_unlocks::<true>(&unlocks, &())?;

        Ok(Self(
            BoxedSlicePrefix::<Unlock, UnlocksCount>::try_from(unlocks).map_err(Error::InvalidMultiUnlockCount)?,
        ))
    }

    /// Return the inner unlocks of an [`MultiUnlock`].
    #[inline(always)]
    pub fn unlocks(&self) -> &[Unlock] {
        &self.0
    }
}

fn verify_unlocks<const VERIFY: bool>(unlocks: &[Unlock], _visitor: &()) -> Result<(), Error> {
    if VERIFY && unlocks.iter().any(Unlock::is_multi) {
        return Err(Error::MultiUnlockRecursion);
    } else {
        Ok(())
    }
}

#[cfg(feature = "serde")]
mod dto {
    use alloc::vec::Vec;

    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct MultiUnlockDto {
        #[serde(rename = "type")]
        kind: u8,
        unlocks: Vec<Unlock>,
    }

    impl From<&MultiUnlock> for MultiUnlockDto {
        fn from(value: &MultiUnlock) -> Self {
            Self {
                kind: MultiUnlock::KIND,
                unlocks: value.0.to_vec(),
            }
        }
    }

    impl TryFrom<MultiUnlockDto> for MultiUnlock {
        type Error = Error;

        fn try_from(value: MultiUnlockDto) -> Result<Self, Self::Error> {
            Self::new(value.unlocks)
        }
    }

    crate::impl_serde_typed_dto!(MultiUnlock, MultiUnlockDto, "multi unlock");
}
