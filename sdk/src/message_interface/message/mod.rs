// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account_method;
mod client_message;
mod wallet_message;

pub use self::{account_method::AccountMethod, client_message::ClientMessage, wallet_message::WalletMessage};

/// The messages that can be sent to the actor.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "cmd", content = "payload", rename_all = "camelCase")]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    /// Consume a client message.
    /// Returns [`Response`](crate::message_interface::Response)
    Client(ClientMessage),
    /// Consume a wallet message.
    /// Returns [`Response`](crate::message_interface::Response)
    Wallet(WalletMessage),
}
