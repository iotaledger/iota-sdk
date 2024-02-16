// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_wallet_method as rust_call_wallet_method,
    iota_sdk::wallet::{events::types::WalletEventType, Wallet as RustWallet},
    Response, WalletMethod, WalletOptions,
};
use pyo3::{prelude::*, types::PyTuple};
use tokio::sync::RwLock;

use crate::{
    client::Client,
    error::{Error, Result},
    SecretManager,
};

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
    let wallet = crate::block_on(async { wallet_options.build().await })?;

    Ok(Wallet {
        wallet: Arc::new(RwLock::new(Some(wallet))),
    })
}

/// Call a wallet method.
#[pyfunction]
pub fn call_wallet_method(wallet: &Wallet, method: String) -> Result<String> {
    let method = serde_json::from_str::<WalletMethod>(&method)?;
    let response = crate::block_on(async {
        match wallet.wallet.read().await.as_ref() {
            Some(wallet) => rust_call_wallet_method(wallet, method).await,
            None => Response::Panic("wallet was destroyed".into()),
        }
    });

    Ok(serde_json::to_string(&response)?)
}

/// Listen to wallet events.
#[pyfunction]
pub fn listen_wallet(wallet: &Wallet, events: Vec<u8>, handler: PyObject) {
    let mut rust_events = Vec::with_capacity(events.len());

    for event in events {
        let event = match WalletEventType::try_from(event) {
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
            .expect("wallet was destroyed")
            .listen(rust_events, move |event| {
                let event_string = serde_json::to_string(&event).expect("json to string error");
                Python::with_gil(|py| {
                    let args = PyTuple::new(py, &[event_string]);
                    handler.call1(py, args).expect("failed to call python callback");
                });
            })
            .await;
    });
}

/// Get the client from the wallet.
#[pyfunction]
pub fn get_client_from_wallet(wallet: &Wallet) -> Result<Client> {
    let client = crate::block_on(async {
        wallet
            .wallet
            .read()
            .await
            .as_ref()
            .map(|w| w.client().clone())
            .ok_or_else(|| {
                Error::from(
                    serde_json::to_string(&Response::Panic("wallet was destroyed".into()))
                        .expect("json to string error")
                        .as_str(),
                )
            })
    })?;

    Ok(Client { client })
}

/// Get the secret manager from the wallet.
#[pyfunction]
pub fn get_secret_manager_from_wallet(wallet: &Wallet) -> Result<SecretManager> {
    let secret_manager = crate::block_on(async {
        wallet
            .wallet
            .read()
            .await
            .as_ref()
            .map(|w| w.get_secret_manager().clone())
            .ok_or_else(|| {
                Error::from(
                    serde_json::to_string(&Response::Panic("wallet was destroyed".into()))
                        .expect("json to string error")
                        .as_str(),
                )
            })
    })?;

    Ok(SecretManager { secret_manager })
}
