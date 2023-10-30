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



#[macro_export]
macro_rules! binding_glue {
    ($method:ident, $method_handler:ident, $name:expr, $code:expr) => {
        match $method_handler.inner.read() {
            Ok(handler) => {
                let method = serde_json::from_str(&$method).map_err(|err| {
                    JsError::new(&serde_json::to_string(&Response::Error(err.into())).expect("json to string error"))
                })?;
                if let Some(inner) = &*handler {
                    let response = $code(inner, method).await;
                    let ser = serde_json::to_string(&response).expect("json to string error");
                    match response {
                        Response::Error(_) | Response::Panic(_) => Err(JsError::new(&ser)),
                        _ => Ok(ser),
                    }
                } else {
                    // Notify that the inner object was destroyed
                    Err(JsError::new(
                        &serde_json::to_string(&Response::Panic(format!("{} was destroyed", $name)))
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