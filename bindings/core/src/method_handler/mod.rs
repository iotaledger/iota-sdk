// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod call_method;
mod client;
mod secret_manager;
mod utils;
mod wallet;

pub use call_method::{call_client_method, call_secret_manager_method, call_utils_method, call_wallet_method};
#[cfg(feature = "mqtt")]
pub use client::listen_mqtt;

/// Result type of the bindings core crate.
pub type Result<T> = std::result::Result<T, super::error::Error>;
