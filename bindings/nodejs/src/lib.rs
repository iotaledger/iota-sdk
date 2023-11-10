// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod client;
pub mod secret_manager;
pub mod wallet;

use iota_sdk_bindings_core::{
    call_utils_method as rust_call_utils_method, init_logger as rust_init_logger, Response, UtilsMethod,
};
use napi::{Error, Result, Status};
use napi_derive::napi;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum NodejsError {
    /// Bindings core errors.
    #[error(transparent)]
    Core(#[from] iota_sdk_bindings_core::Error),
    /// Client errors.
    #[error(transparent)]
    Client(#[from] iota_sdk_bindings_core::iota_sdk::client::Error),
    /// Mqtt errors.
    #[error(transparent)]
    Mqtt(#[from] iota_sdk_bindings_core::iota_sdk::client::node_api::mqtt::Error),
    /// SerdeJson errors.
    #[error(transparent)]
    SerdeJson(#[from] serde_json::error::Error),
    /// IO error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Wallet errors.
    #[error(transparent)]
    Wallet(#[from] iota_sdk_bindings_core::iota_sdk::wallet::Error),
}

impl From<NodejsError> for Error {
    fn from(error: NodejsError) -> Self {
        Error::new(Status::GenericFailure, error.to_string())
    }
}

#[napi(js_name = "initLogger")]
pub fn init_logger(config: String) -> Result<()> {
    match rust_init_logger(config) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::new(
            Status::GenericFailure,
            serde_json::to_string(&Response::Panic(err.to_string())).map_err(NodejsError::from)?,
        )),
    }
}

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

#[macro_export]
macro_rules! binding_glue {
    ($cx:ident, $method:ident, $method_handler:ident, $callback:ident, $name:expr) => {
        match $method_handler?.read() {
            Ok(handler) => {
                if let Some(inner) = handler.clone() {
                    crate::RUNTIME.spawn(async move {
                        let (response, is_error) = inner.call_method($method).await;
                        inner.channel.send(move |mut cx| {
                            let cb = $callback.into_inner(&mut cx);
                            let this = cx.undefined();
                    
                            let args = [
                                if is_error {
                                    cx.error(response.clone())?.upcast::<JsValue>()
                                } else {
                                    cx.undefined().upcast::<JsValue>()
                                },
                                cx.string(response).upcast::<JsValue>(),
                            ];
                    
                            cb.call(&mut cx, this, args)?;
                            Ok(())
                        });
                    }); 
                    Ok($cx.undefined())
                } else {
                    $cx.throw_error(
                        serde_json::to_string(&Response::Panic(format!("{} was destroyed", $name)))
                            .expect("json to string error"),
                    )
                }
            },
            Err(e) => $cx.throw_error(serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error")),
        }
    };
}



#[macro_export]
macro_rules! binding_glue1 {
    ($cx:ident, $method:ident, $method_handler:ident, $callback:ident, $name:expr, $code:expr) => {
        match $method_handler?.read() {
            Ok(handler) => {
                if let Some(inner) = handler.clone() {
                    crate::RUNTIME.spawn(async move {
                        $code(inner).await;
                    }); 
                    Ok($cx.undefined())
                } else {
                    $cx.throw_error(
                        serde_json::to_string(&Response::Panic(format!("{} was destroyed", $name)))
                            .expect("json to string error"),
                    )
                }
            },
            Err(e) => $cx.throw_error(serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error")),
        }
    };
}
