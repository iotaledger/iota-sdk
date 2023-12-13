// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_wallet_method as rust_call_wallet_method,
    iota_sdk::wallet::{events::WalletEventType, Wallet},
    Response, WalletMethod, WalletOptions,
};
use napi::{bindgen_prelude::External, threadsafe_function::ThreadsafeFunction, Error, Result};
use napi_derive::napi;
use tokio::sync::RwLock;

use crate::{
    build_js_error, client::ClientMethodHandler, destroyed_err, secret_manager::SecretManagerMethodHandler, NodejsError,
};

pub type WalletMethodHandler = Arc<RwLock<Option<Wallet>>>;

#[napi(js_name = "createWallet")]
pub async fn create_wallet(options: String) -> Result<External<WalletMethodHandler>> {
    let wallet_options = serde_json::from_str::<WalletOptions>(&options).map_err(NodejsError::new)?;
    let wallet = wallet_options.build().await.map_err(NodejsError::new)?;

    Ok(External::new(Arc::new(RwLock::new(Some(wallet)))))
}

#[napi(js_name = "destroyWallet")]
pub async fn destroy_wallet(wallet: External<WalletMethodHandler>) {
    *wallet.as_ref().write().await = None;
}

#[napi(js_name = "callWalletMethod")]
pub async fn call_wallet_method(wallet: External<WalletMethodHandler>, method: String) -> Result<String> {
    let method = serde_json::from_str::<WalletMethod>(&method).map_err(NodejsError::new)?;

    match &*wallet.as_ref().read().await {
        Some(wallet) => {
            let response = rust_call_wallet_method(&wallet, method).await;
            match response {
                Response::Error(_) | Response::Panic(_) => Err(build_js_error(response)),
                _ => Ok(serde_json::to_string(&response).map_err(NodejsError::new)?),
            }
        }
        None => Err(destroyed_err("Wallet")),
    }
}

#[napi(js_name = "listenWallet")]
pub async fn listen_wallet(
    wallet: External<WalletMethodHandler>,
    event_types: Vec<u8>,
    callback: ThreadsafeFunction<String>,
) -> Result<()> {
    let mut validated_event_types = Vec::with_capacity(event_types.len());
    for event_type in event_types {
        validated_event_types.push(WalletEventType::try_from(event_type).map_err(NodejsError::new)?);
    }

    match &*wallet.as_ref().read().await {
        Some(wallet) => {
            wallet
                .listen(validated_event_types, move |event_data| {
                    callback.call(
                        serde_json::to_string(event_data)
                            .map_err(NodejsError::new)
                            .map_err(Error::from),
                        napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
                    );
                })
                .await;
            Ok(())
        }
        None => Err(destroyed_err("Wallet")),
    }
}

#[napi(js_name = "getClient")]
pub async fn get_client(wallet: External<WalletMethodHandler>) -> Result<External<ClientMethodHandler>> {
    if let Some(wallet) = &*wallet.as_ref().read().await {
        Ok(External::new(Arc::new(RwLock::new(Some(wallet.client().clone())))))
    } else {
        Err(destroyed_err("Wallet"))
    }
}

#[napi(js_name = "getSecretManager")]
pub async fn get_secret_manager(wallet: External<WalletMethodHandler>) -> Result<External<SecretManagerMethodHandler>> {
    if let Some(wallet) = &*wallet.as_ref().read().await {
        Ok(External::new(wallet.get_secret_manager().clone()))
    } else {
        Err(destroyed_err("Wallet"))
    }
}
