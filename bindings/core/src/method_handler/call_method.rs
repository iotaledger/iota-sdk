// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{client::Client, wallet::wallet::Wallet};

use crate::{
    method::{ClientMethod, WalletMethod},
    method_handler::{
        client::call_client_method_internal, utility::call_utility_method_internal, wallet::call_wallet_method_internal,
    },
    panic::convert_async_panics,
    response::Response,
    UtilityMethod,
};

/// Call a client method.
pub async fn call_client_method(client: &Client, method: ClientMethod) -> Response {
    log::debug!("Client method: {method:?}");

    let result = convert_async_panics(|| async { call_client_method_internal(client, method).await }).await;

    let response = match result {
        Ok(r) => r,
        Err(e) => Response::Error(e),
    };

    log::debug!("Client response: {response:?}");

    response
}

/// Call a wallet method.
pub async fn call_wallet_method(wallet: &Wallet, method: WalletMethod) -> Response {
    log::debug!("Wallet method: {method:?}");

    let result = convert_async_panics(|| async { call_wallet_method_internal(wallet, method).await }).await;

    let response = match result {
        Ok(r) => r,
        Err(e) => Response::Error(e),
    };

    log::debug!("Wallet response: {response:?}");

    response
}

/// Call a utility method.
pub async fn call_utility_method(method: UtilityMethod) -> Response {
    log::debug!("Utility method: {method:?}");

    let result = convert_async_panics(|| async { call_utility_method_internal(method).await }).await;

    let response = match result {
        Ok(r) => r,
        Err(e) => Response::Error(e),
    };

    log::debug!("Utility response: {response:?}");

    response
}
