// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

pub mod client;
pub mod secret_manager;
pub mod utils;
pub mod wallet;

use wasm_bindgen::{prelude::wasm_bindgen, JsError};

/// Initializes the console error panic hook for better panic messages.
/// Gets automatically called when using wasm
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsError> {
    console_error_panic_hook::set_once();
    Ok(())
}

/// The Wasm bindings do not support internal logging configuration yet.
///
/// Calling this will enable all rust logs to be show
#[wasm_bindgen(js_name = initLogger)]
pub async fn init_logger(_config: String) -> Result<(), JsError> {
    wasm_logger::init(wasm_logger::Config::default());
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "string[]")]
    pub type ArrayString;
}

// Maps a bindings error into the proper js error
pub(crate) fn map_err<T>(err: T) -> JsError
where
    T: Into<iota_sdk_bindings_core::Error>,
{
    build_js_error(iota_sdk_bindings_core::Response::Error(err.into()))
}

pub(crate) fn destroyed_err(instance: &str) -> JsError {
    build_js_error(iota_sdk_bindings_core::Response::Panic(format!(
        "{} was destroyed",
        instance
    )))
}

// Serializes a bindings response and puts it in a JsError
pub(crate) fn build_js_error<T>(response: T) -> JsError
where
    T: Into<iota_sdk_bindings_core::Response>,
{
    JsError::new(&serde_json::to_string(&response.into()).expect("json to string error"))
}
