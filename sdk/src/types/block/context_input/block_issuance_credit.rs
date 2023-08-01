// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Display, From};

use crate::types::block::output::AccountId;

/// A Block Issuance Credit Input provides the VM with context for the value of
/// the BIC vector of a specific slot.
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
pub struct BlockIssuanceCreditContextInput(AccountId);

impl BlockIssuanceCreditContextInput {
    /// The context input kind of a [`BlockIssuanceCreditContextInput`].
    pub const KIND: u8 = 1;

    /// Creates a new [`BlockIssuanceCreditContextInput`].
    pub fn new(account_id: AccountId) -> Self {
        Self(account_id)
    }

    /// Returns the account id of the [`BlockIssuanceCreditContextInput`].
    pub fn account_id(&self) -> AccountId {
        self.0
    }
}

pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    /// A Block Issuance Credit Input provides the VM with context for the value of
    /// the BIC vector of a specific slot.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct BlockIssuanceCreditContextInputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub account_id: AccountId,
    }

    impl From<&BlockIssuanceCreditContextInput> for BlockIssuanceCreditContextInputDto {
        fn from(value: &BlockIssuanceCreditContextInput) -> Self {
            Self {
                kind: BlockIssuanceCreditContextInput::KIND,
                account_id: value.account_id(),
            }
        }
    }

    impl From<BlockIssuanceCreditContextInputDto> for BlockIssuanceCreditContextInput {
        fn from(value: BlockIssuanceCreditContextInputDto) -> Self {
            Self::new(value.account_id)
        }
    }
}
