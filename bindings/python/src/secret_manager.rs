// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_secret_manager_method as rust_call_secret_manager_method,
    iota_sdk::client::secret::{SecretManager as RustSecretManager, SecretManagerDto},
    SecretManagerMethod,
};
use pyo3::prelude::*;
use tokio::sync::RwLock;

use crate::Error;

#[pyclass]
pub struct SecretManager {
    pub secret_manager: Arc<RwLock<RustSecretManager>>,
}

/// Create secret_manager for python-side usage.
#[pyfunction]
pub fn create_secret_manager(options: String) -> Result<SecretManager, Error> {
    let secret_manager_dto = serde_json::from_str::<SecretManagerDto>(&options)?;
    let secret_manager = RustSecretManager::try_from(secret_manager_dto)?;
    Ok(SecretManager {
        secret_manager: Arc::new(RwLock::new(secret_manager)),
    })
}

#[pyfunction]
pub fn call_secret_manager_method(secret_manager: &SecretManager, method: String) -> Result<String, Error> {
    let method = serde_json::from_str::<SecretManagerMethod>(&method)?;
    let response = crate::block_on(async {
        let secret_manager = secret_manager.secret_manager.read().await;
        rust_call_secret_manager_method(&*secret_manager, method).await
    });

    Ok(serde_json::to_string(&response)?)
}
