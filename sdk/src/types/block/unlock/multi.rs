// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;

use derive_more::Deref;
use packable::{prefix::BoxedSlicePrefix, Packable};

use crate::types::block::{
    address::WeightedAddressCount,
    protocol::{WorkScore, WorkScoreParameters},
    unlock::{Unlock, UnlockError},
};

pub(crate) type UnlocksCount = WeightedAddressCount;

/// Unlocks a [`MultiAddress`](crate::types::block::address::MultiAddress) with a list of other unlocks.
#[derive(Clone, Debug, Deref, Eq, PartialEq, Hash, Packable)]
#[packable(unpack_error = UnlockError, with = |e| e.unwrap_item_err_or_else(|p| UnlockError::MultiUnlockCount(p.into())))]
pub struct MultiUnlock(#[packable(verify_with = verify_unlocks)] BoxedSlicePrefix<Unlock, UnlocksCount>);

impl MultiUnlock {
    /// The [`Unlock`] kind of an [`MultiUnlock`].
    pub const KIND: u8 = 5;

    /// Creates a new [`MultiUnlock`].
    #[inline(always)]
    pub fn new(unlocks: impl IntoIterator<Item = Unlock>) -> Result<Self, UnlockError> {
        let unlocks = unlocks.into_iter().collect::<Box<[_]>>();

        verify_unlocks(&unlocks)?;

        Ok(Self(
            BoxedSlicePrefix::<Unlock, UnlocksCount>::try_from(unlocks).map_err(UnlockError::MultiUnlockCount)?,
        ))
    }

    /// Return the inner unlocks of an [`MultiUnlock`].
    #[inline(always)]
    pub fn unlocks(&self) -> &[Unlock] {
        &self.0
    }
}

impl WorkScore for MultiUnlock {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        self.0.work_score(params)
    }
}

fn verify_unlocks(unlocks: &[Unlock]) -> Result<(), UnlockError> {
    if unlocks.iter().any(Unlock::is_multi) {
        Err(UnlockError::MultiUnlockRecursion)
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
        type Error = UnlockError;

        fn try_from(value: MultiUnlockDto) -> Result<Self, Self::Error> {
            Self::new(value.unlocks)
        }
    }

    crate::impl_serde_typed_dto!(MultiUnlock, MultiUnlockDto, "multi unlock");
}
