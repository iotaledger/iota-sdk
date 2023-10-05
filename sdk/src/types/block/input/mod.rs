// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod utxo;

use core::ops::RangeInclusive;

use derive_more::From;

pub use self::utxo::UtxoInput;
use crate::types::block::Error;

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
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidInputKind)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum Input {
    /// A UTXO input.
    #[packable(tag = UtxoInput::KIND)]
    Utxo(UtxoInput),
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

    /// Checks whether the input is a [`UtxoInput`].
    pub fn is_utxo(&self) -> bool {
        matches!(self, Self::Utxo(_))
    }

    /// Gets the input as an actual [`UtxoInput`].
    /// PANIC: do not call on a non-utxo input.
    pub fn as_utxo(&self) -> &UtxoInput {
        let Self::Utxo(input) = self;
        input
    }
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::utils::json::{FromJson, ToJson, Value};

    impl ToJson for Input {
        fn to_json(&self) -> Value {
            match self {
                Self::Utxo(i) => i.to_json(),
            }
        }
    }

    impl FromJson for Input {
        type Error = Error;

        fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            Ok(match value["type"].as_u8() {
                Some(UtxoInput::KIND) => UtxoInput::from_json(value)?.into(),
                _ => {
                    return Err(Error::invalid_type::<Self>(
                        format!("one of {:?}", [UtxoInput::KIND]),
                        &value["type"],
                    ));
                }
            })
        }
    }
}
