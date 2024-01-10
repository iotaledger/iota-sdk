// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_client_method as rust_call_client_method,
    iota_sdk::client::{Client, ClientBuilder},
    Response,
};
use tokio::sync::RwLock;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{build_js_error, destroyed_err, map_err, ArrayString};

/// The Client method handler.
#[wasm_bindgen(js_name = ClientMethodHandler)]
pub struct ClientMethodHandler(Arc<RwLock<Option<Client>>>);

impl ClientMethodHandler {
    pub(crate) fn new(client: Client) -> Self {
        Self(Arc::new(RwLock::new(Some(client))))
    }
}

/// Creates a method handler with the given client options.
#[wasm_bindgen(js_name = createClient)]
pub async fn create_client(options: String) -> Result<ClientMethodHandler, JsError> {
    let client = ClientBuilder::new()
        .from_json(&options)
        .map_err(map_err)?
        .finish()
        .await
        .map_err(map_err)?;

    Ok(ClientMethodHandler(Arc::new(RwLock::new(Some(client)))))
}

/// Necessary for compatibility with the node.js bindings.
#[wasm_bindgen(js_name = destroyClient)]
pub async fn destroy_client(method_handler: &ClientMethodHandler) -> Result<(), JsError> {
    method_handler.0.write().await.take();
    Ok(())
}

/// Handles a method, returns the response as a JSON-encoded string.
///
/// Returns an error if the response itself is an error or panic.
#[wasm_bindgen(js_name = callClientMethod)]
pub async fn call_client_method(method_handler: &ClientMethodHandler, method: String) -> Result<String, JsError> {
    let method = serde_json::from_str(&method).map_err(map_err)?;
    match &*method_handler.0.read().await {
        Some(client) => {
            let response = rust_call_client_method(client, method).await;
            let ser = serde_json::to_string(&response)?;
            match response {
                Response::Error(_) | Response::Panic(_) => Err(JsError::new(&ser)),
                _ => Ok(ser),
            }
        }
        None => Err(destroyed_err("Client")),
    }
}

/// MQTT is not supported for WebAssembly bindings.
///
/// Throws an error if called, only included for compatibility
/// with the Node.js bindings TypeScript definitions.
#[wasm_bindgen(js_name = listenMqtt)]
pub async fn listen_mqtt(_topics: ArrayString, _callback: &js_sys::Function) -> Result<(), JsError> {
    Err(build_js_error(Response::Panic(
        "Client MQTT not supported for WebAssembly".to_string(),
    )))
}
