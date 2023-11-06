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
    /// Bindings errors.
    #[error(transparent)]
    Bindings(#[from] iota_sdk_bindings_core::Error),
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
