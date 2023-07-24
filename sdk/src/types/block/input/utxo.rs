// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use derive_more::From;

use crate::types::block::{output::OutputId, payload::transaction::TransactionId, Error};

/// Represents an input referencing an output.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
pub struct UtxoInput(OutputId);

impl UtxoInput {
    /// The input kind of a [`UtxoInput`].
    pub const KIND: u8 = 0;

    /// Creates a new [`UtxoInput`].
    pub fn new(id: TransactionId, index: u16) -> Result<Self, Error> {
        Ok(Self(OutputId::new(id, index)?))
    }

    /// Returns the output id of a [`UtxoInput`].
    pub fn output_id(&self) -> &OutputId {
        &self.0
    }
}

impl FromStr for UtxoInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(OutputId::from_str(s)?))
    }
}

impl core::fmt::Display for UtxoInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::fmt::Debug for UtxoInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UtxoInput({})", self.0)
    }
}

pub(super) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    /// Describes an input which references an unspent transaction output to consume.
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct UtxoInputDto {
        #[serde(rename = "type")]
        kind: u8,
        transaction_id: TransactionId,
        transaction_output_index: u16,
    }

    impl From<&UtxoInput> for UtxoInputDto {
        fn from(value: &UtxoInput) -> Self {
            Self {
                kind: UtxoInput::KIND,
                transaction_id: *value.output_id().transaction_id(),
                transaction_output_index: value.output_id().index(),
            }
        }
    }

    impl TryFrom<UtxoInputDto> for UtxoInput {
        type Error = Error;

        fn try_from(value: UtxoInputDto) -> Result<Self, Self::Error> {
            Self::new(value.transaction_id, value.transaction_output_index)
        }
    }

    impl<'de> Deserialize<'de> for UtxoInput {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let dto = UtxoInputDto::deserialize(d)?;
            if dto.kind != Self::KIND {
                return Err(serde::de::Error::custom(format!(
                    "invalid UTXO input type: expected {}, found {}",
                    Self::KIND,
                    dto.kind
                )));
            }
            dto.try_into().map_err(serde::de::Error::custom)
        }
    }

    impl Serialize for UtxoInput {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            UtxoInputDto::from(self).serialize(s)
        }
    }
}
