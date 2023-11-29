// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_wallet_method as rust_call_wallet_method,
    iota_sdk::wallet::{
        events::types::{WalletEvent, WalletEventType},
        Wallet,
    },
    Response, WalletMethod, WalletOptions,
};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    Mutex,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{client::ClientMethodHandler, secret_manager::SecretManagerMethodHandler};

/// The Wallet method handler.
#[wasm_bindgen(js_name = WalletMethodHandler)]
pub struct WalletMethodHandler {
    inner: Arc<Mutex<Option<(Wallet, SecretManagerMethodHandler)>>>,
}

/// Creates a method handler with the given options.
#[wasm_bindgen(js_name = createWallet)]
#[allow(non_snake_case)]
pub async fn create_wallet(
    options: String,
    secret_manager: &SecretManagerMethodHandler,
) -> Result<WalletMethodHandler, JsValue> {
    let wallet_options = serde_json::from_str::<WalletOptions>(&options).map_err(|e| e.to_string())?;

    let wallet_method_handler = wallet_options
        .build(&secret_manager.inner)
        .await
        .map_err(|e| e.to_string())?;

    Ok(WalletMethodHandler {
        inner: Arc::new(Mutex::new(Some((wallet_method_handler, secret_manager.clone())))),
    })
}

#[wasm_bindgen(js_name = destroyWallet)]
pub async fn destroy_wallet(method_handler: &WalletMethodHandler) -> Result<(), JsValue> {
    *method_handler.inner.lock().await = None;
    Ok(())
}

#[wasm_bindgen(js_name = getClientFromWallet)]
pub async fn get_client(method_handler: &WalletMethodHandler) -> Result<ClientMethodHandler, JsValue> {
    let lock = method_handler.inner.lock().await;

    let client = lock
        .as_ref()
        .ok_or_else(|| "wallet got destroyed".to_string())?
        .0
        .client()
        .clone();

    Ok(ClientMethodHandler { client })
}

#[wasm_bindgen(js_name = getSecretManagerFromWallet)]
pub async fn get_secret_manager(method_handler: &WalletMethodHandler) -> Result<SecretManagerMethodHandler, JsValue> {
    let lock = method_handler.inner.lock().await;

    let secret_manager = lock
        .as_ref()
        .ok_or_else(|| "wallet got destroyed".to_string())?
        .1
        .clone();

    Ok(secret_manager)
}

/// Handles a method, returns the response as a JSON-encoded string.
///
/// Returns an error if the response itself is an error or panic.
#[wasm_bindgen(js_name = callWalletMethod)]
pub async fn call_wallet_method(method_handler: &WalletMethodHandler, method: String) -> Result<String, JsValue> {
    let lock = method_handler.inner.lock().await;
    let method: WalletMethod = serde_json::from_str(&method).map_err(|err| err.to_string())?;
    let (wallet, secret_manager) = lock.as_ref().expect("wallet got destroyed");

    let response = rust_call_wallet_method(wallet, &secret_manager.inner, method).await;
    match response {
        Response::Error(e) => Err(e.to_string().into()),
        Response::Panic(p) => Err(p.into()),
        _ => Ok(serde_json::to_string(&response).map_err(|e| e.to_string())?),
    }
}

/// It takes a list of event types, registers a callback function, and then listens for events of those
/// types
///
/// Arguments:
///
/// * `vec`: An array of strings that represent the event types you want to listen to.
/// * `callback`: A JavaScript function that will be called when a wallet event occurs.
/// * `method_handler`: This is the same method handler that we used in the previous section.
#[wasm_bindgen(js_name = listenWallet)]
pub async fn listen_wallet(
    method_handler: &WalletMethodHandler,
    vec: js_sys::Array,
    callback: js_sys::Function,
) -> Result<JsValue, JsValue> {
    let mut event_types = Vec::with_capacity(vec.length() as _);
    for event_type in vec.keys() {
        // We know the built-in iterator for set elements won't throw
        // exceptions, so just unwrap the element.
        let event_type = event_type.unwrap().as_f64().unwrap() as u8;
        let wallet_event_type = WalletEventType::try_from(event_type).map_err(|err| err.to_string())?;
        event_types.push(wallet_event_type);
    }

    let (tx, mut rx): (UnboundedSender<WalletEvent>, UnboundedReceiver<WalletEvent>) = unbounded_channel();
    method_handler
        .inner
        .lock()
        .await
        .as_ref()
        .expect("wallet not initialised")
        .0
        .listen(event_types, move |wallet_event| {
            tx.send(wallet_event.clone()).unwrap();
        })
        .await;

    // Spawn on the same thread a continuous loop to check the channel
    wasm_bindgen_futures::spawn_local(async move {
        while let Some(wallet_event) = rx.recv().await {
            callback
                .call1(
                    &JsValue::NULL,
                    &JsValue::from(serde_json::to_string(&wallet_event).unwrap()),
                )
                // Safe to unwrap, our callback has no return
                .unwrap();
        }
        // No more links to the unbounded_channel, exit loop
    });

    Ok(JsValue::UNDEFINED)
}
