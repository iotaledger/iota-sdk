// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_bindings_core::{call_utils_method as rust_call_utils_method, Response, UtilsMethod};
use napi::Result;
use napi_derive::napi;

use crate::NodejsError;

#[napi(js_name = "callUtilsMethodRust")]
pub fn call_utils_method(method_json: String) -> Result<String> {
    let method = match serde_json::from_str::<UtilsMethod>(&method_json) {
        Ok(method) => method,
        Err(err) => {
            return Ok(serde_json::to_string(&Response::Error(err.into())).map_err(NodejsError::from)?);
        }
    };
    let response = rust_call_utils_method(method);

    Ok(serde_json::to_string(&response).map_err(NodejsError::from)?)
}
