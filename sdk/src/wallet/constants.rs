// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Amount at which outputs on a single address will get consolidated by default if consolidation is enabled
pub(crate) const DEFAULT_OUTPUT_CONSOLIDATION_THRESHOLD: usize = 100;
/// Amount at which outputs on a single address will get consolidated by default with a ledger secret_manager if
/// consolidation is enabled, needs to be smaller because the memory of the ledger nano s is limited
#[cfg(feature = "ledger_nano")]
pub(crate) const DEFAULT_LEDGER_OUTPUT_CONSOLIDATION_THRESHOLD: usize = 15;

/// ms before the wallet actually syncs with the network, before it just returns the previous syncing result
/// this is done to prevent unnecessary simultaneous synchronizations
pub(crate) const MIN_SYNC_INTERVAL: u128 = 5;

// TODO Used to be one day in seconds, what now ?
// Default expiration slots for [ExpirationUnlockCondition] when sending native tokens,
pub(crate) const DEFAULT_EXPIRATION_SLOTS: u32 = 100;
