// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::protocol::WorkScore;

/// Used to maintain correct index relationship between addresses and signatures when unlocking a
/// [`MultiAddress`](crate::types::block::address::MultiAddress) where not all addresses are unlocked.
#[derive(Clone, Debug, Eq, PartialEq, Hash, packable::Packable)]
pub struct EmptyUnlock;

impl EmptyUnlock {
    /// The [`Unlock`](crate::types::block::unlock::Unlock) kind of an [`EmptyUnlock`].
    pub const KIND: u8 = 6;
}

impl WorkScore for EmptyUnlock {}

#[cfg(feature = "serde")]
mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct EmptyUnlockDto {
        #[serde(rename = "type")]
        kind: u8,
    }

    impl From<&EmptyUnlock> for EmptyUnlockDto {
        fn from(_: &EmptyUnlock) -> Self {
            Self {
                kind: EmptyUnlock::KIND,
            }
        }
    }

    impl From<EmptyUnlockDto> for EmptyUnlock {
        fn from(_: EmptyUnlockDto) -> Self {
            Self
        }
    }

    crate::impl_serde_typed_dto!(EmptyUnlock, EmptyUnlockDto, "empty unlock");
}
