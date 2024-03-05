// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod error;
mod utxo;

use core::ops::RangeInclusive;

use derive_more::From;

pub use self::{error::InputError, utxo::UtxoInput};
use crate::types::block::protocol::{WorkScore, WorkScoreParameters};

/// The maximum number of inputs of a transaction.
pub const INPUT_COUNT_MAX: u16 = 128;
/// The range of valid numbers of inputs of a transaction.
pub const INPUT_COUNT_RANGE: RangeInclusive<u16> = 1..=INPUT_COUNT_MAX; // [1..128]
/// The maximum index of inputs of a transaction.
pub const INPUT_INDEX_MAX: u16 = INPUT_COUNT_MAX - 1; // 127
/// The range of valid indices of inputs of a transaction.
pub const INPUT_INDEX_RANGE: RangeInclusive<u16> = 0..=INPUT_INDEX_MAX; // [0..127]

/// A generic input supporting different input kinds.
#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
#[packable(unpack_error = InputError)]
#[packable(tag_type = u8, with_error = InputError::Kind)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum Input {
    /// A UTXO input.
    #[packable(tag = UtxoInput::KIND)]
    Utxo(UtxoInput),
}

impl WorkScore for Input {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        match self {
            Self::Utxo(utxo) => utxo.work_score(params),
        }
    }
}

impl core::fmt::Debug for Input {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Utxo(input) => input.fmt(f),
        }
    }
}

impl Input {
    /// Returns the input kind of an `Input`.
    pub fn kind(&self) -> u8 {
        match self {
            Self::Utxo(_) => UtxoInput::KIND,
        }
    }

    crate::def_is_as_opt!(Input: Utxo);
}

#[cfg(feature = "serde")]
crate::impl_deserialize_untagged!(Input: Utxo);
