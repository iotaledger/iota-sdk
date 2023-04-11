// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod client;
mod secret_manager;
mod utils;
mod wallet;

use std::fmt::{Formatter, Result as FmtResult};

use iota_sdk::client::secret::SecretManagerDto;

pub use self::{account::AccountMethod, client::ClientMethod, utils::UtilityMethod, wallet::WalletMethod};

pub(crate) trait OmittedDebug {
    fn omitted_fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<omitted>")
    }
}
impl OmittedDebug for String {}
impl OmittedDebug for SecretManagerDto {}
impl<T: OmittedDebug> OmittedDebug for Option<T> {
    fn omitted_fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Some(_) => f.write_str("Some(<omitted>)"),
            None => f.write_str("None"),
        }
    }
}
