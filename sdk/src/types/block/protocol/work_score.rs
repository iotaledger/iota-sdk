// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::CopyGetters;
use packable::Packable;

use crate::types::block::Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, CopyGetters)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[packable(unpack_error = Error)]
#[getset(get_copy = "pub")]
pub struct WorkScoreParameters {
    /// Accounts for the network traffic per byte.
    data_byte: u32,
    /// Accounts for work done to process a block in the node software.
    block: u32,
    /// Accounts for loading the UTXO from the database and performing the mana balance check.
    input: u32,
    /// Accounts for loading and checking the context input.
    context_input: u32,
    /// Accounts for storing the UTXO in the database.
    output: u32,
    /// Accounts for native token balance checks which use big integers.
    native_token: u32,
    /// Accounts for the cost of updating the staking vector when a staking feature is present.
    staking: u32,
    /// Accounts for the cost of updating the block issuer keys when a block issuer feature is present.
    block_issuer: u32,
    /// Accounts for accessing the account based ledger to transform the allotted mana to block issuance credits.
    allotment: u32,
    /// Accounts for an Ed25519 signature check.
    signature_ed25519: u32,
}

impl Default for WorkScoreParameters {
    fn default() -> Self {
        Self {
            data_byte: 0,
            block: 100,
            input: 20,
            context_input: 20,
            output: 20,
            native_token: 20,
            staking: 100,
            block_issuer: 100,
            allotment: 100,
            signature_ed25519: 200,
        }
    }
}

/// A trait to facilitate the computation of the work score of a block, which is central to mana cost calculation.
pub trait WorkScore {
    /// Returns its work score. Defaults to 0.
    fn work_score(&self, _params: WorkScoreParameters) -> u32 {
        0
    }

    /// Returns the Mana cost given its work score.
    fn mana_cost(&self, params: WorkScoreParameters, reference_mana_cost: u64) -> u64 {
        reference_mana_cost * self.work_score(params) as u64
    }
}

impl<T: WorkScore, const N: usize> WorkScore for [T; N] {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        self.as_slice().work_score(params)
    }
}

impl<T: WorkScore> WorkScore for [T] {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        self.iter().map(|o| o.work_score(params)).sum()
    }
}
