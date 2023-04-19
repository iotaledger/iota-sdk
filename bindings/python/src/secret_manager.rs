// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_bindings_core::{
    call_secret_manager_method as rust_call_secret_manager_method,
    iota_sdk::client::secret::{SecretManager as RustSecretManager, SecretManagerDto},
    SecretManagerMethod,
};
use pyo3::prelude::*;

use crate::error::Result;

#[pyclass]
pub struct SecretManager {
    pub secret_manager: RustSecretManager,
}

#[pyfunction]
/// Create secret_manager for python-side usage.
pub fn create_secret_manager(options: String) -> Result<SecretManager> {
    let secret_manager_dto = serde_json::from_str::<SecretManagerDto>(&options)?;
    let secret_manager = RustSecretManager::try_from(&secret_manager_dto)?;
    Ok(SecretManager { secret_manager })
}

#[pyfunction]
pub fn call_secret_manager_method(secret_manager: &mut SecretManager, method: String) -> Result<String> {
    let method = serde_json::from_str::<SecretManagerMethod>(&method)?;
    let response =
        crate::block_on(async { rust_call_secret_manager_method(&mut secret_manager.secret_manager, method).await });

    Ok(serde_json::to_string(&response)?)
}
