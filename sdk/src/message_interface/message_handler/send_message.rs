// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::Client,
    message_interface::{
        message::{ClientMessage, WalletMessage},
        panic::convert_async_panics,
        response::Response,
    },
    wallet::account_manager::AccountManager,
};

impl Client {
    /// Send a message.
    pub async fn send_message(&self, message: ClientMessage) -> Response {
        log::debug!("Message: {:?}", message);

        let result = convert_async_panics(|| async { self.handle_message(message).await }).await;

        let response = match result {
            Ok(r) => r,
            Err(e) => Response::Error(e),
        };

        log::debug!("Response: {:?}", response);

        response
    }
}

impl AccountManager {
    /// Send a message.
    pub async fn send_message(&self, message: WalletMessage) -> Response {
        log::debug!("Message: {:?}", message);

        let result = convert_async_panics(|| async { self.handle_message(message).await }).await;

        let response = match result {
            Ok(r) => r,
            Err(e) => Response::Error(e),
        };

        log::debug!("Response: {:?}", response);

        response
    }
}
