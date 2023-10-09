// Copyright 2022 IOTA Stiftung
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
pub struct WorkScoreStructure {
    /// Modifier for network traffic per byte.
    pub(crate) data_byte: u32,
    /// Modifier for work done to process a block.
    pub(crate) block: u32,
    /// Modifier for slashing when there are insufficient strong tips.
    pub(crate) missing_parent: u32,
    /// Modifier for loading UTXOs and performing mana calculations.
    pub(crate) input: u32,
    /// Modifier for loading and checking the context input.
    pub(crate) context_input: u32,
    /// Modifier for storing UTXOs.
    pub(crate) output: u32,
    /// Modifier for calculations using native tokens.
    pub(crate) native_token: u32,
    /// Modifier for storing staking features.
    pub(crate) staking: u32,
    /// Modifier for storing block issuer features.
    pub(crate) block_issuer: u32,
    /// Modifier for accessing the account-based ledger to transform mana to Block Issuance Credits.
    pub(crate) allotment: u32,
    /// Modifier for the block signature check.
    pub(crate) signature_ed25519: u32,
    /// The minimum count of strong parents in a basic block.
    pub(crate) min_strong_parents_threshold: u8,
}

impl Default for WorkScoreStructure {
    fn default() -> Self {
        Self {
            data_byte: 0,
            block: 100,
            missing_parent: 500,
            input: 20,
            context_input: 20,
            output: 20,
            native_token: 20,
            staking: 100,
            block_issuer: 100,
            allotment: 100,
            signature_ed25519: 200,
            min_strong_parents_threshold: 4,
        }
    }
}

/// A trait to facilitate the computation of the work score of a block, which is central to mana cost calculation.
pub trait WorkScore {
    /// Returns its work score.
    fn work_score(&self, work_score_params: WorkScoreStructure) -> u32;

    /// Returns the Mana cost given its work score.
    fn mana_cost(&self, work_score_params: WorkScoreStructure, reference_mana_cost: u64) -> u64 {
        reference_mana_cost * self.work_score(work_score_params) as u64
    }
}

impl<T: WorkScore, const N: usize> WorkScore for [T; N] {
    fn work_score(&self, work_score_params: WorkScoreStructure) -> u32 {
        self.as_slice().work_score(work_score_params)
    }
}

impl<T: WorkScore> WorkScore for [T] {
    fn work_score(&self, work_score_params: WorkScoreStructure) -> u32 {
        self.iter().map(|o| o.work_score(work_score_params)).sum()
    }
}
