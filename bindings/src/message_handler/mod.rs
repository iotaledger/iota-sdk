// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account_handle;
mod client;
mod send_message;
mod wallet;

#[cfg(feature = "mqtt")]
pub use client::listen_mqtt;
pub use send_message::{call_client_method, call_wallet_method};

/// Result type of the message interface.
pub type Result<T> = std::result::Result<T, super::error::MessageInterfaceError>;

#[cfg(test)]
mod tests {
    use super::super::{panic::convert_async_panics, Response};

    #[tokio::test]
    async fn panic_to_response() {
        match convert_async_panics(|| async { panic!("rekt") }).await.unwrap() {
            Response::Panic(msg) => {
                assert!(msg.contains("rekt"));
            }
            response_type => panic!("Unexpected response type: {response_type:?}"),
        };
    }
}
