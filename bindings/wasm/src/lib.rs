// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

pub mod client;
pub mod secret_manager;
pub mod utils;
pub mod wallet;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// Initializes the console error panic hook for better panic messages.
/// Gets automatically called when using wasm
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

/// The Wasm bindings do not support internal logging configuration yet.
///
/// Calling this will enable all rust logs to be show
#[wasm_bindgen(js_name = initLogger)]
pub async fn init_logger(_config: String) -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "string[]")]
    pub type ArrayString;

    #[wasm_bindgen(typescript_type = "Promise<string>")]
    pub type PromiseString;
}
