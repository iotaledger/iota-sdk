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
    commitment_id: Option<SlotCommitmentId>, // `serix:",omitempty"`
}

impl OutputInclusionMetadata {
    pub fn new(slot: SlotIndex, transaction_id: TransactionId, commitment_id: Option<SlotCommitmentId>) -> Self {
        Self {
            slot,
            transaction_id,
            commitment_id,
        }
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
    commitment_id: Option<SlotCommitmentId>, // `serix:",omitempty"`
}

impl OutputConsumptionMetadata {
    pub fn new(slot: SlotIndex, transaction_id: TransactionId, commitment_id: Option<SlotCommitmentId>) -> Self {
        Self {
            slot,
            transaction_id,
            commitment_id,
        }
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
    spent: Option<OutputConsumptionMetadata>, // `serix:",optional,omitempty"`
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
