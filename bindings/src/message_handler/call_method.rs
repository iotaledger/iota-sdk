// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{client::Client, wallet::wallet::Wallet};

use crate::{
    message_handler::{client::call_client_method_internal, wallet::call_wallet_method_internal},
    method::{ClientMethod, WalletMethod},
    panic::convert_async_panics,
    response::Response,
};

/// Call a client method.
pub async fn call_client_method(client: &Client, message: ClientMethod) -> Response {
    log::debug!("Message: {:?}", message);

    let result = convert_async_panics(|| async { call_client_method_internal(client, message).await }).await;

    let response = match result {
        Ok(r) => r,
        Err(e) => Response::Error(e),
    };

    log::debug!("Response: {:?}", response);

    response
}

/// Call a wallet method.
pub async fn call_wallet_method(wallet: &Wallet, message: WalletMethod) -> Response {
    log::debug!("Message: {:?}", message);

    let result = convert_async_panics(|| async { call_wallet_method_internal(wallet, message).await }).await;

    let response = match result {
        Ok(r) => r,
        Err(e) => Response::Error(e),
    };

    log::debug!("Response: {:?}", response);

    response
}
