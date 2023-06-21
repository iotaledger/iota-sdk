// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account_recovery;
mod accounts;
#[cfg(all(feature = "stronghold", feature = "storage"))]
mod backup_restore;
mod balance;
mod bech32_hrp_validation;
mod burn_outputs;
mod claim_outputs;
mod common;
mod consolidation;
mod error;
#[cfg(feature = "events")]
mod events;
#[cfg(feature = "message_interface")]
mod message_interface;
#[cfg(feature = "stronghold")]
mod migrate_stronghold_snapshot_v2_to_v3;
mod native_tokens;
mod output_preparation;
mod syncing;
mod transactions;
#[allow(clippy::module_inception)]
mod wallet;
