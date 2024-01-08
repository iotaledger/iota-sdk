// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Display, From};

use crate::types::block::{
    protocol::{WorkScore, WorkScoreParameters},
    slot::{SlotCommitmentId, SlotIndex},
};

/// A Commitment Context Input references a commitment to a certain slot.
#[derive(Clone, Copy, Display, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
pub struct CommitmentContextInput(SlotCommitmentId);

impl CommitmentContextInput {
    /// The context input kind of a [`CommitmentContextInput`].
    pub const KIND: u8 = 0;

    /// Creates a new [`CommitmentContextInput`].
    pub fn new(commitment_id: SlotCommitmentId) -> Self {
        Self(commitment_id)
    }

    /// Returns the slot commitment id of the [`CommitmentContextInput`].
    pub fn slot_commitment_id(&self) -> SlotCommitmentId {
        self.0
    }

    /// Returns the slot index of the [`CommitmentContextInput`].
    pub fn slot_index(&self) -> SlotIndex {
        self.0.slot_index()
    }
}

impl WorkScore for CommitmentContextInput {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.context_input()
    }
}

#[cfg(feature = "serde")]
mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    /// A Commitment Context Input references a commitment to a certain slot.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct CommitmentContextInputDto {
        #[serde(rename = "type")]
        kind: u8,
        commitment_id: SlotCommitmentId,
    }

    impl From<&CommitmentContextInput> for CommitmentContextInputDto {
        fn from(value: &CommitmentContextInput) -> Self {
            Self {
                kind: CommitmentContextInput::KIND,
                commitment_id: value.slot_commitment_id(),
            }
        }
    }

    impl From<CommitmentContextInputDto> for CommitmentContextInput {
        fn from(value: CommitmentContextInputDto) -> Self {
            Self::new(value.commitment_id)
        }
    }
    crate::impl_serde_typed_dto!(
        CommitmentContextInput,
        CommitmentContextInputDto,
        "commitment context input"
    );
}
