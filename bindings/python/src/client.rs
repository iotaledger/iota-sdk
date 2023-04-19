// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_bindings_core::{
    call_client_method as rust_call_client_method,
    iota_sdk::client::{Client as RustClient, ClientBuilder},
    ClientMethod,
};
use pyo3::prelude::*;

use crate::error::Result;

#[pyclass]
pub struct Client {
    pub client: RustClient,
}

/// Create client for python-side usage.
#[pyfunction]
pub fn create_client(options: Option<String>) -> Result<Client> {
    let client = match options {
        Some(options) => ClientBuilder::new().from_json(&options)?.finish()?,
        None => ClientBuilder::new().finish()?,
    };

    Ok(Client { client })
}

#[pyfunction]
pub fn call_client_method(client: &Client, method: String) -> Result<String> {
    let method = serde_json::from_str::<ClientMethod>(&method)?;
    let response = crate::block_on(async { rust_call_client_method(&client.client, method).await });

    Ok(serde_json::to_string(&response)?)
}
