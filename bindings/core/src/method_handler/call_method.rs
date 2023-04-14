// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{secret::SecretManager, Client},
    wallet::{wallet::Wallet, AccountHandle},
};

use crate::{
    method::{AccountMethod, ClientMethod, SecretManagerMethod, WalletMethod},
    method_handler::{
        account::call_account_method_internal, client::call_client_method_internal,
        secret_manager::call_secret_manager_method_internal, utils::call_utils_method_internal,
        wallet::call_wallet_method_internal,
    },
    panic::convert_async_panics,
    response::Response,
    UtilsMethod,
};

/// Call an account method.
pub async fn call_account_method(account: &AccountHandle, method: AccountMethod) -> Response {
    log::debug!("Account method: {method:?}");
    let result = convert_async_panics(|| async { call_account_method_internal(account, method).await }).await;

    let response = match result {
        Ok(r) => r,
        Err(e) => Response::Error(e),
    };

    log::debug!("Account response: {response:?}");
    response
}

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

/// Call a utils method.
pub async fn call_utils_method(method: UtilsMethod) -> Response {
    log::debug!("Utils method: {method:?}");
    let result = convert_async_panics(|| async { call_utils_method_internal(method).await }).await;

    let response = match result {
        Ok(r) => r,
        Err(e) => Response::Error(e),
    };

    log::debug!("Utils response: {response:?}");
    response
}

/// Call a secret manager method.
pub async fn call_secret_manager_method(secret_manager: &mut SecretManager, method: SecretManagerMethod) -> Response {
    log::debug!("Secret manager method: {method:?}");
    let result =
        convert_async_panics(|| async { call_secret_manager_method_internal(secret_manager, method).await }).await;

    let response = match result {
        Ok(r) => r,
        Err(e) => Response::Error(e),
    };

    log::debug!("Secret manager response: {response:?}");
    response
}
