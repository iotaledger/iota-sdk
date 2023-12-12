// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_wallet_method as rust_call_wallet_method,
    iota_sdk::wallet::{
        events::types::{WalletEvent, WalletEventType},
        Wallet,
    },
    Response, WalletOptions,
};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    RwLock,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

use crate::{client::ClientMethodHandler, destroyed_err, map_err, secret_manager::SecretManagerMethodHandler};

/// The Wallet method handler.
#[wasm_bindgen(js_name = WalletMethodHandler)]
pub struct WalletMethodHandler(Arc<RwLock<Option<Wallet>>>);

/// Creates a method handler with the given options.
#[wasm_bindgen(js_name = createWallet)]
#[allow(non_snake_case)]
pub async fn create_wallet(options: String) -> Result<WalletMethodHandler, JsError> {
    let wallet_options: WalletOptions = serde_json::from_str::<WalletOptions>(&options).map_err(map_err)?;
    let wallet_method_handler = wallet_options.build().await.map_err(map_err)?;

    Ok(WalletMethodHandler(Arc::new(RwLock::new(Some(wallet_method_handler)))))
}

#[wasm_bindgen(js_name = destroyWallet)]
pub async fn destroy_wallet(method_handler: &WalletMethodHandler) -> Result<(), JsError> {
    let mut lock = method_handler.0.write().await;
    if let Some(_) = &*lock {
        *lock = None;
    }

    // If None, was already destroyed
    Ok(())
}

#[wasm_bindgen(js_name = getClient)]
pub async fn get_client(method_handler: &WalletMethodHandler) -> Result<ClientMethodHandler, JsError> {
    if let Some(wallet) = &*method_handler.0.read().await {
        Ok(ClientMethodHandler::new(wallet.client().clone()))
    } else {
        // Notify that the wallet was destroyed
        Err(destroyed_err("Wallet"))
    }
}

#[wasm_bindgen(js_name = getSecretManager)]
pub async fn get_secret_manager(method_handler: &WalletMethodHandler) -> Result<SecretManagerMethodHandler, JsError> {
    if let Some(wallet) = &*method_handler.0.read().await {
        Ok(SecretManagerMethodHandler::new(wallet.get_secret_manager().clone()))
    } else {
        // Notify that the wallet was destroyed
        Err(destroyed_err("Wallet"))
    }
}

/// Handles a method, returns the response as a JSON-encoded string.
///
/// Returns an error if the response itself is an error or panic.
#[wasm_bindgen(js_name = callWalletMethod)]
pub async fn call_wallet_method(method_handler: &WalletMethodHandler, method: String) -> Result<String, JsError> {
    let method = serde_json::from_str(&method).map_err(map_err)?;
    match &*method_handler.0.read().await {
        Some(wallet) => {
            let response = rust_call_wallet_method(&wallet, method).await;
            let ser = serde_json::to_string(&response)?;
            match response {
                Response::Error(_) | Response::Panic(_) => Err(JsError::new(&ser)),
                _ => Ok(ser),
            }
        }
        None => Err(destroyed_err("Wallet")),
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
) -> Result<JsValue, JsError> {
    let mut event_types = Vec::with_capacity(vec.length() as _);
    for event_type in vec.keys() {
        // We know the built-in iterator for set elements won't throw
        // exceptions, so just unwrap the element.
        let event_type = event_type.unwrap().as_f64().unwrap() as u8;
        let wallet_event_type = WalletEventType::try_from(event_type).map_err(map_err)?;
        event_types.push(wallet_event_type);
    }

    if let Some(wallet) = &*method_handler.0.read().await {
        let (tx, mut rx): (UnboundedSender<WalletEvent>, UnboundedReceiver<WalletEvent>) = unbounded_channel();
        wallet
            .listen(event_types, move |wallet_event| {
                tx.send(wallet_event.clone()).unwrap();
            })
            .await;

        // Spawn on the same thread a continuous loop to check the channel
        wasm_bindgen_futures::spawn_local(async move {
            while let Some(wallet_event) = rx.recv().await {
                let res = callback.call2(
                    &JsValue::NULL,
                    &JsValue::UNDEFINED,
                    &JsValue::from(serde_json::to_string(&wallet_event).unwrap()),
                );
                // Call callback again with the error this time, to prevent wasm crashing.
                // This does mean the callback is called a second time instead of once.
                if let Err(e) = res {
                    callback.call2(&JsValue::NULL, &e, &JsValue::UNDEFINED).unwrap();
                }
            }
            // No more links to the unbounded_channel, exit loop
        });
        Ok(JsValue::UNDEFINED)
    } else {
        // Notify that the wallet was destroyed
        Err(destroyed_err("Wallet"))
    }
}
