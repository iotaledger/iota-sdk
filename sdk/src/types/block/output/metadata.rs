// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    output::OutputId, payload::signed_transaction::TransactionId, slot::SlotCommitmentId, slot::SlotIndex, BlockId,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct OutputInclusionMetadata {
    // Slot in which the output was included.
    slot: SlotIndex,
    // Transaction ID that created the output.
    transaction_id: TransactionId,
    // Commitment ID that includes the creation of the output.
    commitment_id: Option<SlotCommitmentId>, // `serix:",omitempty"`
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct OutputConsumptionMetadata {
    // Slot in which the output was spent.
    slot: SlotIndex,
    // Transaction ID that spent the output.
    transaction_id: TransactionId,
    // Commitment ID that includes the spending of the output.
    commitment_id: Option<SlotCommitmentId>, // `serix:",omitempty"`
}

/// Metadata of an [`Output`](crate::types::block::output::Output).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OutputMetadata {
    /// The ID of the output.
    output_id: OutputId,
    /// The ID of the block in which the output was included.
    block_id: BlockId,
    // Metadata of the output if it is included in the ledger.
    included: OutputInclusionMetadata,
    // Metadata of the output if it is marked as spent in the ledger.
    spent: Option<OutputConsumptionMetadata>, // `serix:",optional,omitempty"`
    /// Latest commitment ID of the node.
    latest_commitment_id: SlotCommitmentId,
}

impl OutputMetadata {
    /// Creates a new [`OutputMetadata`].
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        output_id: OutputId,
        block_id: BlockId,
        included: OutputInclusionMetadata,
        spent: Option<OutputConsumptionMetadata>,
        latest_commitment_id: SlotCommitmentId,
    ) -> Self {
        Self {
            block_id,
            output_id,
            included,
            spent,
            latest_commitment_id,
        }
    }

    /// Returns the output ID of the [`OutputMetadata`].
    pub fn output_id(&self) -> &OutputId {
        &self.output_id
    }

    pub fn transaction_id(&self) -> &TransactionId {
        self.output_id.transaction_id()
    }

    /// Returns the output index of the [`OutputMetadata`].
    pub fn output_index(&self) -> u16 {
        self.output_id.index()
    }

    /// Returns the block ID of the [`OutputMetadata`].
    pub fn block_id(&self) -> &BlockId {
        &self.block_id
    }

    /// Returns whether the output is spent or not.
    pub fn is_spent(&self) -> bool {
        self.spent.is_some()
    }

    // /// Sets whether the output is spent or not.
    // pub fn set_spent(&mut self, spent: bool) {
    //     self.is_spent = spent;
    // }

    // /// Returns the commitment ID spent of the [`OutputMetadata`].
    // pub fn commitment_id_spent(&self) -> Option<&SlotCommitmentId> {
    //     self.commitment_id_spent.as_ref()
    // }

    // /// Returns the transaction ID spent of the [`OutputMetadata`].
    // pub fn transaction_id_spent(&self) -> Option<&TransactionId> {
    //     self.transaction_id_spent.as_ref()
    // }

    // /// Returns the included commitment ID of the [`OutputMetadata`].
    // pub fn included_commitment_id(&self) -> Option<&SlotCommitmentId> {
    //     self.included_commitment_id.as_ref()
    // }

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
        output_id: OutputId,
        block_id: BlockId,
        is_spent: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        commitment_id_spent: Option<SlotCommitmentId>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transaction_id_spent: Option<TransactionId>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        included_commitment_id: Option<SlotCommitmentId>,
        latest_commitment_id: SlotCommitmentId,
    }

    impl From<OutputMetadataDto> for OutputMetadata {
        fn from(value: OutputMetadataDto) -> Self {
            Self {
                output_id: value.output_id,
                block_id: value.block_id,
                is_spent: value.is_spent,
                commitment_id_spent: value.commitment_id_spent,
                transaction_id_spent: value.transaction_id_spent,
                included_commitment_id: value.included_commitment_id,
                latest_commitment_id: value.latest_commitment_id,
            }
        }
    }

    impl From<&OutputMetadata> for OutputMetadataDto {
        fn from(value: &OutputMetadata) -> Self {
            Self {
                output_id: value.output_id,
                block_id: value.block_id,
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
            Ok(OutputMetadataDto::deserialize(d)?.into())
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
