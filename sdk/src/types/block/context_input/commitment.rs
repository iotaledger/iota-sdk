// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Display, From};

use crate::types::block::slot::SlotCommitmentId;

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

    /// Returns the commitment id of the [`CommitmentContextInput`].
    pub fn commitment_id(&self) -> SlotCommitmentId {
        self.0
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
                commitment_id: value.commitment_id(),
            }
        }
    }

    impl From<CommitmentContextInputDto> for CommitmentContextInput {
        fn from(value: CommitmentContextInputDto) -> Self {
            Self::new(value.commitment_id)
        }
    }
    impl_serde_typed_dto!(
        CommitmentContextInput,
        CommitmentContextInputDto,
        "commitment context input"
    );
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::{
        types::block::Error,
        utils::json::{FromJson, TakeValue, ToJson, Value},
    };

    impl ToJson for CommitmentContextInput {
        fn to_json(&self) -> Value {
            crate::json! ({
                "type": Self::KIND,
                "commitmentId": self.0
            })
        }
    }

    impl FromJson for CommitmentContextInput {
        type Error = Error;

        fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            if value["type"] != Self::KIND {
                return Err(Error::invalid_type::<Self>(Self::KIND, &value["type"]));
            }
            Ok(Self::new(value["commitmentId"].take_value()?))
        }
    }
}
