// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_bindings_core::{call_utils_method as rust_call_utils_method, Response, UtilsMethod};
use napi::Result;
use napi_derive::napi;

use crate::{build_js_error, NodejsError};

#[napi(js_name = "callUtilsMethodRust")]
pub fn call_utils_method(method: String) -> Result<String> {
    let method = serde_json::from_str::<UtilsMethod>(&method).map_err(NodejsError::new)?;
    let response = rust_call_utils_method(method);
    match response {
        Response::Error(_) | Response::Panic(_) => Err(build_js_error(response)),
        _ => Ok(serde_json::to_string(&response).map_err(NodejsError::new)?),
    }
}
