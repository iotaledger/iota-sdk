// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{output::OutputId, payload::transaction::TransactionId, slot::SlotCommitmentId, BlockId};

/// Metadata of an [`Output`](crate::types::block::output::Output).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OutputMetadata {
    /// The ID of the block in which the output was included.
    block_id: BlockId,
    /// The ID of the output.
    output_id: OutputId,
    /// Whether the output is spent or not.
    is_spent: bool,
    // Commitment ID that includes the spent output.
    commitment_id_spent: Option<SlotCommitmentId>,
    // Transaction ID that spent the output.
    transaction_id_spent: Option<TransactionId>,
    /// Commitment ID that includes the output.
    included_commitment_id: Option<SlotCommitmentId>,
    /// Latest commitment ID of the node.
    latest_commitment_id: SlotCommitmentId,
}

impl OutputMetadata {
    /// Creates a new [`OutputMetadata`].
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        block_id: BlockId,
        output_id: OutputId,
        is_spent: bool,
        commitment_id_spent: Option<SlotCommitmentId>,
        transaction_id_spent: Option<TransactionId>,
        included_commitment_id: Option<SlotCommitmentId>,
        latest_commitment_id: SlotCommitmentId,
    ) -> Self {
        Self {
            block_id,
            output_id,
            is_spent,
            commitment_id_spent,
            transaction_id_spent,
            included_commitment_id,
            latest_commitment_id,
        }
    }

    /// Returns the block ID of the [`OutputMetadata`].
    pub fn block_id(&self) -> &BlockId {
        &self.block_id
    }

    /// Returns the output ID of the [`OutputMetadata`].
    pub fn output_id(&self) -> &OutputId {
        &self.output_id
    }

    /// Returns the transaction ID of the [`OutputMetadata`].
    pub fn transaction_id(&self) -> &TransactionId {
        self.output_id.transaction_id()
    }

    /// Returns the output index of the [`OutputMetadata`].
    pub fn output_index(&self) -> u16 {
        self.output_id.index()
    }

    /// Returns whether the output is spent ot not.
    pub fn is_spent(&self) -> bool {
        self.is_spent
    }

    /// Sets whether the output is spent ot not.
    pub fn set_spent(&mut self, spent: bool) {
        self.is_spent = spent;
    }

    /// Returns the commitment ID spent of the [`OutputMetadata`].
    pub fn commitment_id_spent(&self) -> Option<&SlotCommitmentId> {
        self.commitment_id_spent.as_ref()
    }

    /// Returns the transaction ID spent of the [`OutputMetadata`].
    pub fn transaction_id_spent(&self) -> Option<&TransactionId> {
        self.transaction_id_spent.as_ref()
    }

    /// Returns the included commitment ID of the [`OutputMetadata`].
    pub fn included_commitment_id(&self) -> Option<&SlotCommitmentId> {
        self.included_commitment_id.as_ref()
    }

    /// Returns the latest commitment ID of the [`OutputMetadata`].
    pub fn latest_commitment_id(&self) -> &SlotCommitmentId {
        &self.latest_commitment_id
    }
}

#[cfg(feature = "serde")]
mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct OutputMetadataDto {
        block_id: BlockId,
        transaction_id: TransactionId,
        output_index: u16,
        is_spent: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        commitment_id_spent: Option<SlotCommitmentId>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transaction_id_spent: Option<TransactionId>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        included_commitment_id: Option<SlotCommitmentId>,
        latest_commitment_id: SlotCommitmentId,
    }

    impl TryFrom<OutputMetadataDto> for OutputMetadata {
        type Error = crate::types::block::Error;

        fn try_from(value: OutputMetadataDto) -> Result<Self, Self::Error> {
            Ok(Self {
                block_id: value.block_id,
                output_id: OutputId::new(value.transaction_id, value.output_index)?,
                is_spent: value.is_spent,
                commitment_id_spent: value.commitment_id_spent,
                transaction_id_spent: value.transaction_id_spent,
                included_commitment_id: value.included_commitment_id,
                latest_commitment_id: value.latest_commitment_id,
            })
        }
    }

    impl From<&OutputMetadata> for OutputMetadataDto {
        fn from(value: &OutputMetadata) -> Self {
            Self {
                block_id: value.block_id,
                transaction_id: *value.transaction_id(),
                output_index: value.output_index(),
                is_spent: value.is_spent,
                commitment_id_spent: value.commitment_id_spent,
                transaction_id_spent: value.transaction_id_spent,
                included_commitment_id: value.included_commitment_id,
                latest_commitment_id: value.latest_commitment_id,
            }
        }
    }

    impl<'de> Deserialize<'de> for OutputMetadata {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            OutputMetadataDto::deserialize(d).and_then(|dto| dto.try_into().map_err(serde::de::Error::custom))
        }
    }

    impl Serialize for OutputMetadata {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            OutputMetadataDto::from(self).serialize(s)
        }
    }
}
