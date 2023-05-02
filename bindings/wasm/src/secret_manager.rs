// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use iota_sdk_bindings_core::{
    call_secret_manager_method,
    iota_sdk::client::secret::{SecretManager, SecretManagerDto},
    Response, SecretManagerMethod,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::future_to_promise;

use crate::PromiseString;

/// The SecretManager method handler.
#[wasm_bindgen(js_name = SecretManagerMethodHandler)]
pub struct SecretManagerMethodHandler {
    pub(crate) secret_manager: Rc<SecretManager>,
}

/// Creates a method handler with the given secret_manager options.
#[wasm_bindgen(js_name = createSecretManager)]
#[allow(non_snake_case)]
pub fn create_secret_manager(options: String) -> Result<SecretManagerMethodHandler, JsValue> {
    let secret_manager_dto = serde_json::from_str::<SecretManagerDto>(&options).map_err(|err| err.to_string())?;
    let secret_manager = SecretManager::try_from(&secret_manager_dto).map_err(|err| err.to_string())?;

    Ok(SecretManagerMethodHandler {
        secret_manager: Rc::new(secret_manager),
    })
}

/// Handles a method, returns the response as a JSON-encoded string.
///
/// Returns an error if the response itself is an error or panic.
#[wasm_bindgen(js_name = callSecretManagerMethodAsync)]
#[allow(non_snake_case)]
pub fn call_secret_manager_method_async(
    method: String,
    methodHandler: &SecretManagerMethodHandler,
) -> Result<PromiseString, JsValue> {
    let secret_manager: Rc<SecretManager> = Rc::clone(&methodHandler.secret_manager);

    let promise: js_sys::Promise = future_to_promise(async move {
        let method: SecretManagerMethod = serde_json::from_str(&method).map_err(|err| err.to_string())?;

        let response = call_secret_manager_method(&secret_manager, method).await;
        let ser = JsValue::from(serde_json::to_string(&response).map_err(|err| err.to_string())?);
        match response {
            Response::Error(_) | Response::Panic(_) => Err(ser),
            _ => Ok(ser),
        }
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into())
}
