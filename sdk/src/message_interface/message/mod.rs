// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account_method;
mod client_message;
mod wallet_message;

use std::fmt::{Formatter, Result as FmtResult};

pub use account_method::AccountMethod;
pub use client_message::ClientMessage;
pub use wallet_message::WalletMessage;

use crate::client::secret::SecretManagerDto;

pub(crate) trait OmittedDebug {
    fn omitted_fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<omitted>")
    }
}
impl OmittedDebug for String {}
impl OmittedDebug for SecretManagerDto {}
impl OmittedDebug for Option<SecretManagerDto> {}
