// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! # Python binding implementation for the iota-sdk library.

mod client;
mod error;
mod secret_manager;
mod wallet;

use std::sync::Mutex;

use iota_sdk_bindings_core::{
    call_utils_method as rust_call_utils_method, init_logger as rust_init_logger, UtilsMethod,
};
use once_cell::sync::OnceCell;
use pyo3::{prelude::*, wrap_pyfunction};
use tokio::runtime::Runtime;

use self::{client::*, error::Result, secret_manager::*, wallet::*};

/// Use one runtime.
pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
    static INSTANCE: OnceCell<Mutex<Runtime>> = OnceCell::new();
    let runtime = INSTANCE.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
    runtime.lock().unwrap().block_on(cb)
}

#[pyfunction]
/// Init the logger of wallet library.
pub fn init_logger(config: String) -> PyResult<()> {
    rust_init_logger(config).expect("failed to init logger");
    Ok(())
}

#[pyfunction]
pub fn call_utils_method(method: String) -> Result<String> {
    let method = serde_json::from_str::<UtilsMethod>(&method)?;
    let response = crate::block_on(async { rust_call_utils_method(method).await });
    Ok(serde_json::to_string(&response)?)
}

/// IOTA SDK implemented in Rust for Python binding.
#[pymodule]
fn iota_sdk(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_logger, m)?).unwrap();

    m.add_function(wrap_pyfunction!(call_utils_method, m)?).unwrap();

    m.add_function(wrap_pyfunction!(create_client, m)?).unwrap();
    m.add_function(wrap_pyfunction!(call_client_method, m)?).unwrap();

    m.add_function(wrap_pyfunction!(create_secret_manager, m)?).unwrap();
    m.add_function(wrap_pyfunction!(call_secret_manager_method, m)?)
        .unwrap();

    m.add_function(wrap_pyfunction!(create_wallet, m)?).unwrap();
    m.add_function(wrap_pyfunction!(call_wallet_method, m)?).unwrap();
    m.add_function(wrap_pyfunction!(destroy_wallet, m)?).unwrap();
    m.add_function(wrap_pyfunction!(listen_wallet, m)?).unwrap();

    Ok(())
}
