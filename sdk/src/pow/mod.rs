// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Provides proof of work implementations and scoring for the IOTA protocol.
//! TIP <https://github.com/iotaledger/tips/blob/main/tips/TIP-0012/tip-0012.md>.

pub mod miner;
pub mod score;
#[cfg(target_family = "wasm")]
pub mod wasm_miner;

// Precomputed natural logarithm of 3 for performance reasons.
// See https://oeis.org/A002391.
const LN_3: f64 = 1.098_612_288_668_109;
