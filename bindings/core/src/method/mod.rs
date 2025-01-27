// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "wallet")]
mod account;
mod client;
mod secret_manager;
mod utils;
#[cfg(feature = "wallet")]
mod wallet;

#[cfg(feature = "wallet")]
pub use self::{account::AccountMethod, wallet::WalletMethod};
pub use self::{client::ClientMethod, secret_manager::SecretManagerMethod, utils::UtilsMethod};
