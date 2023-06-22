// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{output::OutputId, payload::transaction::TransactionId, slot::SlotCommitmentId, BlockId};

/// Metadata of an [`Output`](crate::types::block::output::Output).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct OutputMetadata {
    /// The ID of the block in which the output was included.
    block_id: BlockId,
    /// The ID of the output.
    output_id: OutputId,
    /// Whether the output is spent or not.
    is_spent: bool,
    // Commitment ID that includes the spent output.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    commitment_id_spent: Option<SlotCommitmentId>,
    // Transaction ID that spends the output.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    transaction_id_spent: Option<TransactionId>,
    /// Commitment ID that includes the output.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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

#[allow(missing_docs)]
pub mod dto {
    use alloc::string::{String, ToString};
    use core::str::FromStr;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    /// DTO for an [`OutputMetadata`].
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OutputMetadataDto {
        pub block_id: String,
        pub transaction_id: String,
        pub output_index: u16,
        pub is_spent: bool,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub commitment_id_spent: Option<SlotCommitmentId>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub transaction_id_spent: Option<TransactionId>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub included_commitment_id: Option<SlotCommitmentId>,
        pub latest_commitment_id: SlotCommitmentId,
    }

    impl OutputMetadataDto {
        /// Returns the output id.
        pub fn output_id(&self) -> Result<OutputId, crate::types::block::Error> {
            OutputId::new(TransactionId::from_str(&self.transaction_id)?, self.output_index)
        }
    }

    impl TryFrom<OutputMetadataDto> for OutputMetadata {
        type Error = Error;

        fn try_from(response: OutputMetadataDto) -> Result<Self, Self::Error> {
            Ok(Self {
                block_id: BlockId::from_str(&response.block_id)?,
                output_id: OutputId::new(
                    TransactionId::from_str(&response.transaction_id)?,
                    response.output_index,
                )?,
                is_spent: response.is_spent,
                commitment_id_spent: response.commitment_id_spent,
                transaction_id_spent: response.transaction_id_spent,
                included_commitment_id: response.included_commitment_id,
                latest_commitment_id: response.latest_commitment_id,
            })
        }
    }

    impl From<&OutputMetadata> for OutputMetadataDto {
        fn from(output_metadata: &OutputMetadata) -> Self {
            Self {
                block_id: output_metadata.block_id().to_string(),
                transaction_id: output_metadata.transaction_id().to_string(),
                output_index: output_metadata.output_index(),
                is_spent: output_metadata.is_spent(),
                commitment_id_spent: output_metadata.commitment_id_spent().cloned(),
                transaction_id_spent: output_metadata.transaction_id_spent().cloned(),
                included_commitment_id: output_metadata.included_commitment_id().cloned(),
                latest_commitment_id: *output_metadata.latest_commitment_id(),
            }
        }
    }
}
