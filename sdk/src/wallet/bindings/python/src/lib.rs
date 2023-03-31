// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod types;

use std::sync::Mutex;

use ::iota_sdk::{
    client::stronghold::StrongholdAdapter,
    wallet::{
        events::types::WalletEventType,
        message_interface::{init_logger as init_logger_rust, ManagerOptions, Message},
    },
};
use once_cell::sync::OnceCell;
use pyo3::{prelude::*, wrap_pyfunction};
use tokio::runtime::Runtime;

use self::types::*;

/// Use one runtime.
pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
    static INSTANCE: OnceCell<Mutex<Runtime>> = OnceCell::new();
    let runtime = INSTANCE.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
    runtime.lock().unwrap().block_on(cb)
}

/// Init the logger of wallet library.
#[pyfunction]
pub fn init_logger(config: String) -> PyResult<()> {
    init_logger_rust(config).expect("failed to init logger");
    Ok(())
}

/// Destroys the wallet instance.
/// Currently has no actual effect
#[pyfunction]
pub fn destroy() -> PyResult<()> {
    // Nothing to do here, but added for consistency across bindings
    Ok(())
}

/// Create message handler for python-side usage.
#[pyfunction]
pub fn create_message_handler(options: Option<String>) -> Result<WalletMessageHandler> {
    let options = match options {
        Some(ops) => match serde_json::from_str::<ManagerOptions>(&ops) {
            Ok(options) => Some(options),
            Err(e) => {
                panic!("Wrong options input! {e:?}");
            }
        },
        _ => None,
    };
    let message_handler =
        crate::block_on(async { ::iota_sdk::wallet::message_interface::create_message_handler(options).await })?;

    Ok(WalletMessageHandler {
        wallet_message_handler: message_handler,
    })
}

/// Send message through handler.
#[pyfunction]
pub fn send_message(handle: &WalletMessageHandler, message: String) -> Result<String> {
    let message = match serde_json::from_str::<Message>(&message) {
        Ok(message) => message,
        Err(e) => {
            panic!("Wrong message! {e:?}");
        }
    };
    let response = crate::block_on(async { handle.wallet_message_handler.send_message(message).await });

    Ok(serde_json::to_string(&response)?)
}

/// Listen to events.
#[pyfunction]
pub fn listen(handle: &WalletMessageHandler, events: Vec<String>, handler: PyObject) {
    let mut rust_events = Vec::new();

    for event in events {
        let event = match serde_json::from_str::<WalletEventType>(&event) {
            Ok(event) => event,
            Err(e) => {
                panic!("Wrong event to listen! {e:?}");
            }
        };
        rust_events.push(event);
    }

    crate::block_on(async {
        handle
            .wallet_message_handler
            .listen(rust_events, move |_| {
                Python::with_gil(|py| {
                    handler.call0(py).unwrap();
                });
            })
            .await;
    });
}

/// Migrates a stronghold snapshot from v2 to v3.
#[pyfunction]
pub fn migrate_stronghold_snapshot_v2_to_v3(
    current_path: String,
    current_password: String,
    new_path: Option<String>,
    new_password: Option<String>,
) -> Result<()> {
    StrongholdAdapter::migrate_v2_to_v3(
        &current_path,
        &current_password,
        new_path.as_ref(),
        new_password.as_deref(),
    )?;

    Ok(())
}

/// IOTA Wallet implemented in Rust for Python binding.
#[pymodule]
fn iota_wallet(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_message_handler, m)?).unwrap();
    m.add_function(wrap_pyfunction!(destroy, m)?).unwrap();
    m.add_function(wrap_pyfunction!(init_logger, m)?).unwrap();
    m.add_function(wrap_pyfunction!(listen, m)?).unwrap();
    m.add_function(wrap_pyfunction!(send_message, m)?).unwrap();
    m.add_function(wrap_pyfunction!(migrate_stronghold_snapshot_v2_to_v3, m)?)
        .unwrap();

    Ok(())
}
