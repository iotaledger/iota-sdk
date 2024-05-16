// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod call_method;
mod client;
mod secret_manager;
mod utils;
#[cfg(feature = "wallet")]
mod wallet;

#[cfg(feature = "wallet")]
pub use call_method::call_wallet_method;
pub use call_method::{call_client_method, call_secret_manager_method, call_utils_method, CallMethod};
#[cfg(feature = "mqtt")]
pub use client::listen_mqtt;
