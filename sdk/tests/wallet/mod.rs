// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account_deserialization;
mod account_recovery;
mod accounts;
mod backup_restore;
mod balance;
mod bech32_hrp_validation;
mod burn_outputs;
mod claim_outputs;
mod common;
mod consolidation;
mod error;
#[cfg(feature = "message_interface")]
mod message_interface;
mod native_tokens;
mod output_preparation;
mod syncing;
mod transactions;
#[allow(clippy::module_inception)]
mod wallet;
