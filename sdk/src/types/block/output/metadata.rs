// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    output::OutputId,
    payload::signed_transaction::TransactionId,
    slot::{SlotCommitmentId, SlotIndex},
    BlockId,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct OutputInclusionMetadata {
    // Slot in which the output was included.
    slot: SlotIndex,
    // Transaction ID that created the output.
    transaction_id: TransactionId,
    // Commitment ID that includes the creation of the output.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    commitment_id: Option<SlotCommitmentId>,
}

impl OutputInclusionMetadata {
    pub fn new(slot: SlotIndex, transaction_id: TransactionId, commitment_id: Option<SlotCommitmentId>) -> Self {
        Self {
            slot,
            transaction_id,
            commitment_id,
        }
    }

    pub fn slot(&self) -> SlotIndex {
        self.slot
    }

    pub fn transaction_id(&self) -> &TransactionId {
        &self.transaction_id
    }

    pub fn commitment_id(&self) -> Option<&SlotCommitmentId> {
        self.commitment_id.as_ref()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct OutputConsumptionMetadata {
    // Slot in which the output was spent.
    slot: SlotIndex,
    // Transaction ID that spent the output.
    transaction_id: TransactionId,
    // Commitment ID that includes the spending of the output.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    commitment_id: Option<SlotCommitmentId>,
}

impl OutputConsumptionMetadata {
    pub fn new(slot: SlotIndex, transaction_id: TransactionId, commitment_id: Option<SlotCommitmentId>) -> Self {
        Self {
            slot,
            transaction_id,
            commitment_id,
        }
    }

    pub fn slot(&self) -> SlotIndex {
        self.slot
    }

    pub fn transaction_id(&self) -> &TransactionId {
        &self.transaction_id
    }

    pub fn commitment_id(&self) -> Option<&SlotCommitmentId> {
        self.commitment_id.as_ref()
    }
}

/// Metadata of an [`Output`](crate::types::block::output::Output).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct OutputMetadata {
    /// The ID of the output.
    output_id: OutputId,
    /// The ID of the block in which the output was included.
    block_id: BlockId,
    // Metadata of the output if it is included in the ledger.
    included: OutputInclusionMetadata,
    // Metadata of the output if it is marked as spent in the ledger.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub(crate) spent: Option<OutputConsumptionMetadata>,
    /// Latest commitment ID of the node.
    latest_commitment_id: SlotCommitmentId,
}

impl OutputMetadata {
    /// Creates a new [`OutputMetadata`].
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

    /// Returns the block ID of the [`OutputMetadata`].
    pub fn block_id(&self) -> &BlockId {
        &self.block_id
    }

    /// Returns the inclusion metadata of the [`OutputMetadata`].
    pub fn included(&self) -> &OutputInclusionMetadata {
        &self.included
    }

    /// Returns the consumption metadata of the [`OutputMetadata`].
    pub fn spent(&self) -> Option<&OutputConsumptionMetadata> {
        self.spent.as_ref()
    }

    /// Returns whether the [`OutputMetadata`] is spent or not.
    pub fn is_spent(&self) -> bool {
        self.spent.is_some()
    }

    /// Returns the latest commitment ID of the [`OutputMetadata`].
    pub fn latest_commitment_id(&self) -> &SlotCommitmentId {
        &self.latest_commitment_id
    }
}
