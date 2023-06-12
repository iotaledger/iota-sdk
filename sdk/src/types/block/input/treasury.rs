// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use derive_more::{Deref, From};

use crate::types::block::{payload::milestone::MilestoneId, Error};

/// [`TreasuryInput`] is an input which references a milestone which generated a
/// [`TreasuryOutput`](crate::types::block::output::TreasuryOutput).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, From, Deref, packable::Packable)]
pub struct TreasuryInput(MilestoneId);

impl TreasuryInput {
    /// The input kind of a [`TreasuryInput`].
    pub const KIND: u8 = 1;

    /// Creates a new [`TreasuryInput`].
    pub fn new(id: MilestoneId) -> Self {
        Self(id)
    }

    /// Returns the milestones id of a [`TreasuryInput`].
    pub fn milestone_id(&self) -> &MilestoneId {
        &self.0
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(TreasuryInput);

impl FromStr for TreasuryInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(MilestoneId::from_str(s)?))
    }
}

impl core::fmt::Display for TreasuryInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::fmt::Debug for TreasuryInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TreasuryInput({})", self.0)
    }
}

#[allow(missing_docs)]
pub mod dto {
    use alloc::string::{String, ToString};

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    /// Describes an input which references an unspent treasury output to consume.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TreasuryInputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub milestone_id: String,
    }

    impl From<&TreasuryInput> for TreasuryInputDto {
        fn from(value: &TreasuryInput) -> Self {
            Self {
                kind: TreasuryInput::KIND,
                milestone_id: value.milestone_id().to_string(),
            }
        }
    }

    impl TryFrom<TreasuryInputDto> for TreasuryInput {
        type Error = Error;

        fn try_from(value: TreasuryInputDto) -> Result<Self, Self::Error> {
            Ok(Self::new(
                value
                    .milestone_id
                    .parse::<MilestoneId>()
                    .map_err(|_| Error::InvalidField("milestoneId"))?,
            ))
        }
    }
}
