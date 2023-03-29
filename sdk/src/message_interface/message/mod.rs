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

pub(crate) trait OmmittedDebug {
    fn omitted_fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<omitted>")
    }
}
impl OmmittedDebug for String {}
impl OmmittedDebug for SecretManagerDto {}
impl OmmittedDebug for Option<SecretManagerDto> {}
