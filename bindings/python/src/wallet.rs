// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_wallet_method as rust_call_wallet_method,
    iota_sdk::wallet::{events::types::WalletEventType, Wallet as RustWallet},
    WalletMethod, WalletOptions,
};
use pyo3::prelude::*;
use tokio::sync::RwLock;

use crate::error::Result;

#[pyclass]
pub struct Wallet {
    pub wallet: Arc<RwLock<Option<RustWallet>>>,
}

/// Destroys the wallet instance.
#[pyfunction]
pub fn destroy_wallet(wallet: &Wallet) -> PyResult<()> {
    crate::block_on(async {
        *wallet.wallet.write().await = None;
    });
    Ok(())
}

/// Create wallet handler for python-side usage.
#[pyfunction]
pub fn create_wallet(options: String) -> Result<Wallet> {
    let wallet_options = serde_json::from_str::<WalletOptions>(&options)?;
    let wallet = crate::block_on(async { wallet_options.build_manager().await })?;

    Ok(Wallet {
        wallet: Arc::new(RwLock::new(Some(wallet))),
    })
}

/// Call a wallet method.
#[pyfunction]
pub fn call_wallet_method(wallet: &Wallet, method: String) -> Result<String> {
    let method = serde_json::from_str::<WalletMethod>(&method)?;
    let response = crate::block_on(async {
        rust_call_wallet_method(
            wallet.wallet.read().await.as_ref().expect("wallet got destroyed"),
            method,
        )
        .await
    });

    Ok(serde_json::to_string(&response)?)
}

/// Listen to wallet events.
#[pyfunction]
pub fn listen_wallet(wallet: &Wallet, events: Vec<String>, handler: PyObject) {
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
        wallet
            .wallet
            .read()
            .await
            .as_ref()
            .expect("wallet got destroyed")
            .listen(rust_events, move |_| {
                Python::with_gil(|py| {
                    handler.call0(py).unwrap();
                });
            })
            .await;
    });
}
