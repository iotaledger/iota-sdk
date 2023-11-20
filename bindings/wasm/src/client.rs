// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, RwLock};

use iota_sdk_bindings_core::{
    call_client_method as rust_call_client_method,
    iota_sdk::client::{Client, ClientBuilder},
    ClientMethod, Response,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{binding_glue, ArrayString};

/// The Client method handler.
#[wasm_bindgen(js_name = ClientMethodHandler)]
pub struct ClientMethodHandler {
    pub(crate) inner: Arc<RwLock<Option<Client>>>,
}

impl ClientMethodHandler {
    pub(crate) fn new(client: Client) -> Self {
        Self {
            inner: Arc::new(RwLock::new(Some(client))),
        }
    }
}

/// Creates a method handler with the given client options.
#[wasm_bindgen(js_name = createClient)]
#[allow(non_snake_case)]
pub fn create_client(clientOptions: String) -> Result<ClientMethodHandler, JsError> {
    let runtime = tokio::runtime::Builder::new_current_thread().build().map_err(|err| {
        JsError::new(&serde_json::to_string(&Response::Panic(err.to_string())).expect("json to string error"))
    })?;

    let client = runtime.block_on(async move {
        ClientBuilder::new()
            .from_json(&clientOptions)
            .map_err(|err| {
                JsError::new(&serde_json::to_string(&Response::Error(err.into())).expect("json to string error"))
            })?
            .finish()
            .await
            .map_err(|err| {
                JsError::new(&serde_json::to_string(&Response::Error(err.into())).expect("json to string error"))
            })
    })?;

    Ok(ClientMethodHandler::new(client))
}

/// Necessary for compatibility with the node.js bindings.
#[wasm_bindgen(js_name = destroyClient)]
pub fn destroy_client(client_method_handler: &ClientMethodHandler) -> Result<(), JsError> {
    match client_method_handler.inner.write() {
        Ok(mut lock) => *lock = None,
        Err(e) => {
            return Err(JsError::new(
                &serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error"),
            ));
        }
    };
    Ok(())
}

/// Handles a method, returns the response as a JSON-encoded string.
///
/// Returns an error if the response itself is an error or panic.
#[wasm_bindgen(js_name = callClientMethod)]
#[allow(non_snake_case)]
pub async fn call_client_method(methodHandler: &ClientMethodHandler, method: String) -> Result<PromiseString, JsValue> {
    binding_glue!(method, methodHandler, "Client", call_client_method)
}

/// MQTT is not supported for WebAssembly bindings. 
///
/// Throws an error if called, only included for compatibility
/// with the Node.js bindings TypeScript definitions.
#[wasm_bindgen(js_name = listenMqtt)]
pub fn listen_mqtt(_topics: ArrayString, _callback: &js_sys::Function) -> Result<(), JsError> {
    Err(JsError::new(
        &serde_json::to_string(&Response::Panic(
            "Client MQTT not supported for WebAssembly".to_string(),
        ))
        .expect("json to string error"),
    ))
}
