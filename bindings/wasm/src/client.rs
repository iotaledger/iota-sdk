// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, RwLock};

use iota_sdk_bindings_core::{
    call_client_method,
    iota_sdk::client::{Client, ClientBuilder},
    ClientMethod, Response,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::ArrayString;

/// The Client method handler.
#[wasm_bindgen(js_name = ClientMethodHandler)]
pub struct ClientMethodHandler {
    pub(crate) client: Arc<RwLock<Option<Client>>>,
}

impl ClientMethodHandler {
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client: Arc::new(RwLock::new(Some(client))),
        }
    }
}

macro_rules! client_pre {
    ($method_handler:ident) => {
        match $method_handler.client.read() {
            Ok(handler) => {
                if let Some(client) = handler.clone() {
                    Ok(client)
                } else {
                    // Notify that the client was destroyed
                    Err(JsError::new(
                        &serde_json::to_string(&Response::Panic("Client was destroyed".to_string()))
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
                JsError::new(&serde_json::to_string(&Response::Panic(err.to_string())).expect("json to string error"))
            })?
            .finish()
            .await
            .map_err(|err| {
                JsError::new(&serde_json::to_string(&Response::Panic(err.to_string())).expect("json to string error"))
            })
    })?;

    Ok(ClientMethodHandler::new(client))
}

/// Necessary for compatibility with the node.js bindings.
#[wasm_bindgen(js_name = destroyClient)]
pub fn destroy_client(client_method_handler: &ClientMethodHandler) -> Result<(), JsError> {
    match client_method_handler.client.write() {
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
#[wasm_bindgen(js_name = callClientMethodAsync)]
#[allow(non_snake_case)]
pub async fn call_client_method_async(method: String, methodHandler: &ClientMethodHandler) -> Result<String, JsError> {
    let client = client_pre!(methodHandler)?;

    let method: ClientMethod = serde_json::from_str(&method).map_err(|err| {
        JsError::new(&serde_json::to_string(&Response::Panic(err.to_string())).expect("json to string error"))
    })?;

    let response = call_client_method(&client, method).await;
    let ser = serde_json::to_string(&response).expect("json to string error");
    match response {
        Response::Error(_) | Response::Panic(_) => Err(JsError::new(&ser)),
        _ => Ok(ser),
    }
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
