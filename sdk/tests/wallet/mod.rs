// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod address_generation;
#[cfg(all(feature = "stronghold", feature = "storage"))]
// TODO: see what's still needed
// mod backup_restore;
mod balance;
mod bech32_hrp_validation;
mod burn_outputs;
mod claim_outputs;
mod common;
mod consolidation;
mod core;
mod error;
#[cfg(feature = "events")]
mod events;
#[cfg(feature = "stronghold")]
mod migrate_stronghold_snapshot_v2_to_v3;
mod native_tokens;
mod output_preparation;
// TODO: update
// mod syncing;
// mod transactions;
