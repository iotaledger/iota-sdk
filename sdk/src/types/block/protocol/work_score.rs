// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::CopyGetters;
use packable::Packable;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, CopyGetters)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[getset(get_copy = "pub")]
pub struct WorkScoreParameters {
    /// Accounts for the network traffic per byte.
    pub(crate) data_byte: u32,
    /// Accounts for work done to process a block in the node software.
    pub(crate) block: u32,
    /// Accounts for loading the UTXO from the database and performing the mana balance check.
    pub(crate) input: u32,
    /// Accounts for loading and checking the context input.
    pub(crate) context_input: u32,
    /// Accounts for storing the UTXO in the database.
    pub(crate) output: u32,
    /// Accounts for native token balance checks which use big integers.
    pub(crate) native_token: u32,
    /// Accounts for the cost of updating the staking vector when a staking feature is present.
    pub(crate) staking: u32,
    /// Accounts for the cost of updating the block issuer keys when a block issuer feature is present.
    pub(crate) block_issuer: u32,
    /// Accounts for accessing the account based ledger to transform the allotted mana to block issuance credits.
    pub(crate) allotment: u32,
    /// Accounts for an Ed25519 signature check.
    pub(crate) signature_ed25519: u32,
}

/// A trait to facilitate the computation of the work score of a block, which is central to mana cost calculation.
pub trait WorkScore {
    fn work_score(&self, _params: WorkScoreParameters) -> u32 {
        0
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
