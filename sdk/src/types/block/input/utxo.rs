// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use derive_more::From;

use crate::types::block::{
    input::InputError,
    output::OutputId,
    payload::signed_transaction::TransactionId,
    protocol::{WorkScore, WorkScoreParameters},
};

/// Represents an input referencing an output.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
pub struct UtxoInput(OutputId);

impl UtxoInput {
    /// The input kind of a [`UtxoInput`].
    pub const KIND: u8 = 0;

    /// Creates a new [`UtxoInput`].
    pub fn new(id: TransactionId, index: u16) -> Self {
        Self(OutputId::new(id, index))
    }

    /// Returns the output id of a [`UtxoInput`].
    pub fn output_id(&self) -> &OutputId {
        &self.0
    }
}

impl WorkScore for UtxoInput {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.input()
    }
}

impl FromStr for UtxoInput {
    type Err = InputError;

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

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

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

    impl From<UtxoInputDto> for UtxoInput {
        fn from(value: UtxoInputDto) -> Self {
            Self::new(value.transaction_id, value.transaction_output_index)
        }
    }

    crate::impl_serde_typed_dto!(UtxoInput, UtxoInputDto, "UTXO input");
}
