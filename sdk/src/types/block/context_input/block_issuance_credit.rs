// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Display, From};

use crate::types::block::{
    output::AccountId,
    protocol::{WorkScore, WorkScoreParameters},
};

/// A Block Issuance Credit (BIC) Context Input provides the VM with context for the value of
/// the BIC vector of a specific slot.
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
pub struct BlockIssuanceCreditContextInput(AccountId);

impl BlockIssuanceCreditContextInput {
    /// The context input kind of a [`BlockIssuanceCreditContextInput`].
    pub const KIND: u8 = 2;

    /// Creates a new [`BlockIssuanceCreditContextInput`].
    pub fn new(account_id: AccountId) -> Self {
        Self(account_id)
    }

    /// Returns the account id of the [`BlockIssuanceCreditContextInput`].
    pub fn account_id(&self) -> AccountId {
        self.0
    }
}

impl WorkScore for BlockIssuanceCreditContextInput {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.context_input()
    }
}

#[cfg(feature = "serde")]
mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    /// A Block Issuance Credit (BIC) Context Input provides the VM with context for the value of
    /// the BIC vector of a specific slot.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct BlockIssuanceCreditContextInputDto {
        #[serde(rename = "type")]
        kind: u8,
        account_id: AccountId,
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

    crate::impl_serde_typed_dto!(
        BlockIssuanceCreditContextInput,
        BlockIssuanceCreditContextInputDto,
        "block issuance credit context input"
    );
}