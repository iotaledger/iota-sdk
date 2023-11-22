// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_wallet_method as rust_call_wallet_method,
    iota_sdk::wallet::{events::WalletEventType, Wallet},
    Response, WalletMethod, WalletOptions,
};
use napi::{bindgen_prelude::External, threadsafe_function::ThreadsafeFunction, Error, Result, Status};
use napi_derive::napi;
use tokio::sync::RwLock;

use crate::{client::ClientMethodHandler, secret_manager::SecretManagerMethodHandler, NodejsError};

pub type WalletMethodHandler = Arc<RwLock<Option<Wallet>>>;

#[napi(js_name = "createWallet")]
pub async fn create_wallet(options: String) -> Result<External<WalletMethodHandler>> {
    let wallet_options = serde_json::from_str::<WalletOptions>(&options).map_err(NodejsError::from)?;
    let wallet = wallet_options.build().await.map_err(NodejsError::from)?;

    Ok(External::new(Arc::new(RwLock::new(Some(wallet)))))
}

#[napi(js_name = "destroyWallet")]
pub async fn destroy_wallet(wallet: External<WalletMethodHandler>) {
    *wallet.as_ref().write().await = None;
}

#[napi(js_name = "callWalletMethod")]
pub async fn call_wallet_method(wallet: External<WalletMethodHandler>, method: String) -> Result<String> {
    let wallet_method = serde_json::from_str::<WalletMethod>(&method).map_err(NodejsError::from)?;

    if let Some(wallet) = &*wallet.as_ref().read().await {
        let res = rust_call_wallet_method(wallet, wallet_method).await;
        if matches!(res, Response::Error(_) | Response::Panic(_)) {
            return Err(Error::new(
                Status::GenericFailure,
                serde_json::to_string(&res).map_err(NodejsError::from)?,
            ));
        }

        Ok(serde_json::to_string(&res).map_err(NodejsError::from)?)
    } else {
        Err(Error::new(
            Status::GenericFailure,
            serde_json::to_string(&Response::Panic("Wallet got destroyed".to_string())).map_err(NodejsError::from)?,
        ))
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
        validated_event_types.push(WalletEventType::try_from(event_type).map_err(NodejsError::from)?);
    }

    if let Some(wallet) = &*wallet.as_ref().read().await {
        wallet
            .listen(validated_event_types, move |event_data| {
                callback.call(
                    serde_json::to_string(event_data)
                        .map_err(NodejsError::from)
                        .map_err(Error::from),
                    napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
                );
            })
            .await;
        Ok(())
    } else {
        Err(Error::new(
            Status::GenericFailure,
            serde_json::to_string(&Response::Panic("Wallet got destroyed".to_string())).map_err(NodejsError::from)?,
        ))
    }
}

#[napi(js_name = "getClient")]
pub async fn get_client(wallet: External<WalletMethodHandler>) -> Result<External<ClientMethodHandler>> {
    if let Some(wallet) = &*wallet.as_ref().read().await {
        Ok(External::new(Arc::new(RwLock::new(Some(wallet.client().clone())))))
    } else {
        Err(Error::new(
            Status::GenericFailure,
            serde_json::to_string(&Response::Panic("Wallet got destroyed".to_string())).map_err(NodejsError::from)?,
        ))
    }
}

#[napi(js_name = "getSecretManager")]
pub async fn get_secret_manager(wallet: External<WalletMethodHandler>) -> Result<External<SecretManagerMethodHandler>> {
    if let Some(wallet) = &*wallet.as_ref().read().await {
        Ok(External::new(wallet.get_secret_manager().clone()))
    } else {
        Err(Error::new(
            Status::GenericFailure,
            serde_json::to_string(&Response::Panic("Wallet got destroyed".to_string())).map_err(NodejsError::from)?,
        ))
    }
}
