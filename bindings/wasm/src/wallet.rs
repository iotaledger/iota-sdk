// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, RwLock};

use iota_sdk_bindings_core::{
    call_wallet_method,
    iota_sdk::wallet::{
        events::types::{Event, WalletEventType},
        Wallet,
    },
    Response, WalletMethod, WalletOptions,
};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

use crate::{client::ClientMethodHandler, secret_manager::SecretManagerMethodHandler};

/// The Wallet method handler.
#[wasm_bindgen(js_name = WalletMethodHandler)]
pub struct WalletMethodHandler {
    wallet: Arc<RwLock<Option<Wallet>>>,
}

macro_rules! wallet_pre {
    ($method_handler:ident) => {
        match $method_handler.wallet.read() {
            Ok(handler) => {
                if let Some(wallet) = handler.clone() {
                    Ok(wallet)
                } else {
                    // Notify that the wallet was destroyed
                    Err(JsError::new(
                        &serde_json::to_string(&Response::Panic("Wallet was destroyed".to_string()))
                            .expect("json to string error"),
                    ))
                }
            }
            Err(e) => Err(JsError::new(
                &serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error"),
            )),
        }
    };
}

/// Creates a method handler with the given options.
#[wasm_bindgen(js_name = createWallet)]
#[allow(non_snake_case)]
pub fn create_wallet(options: String) -> Result<WalletMethodHandler, JsError> {
    let wallet_options = serde_json::from_str::<WalletOptions>(&options).map_err(|e| {
        JsError::new(&serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error"))
    })?;

    let wallet_method_handler = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async move { wallet_options.build().await })
        .map_err(|e| {
            JsError::new(&serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error"))
        })?;

    Ok(WalletMethodHandler {
        wallet: Arc::new(RwLock::new(Some(wallet_method_handler))),
    })
}

#[wasm_bindgen(js_name = destroyWallet)]
pub fn destroy_wallet(method_handler: &WalletMethodHandler) -> Result<(), JsError> {
    match method_handler.wallet.write() {
        Ok(mut lock) => *lock = None,
        Err(e) => {
            return Err(JsError::new(
                &serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error"),
            ));
        }
    };
    Ok(())
}

#[wasm_bindgen(js_name = getClientFromWallet)]
pub fn get_client(method_handler: &WalletMethodHandler) -> Result<ClientMethodHandler, JsError> {
    let wallet = wallet_pre!(method_handler)?;

    let client = wallet.client().clone();

    Ok(ClientMethodHandler::new(client))
}

#[wasm_bindgen(js_name = getSecretManagerFromWallet)]
pub fn get_secret_manager(method_handler: &WalletMethodHandler) -> Result<SecretManagerMethodHandler, JsError> {
    let wallet = wallet_pre!(method_handler)?;
    let mngr = wallet.get_secret_manager().clone();

    Ok(SecretManagerMethodHandler::new(mngr))
}

/// Handles a method, returns the response as a JSON-encoded string.
///
/// Returns an error if the response itself is an error or panic.
#[wasm_bindgen(js_name = callWalletMethodAsync)]
pub async fn call_wallet_method_async(method: String, method_handler: &WalletMethodHandler) -> Result<String, JsError> {
    let wallet = wallet_pre!(method_handler)?;
    let method: WalletMethod = serde_json::from_str(&method).map_err(|err| JsError::new(&err.to_string()))?;

    let response = call_wallet_method(&wallet, method).await;
    match response {
        Response::Error(e) => Err(JsError::new(
            &serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error"),
        )),
        Response::Panic(p) => Err(JsError::new(&p)),
        _ => Ok(serde_json::to_string(&response).map_err(|e| JsError::new(&e.to_string()))?),
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
#[wasm_bindgen(js_name = listenWalletAsync)]
pub async fn listen_wallet(
    vec: js_sys::Array,
    callback: js_sys::Function,
    method_handler: &WalletMethodHandler,
) -> Result<JsValue, JsError> {
    let mut event_types = Vec::with_capacity(vec.length() as _);
    for event_type in vec.keys() {
        // We know the built-in iterator for set elements won't throw
        // exceptions, so just unwrap the element.
        let event_type = event_type.unwrap().as_f64().unwrap() as u8;
        let wallet_event_type = WalletEventType::try_from(event_type).map_err(|e| {
            JsError::new(&serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error"))
        })?;
        event_types.push(wallet_event_type);
    }

    let (tx, mut rx): (UnboundedSender<Event>, UnboundedReceiver<Event>) = unbounded_channel();
    let wallet = wallet_pre!(method_handler)?;
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
}

/// Rocksdb chrysalis migration is not supported for WebAssembly bindings.
///
/// Throws an error if called, only included for compatibility
/// with the Node.js bindings TypeScript definitions.
#[wasm_bindgen(js_name = migrateDbChrysalisToStardust)]
pub fn migrate_db_chrysalis_to_stardust(_storage_path: String, _password: Option<String>) -> Result<(), JsValue> {
    let js_error = js_sys::Error::new("Rocksdb chrysalis migration is not supported for WebAssembly");

    Err(JsValue::from(js_error))
}
