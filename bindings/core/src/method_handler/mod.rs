// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod call_method;
mod client;
mod secret_manager;
mod utils;
mod wallet;

#[cfg(not(target_family = "wasm"))]
pub use call_method::CallMethod;
pub use call_method::{call_client_method, call_secret_manager_method, call_utils_method, call_wallet_method};
#[cfg(feature = "mqtt")]
pub use client::listen_mqtt;
