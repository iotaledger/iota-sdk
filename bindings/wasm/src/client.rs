// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_bindings_core::{
    call_client_method,
    iota_sdk::client::{Client, ClientBuilder},
    ClientMethod, Response,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::future_to_promise;

use crate::{ArrayString, PromiseString};

/// The Client method handler.
#[wasm_bindgen(js_name = ClientMethodHandler)]
pub struct ClientMethodHandler {
    pub(crate) client: Client,
}

/// Creates a method handler with the given client options.
#[wasm_bindgen(js_name = createClient)]
#[allow(non_snake_case)]
pub fn create_client(clientOptions: String) -> Result<ClientMethodHandler, JsValue> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .map_err(|err| err.to_string())?;

    let client = runtime.block_on(async move {
        ClientBuilder::new()
            .from_json(&clientOptions)
            .map_err(|err| err.to_string())?
            .finish()
            .await
            .map_err(|err| err.to_string())
    })?;

    Ok(ClientMethodHandler { client })
}

/// Handles a method, returns the response as a JSON-encoded string.
///
/// Returns an error if the response itself is an error or panic.
#[wasm_bindgen(js_name = callClientMethodAsync)]
#[allow(non_snake_case)]
pub fn call_client_method_async(method: String, methodHandler: &ClientMethodHandler) -> Result<PromiseString, JsValue> {
    let client: Client = methodHandler.client.clone();

    let promise: js_sys::Promise = future_to_promise(async move {
        let method: ClientMethod = serde_json::from_str(&method).map_err(|err| err.to_string())?;

        let response = call_client_method(&client, method).await;
        let ser = JsValue::from(serde_json::to_string(&response).map_err(|err| err.to_string())?);
        match response {
            Response::Error(_) | Response::Panic(_) => Err(ser),
            _ => Ok(ser),
        }
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into())
}

/// MQTT is not supported for WebAssembly bindings.
///
/// Throws an error if called, only included for compatibility
/// with the Node.js bindings TypeScript definitions.
#[wasm_bindgen(js_name = listenMqtt)]
pub fn listen_mqtt(_topics: ArrayString, _callback: &js_sys::Function) -> Result<(), JsValue> {
    let js_error = js_sys::Error::new("Client MQTT not supported for WebAssembly");

    Err(JsValue::from(js_error))
}
