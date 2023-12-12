// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod client;
pub mod error;
pub mod secret_manager;
pub mod utils;
pub mod wallet;

use iota_sdk_bindings_core::{init_logger as rust_init_logger, Response};
use napi::{Error, Result, Status};
use napi_derive::napi;

pub use self::error::NodejsError;

#[napi(js_name = "initLogger")]
pub fn init_logger(config: String) -> Result<()> {
    match rust_init_logger(config) {
        Ok(_) => Ok(()),
        Err(err) => Err(build_js_error(Response::Panic(err.to_string()))),
    }
}

// Util fn for making the "X was destroyed" error message.
pub(crate) fn destroyed_err(instance: &str) -> Error {
    build_js_error(Response::Panic(format!("{} was destroyed", instance)))
}

// Serializes a bindings response and puts it in a napi Error.
pub(crate) fn build_js_error(response: Response) -> Error {
    Error::new(
        Status::GenericFailure,
        serde_json::to_string(&response).expect("json to string error"),
    )
}
