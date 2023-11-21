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
    /// Modifier for network traffic per byte.
    data_byte: u32,
    /// Modifier for work done to process a block.
    block: u32,
    /// Modifier for loading UTXOs and performing mana calculations.
    input: u32,
    /// Modifier for loading and checking the context input.
    context_input: u32,
    /// Modifier for storing UTXOs.
    output: u32,
    /// Modifier for calculations using native token features.
    native_token: u32,
    /// Modifier for storing staking features.
    staking: u32,
    /// Modifier for storing block issuer features.
    block_issuer: u32,
    /// Modifier for accessing the account-based ledger to transform mana to Block Issuance Credits.
    allotment: u32,
    /// Modifier for the block signature check.
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
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
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
