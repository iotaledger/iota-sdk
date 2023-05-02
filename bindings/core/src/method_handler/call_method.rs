// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(not(target_family = "wasm"))]
use std::pin::Pin;

#[cfg(not(target_family = "wasm"))]
use futures::Future;
use iota_sdk::{
    client::{secret::SecretManager, Client},
    wallet::wallet::Wallet,
};

use crate::{
    method::{ClientMethod, SecretManagerMethod, WalletMethod},
    method_handler::{
        client::call_client_method_internal, secret_manager::call_secret_manager_method_internal,
        utils::call_utils_method_internal, wallet::call_wallet_method_internal,
    },
    panic::{convert_async_panics, convert_panics},
    response::Response,
    UtilsMethod,
};

#[cfg(not(target_family = "wasm"))]
pub trait CallMethod {
    type Method;

    // This uses a manual async_trait-like impl because it's not worth it to import the lib for one trait
    fn call_method<'a>(&'a self, method: Self::Method) -> Pin<Box<dyn Future<Output = Response> + Send + 'a>>;
}

#[cfg(not(target_family = "wasm"))]
impl CallMethod for Client {
    type Method = ClientMethod;

    fn call_method<'a>(&'a self, method: Self::Method) -> Pin<Box<dyn Future<Output = Response> + Send + 'a>> {
        Box::pin(call_client_method(self, method))
    }
}

#[cfg(not(target_family = "wasm"))]
impl CallMethod for Wallet {
    type Method = WalletMethod;

    fn call_method<'a>(&'a self, method: Self::Method) -> Pin<Box<dyn Future<Output = Response> + Send + 'a>> {
        Box::pin(call_wallet_method(self, method))
    }
}

#[cfg(not(target_family = "wasm"))]
impl CallMethod for SecretManager {
    type Method = SecretManagerMethod;

    fn call_method<'a>(&'a self, method: Self::Method) -> Pin<Box<dyn Future<Output = Response> + Send + 'a>> {
        Box::pin(call_secret_manager_method(self, method))
    }
}

/// Call a client method.
pub async fn call_client_method(client: &Client, method: ClientMethod) -> Response {
    log::debug!("Client method: {method:?}");
    let result = convert_async_panics(|| async { call_client_method_internal(client, method).await }).await;

    let response = result.unwrap_or_else(Response::Error);

    log::debug!("Client response: {response:?}");
    response
}

/// Call a wallet method.
pub async fn call_wallet_method(wallet: &Wallet, method: WalletMethod) -> Response {
    log::debug!("Wallet method: {method:?}");
    let result = convert_async_panics(|| async { call_wallet_method_internal(wallet, method).await }).await;

    let response = result.unwrap_or_else(Response::Error);

    log::debug!("Wallet response: {response:?}");
    response
}

/// Call a utils method.
pub fn call_utils_method(method: UtilsMethod) -> Response {
    log::debug!("Utils method: {method:?}");
    let result = convert_panics(|| call_utils_method_internal(method));

    let response = result.unwrap_or_else(Response::Error);

    log::debug!("Utils response: {response:?}");
    response
}

/// Call a secret manager method.
pub async fn call_secret_manager_method(secret_manager: &SecretManager, method: SecretManagerMethod) -> Response {
    log::debug!("Secret manager method: {method:?}");
    let result =
        convert_async_panics(|| async { call_secret_manager_method_internal(secret_manager, method).await }).await;

    let response = result.unwrap_or_else(Response::Error);

    log::debug!("Secret manager response: {response:?}");
    response
}
