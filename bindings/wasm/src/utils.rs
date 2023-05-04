// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_bindings_core::{call_utils_method as rust_call_utils_method, UtilsMethod};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// Handles a method, returns the response as a JSON-encoded string.
///
/// Returns an error if the response itself is an error or panic.
#[wasm_bindgen(js_name = callUtilsMethodRust)]
#[allow(non_snake_case)]
pub fn call_utils_method(method: String) -> Result<JsValue, JsValue> {
    let method: UtilsMethod = serde_json::from_str(&method).map_err(|err| err.to_string())?;
    let response = rust_call_utils_method(method);
    Ok(JsValue::from(
        serde_json::to_string(&response).map_err(|err| err.to_string())?,
    ))
}
