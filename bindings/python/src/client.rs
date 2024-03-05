// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_bindings_core::{
    call_client_method as rust_call_client_method,
    iota_sdk::client::{
        mqtt::{Error as MqttError, Topic},
        Client as RustClient, ClientBuilder,
    },
    listen_mqtt as rust_listen_mqtt, ClientMethod,
};
use pyo3::{prelude::*, types::PyTuple};

use crate::Error;

#[pyclass]
pub struct Client {
    pub client: RustClient,
}

/// Create client for python-side usage.
#[pyfunction]
pub fn create_client(options: Option<String>) -> Result<Client, Error> {
    let runtime = tokio::runtime::Runtime::new()?;
    let client = runtime.block_on(async move {
        Result::<_, Error>::Ok(match options {
            Some(options) => ClientBuilder::new().from_json(&options)?.finish().await?,
            None => ClientBuilder::new().finish().await?,
        })
    })?;

    Ok(Client { client })
}

#[pyfunction]
pub fn call_client_method(client: &Client, method: String) -> Result<String, Error> {
    let method = serde_json::from_str::<ClientMethod>(&method)?;
    let response = crate::block_on(async { rust_call_client_method(&client.client, method).await });

    Ok(serde_json::to_string(&response)?)
}

#[pyfunction]
pub fn listen_mqtt(client: &Client, topics: Vec<String>, handler: PyObject) -> Result<(), Error> {
    let topics = topics
        .iter()
        .map(Topic::new)
        .collect::<std::result::Result<Vec<Topic>, MqttError>>()?;
    crate::block_on(async {
        rust_listen_mqtt(&client.client, topics, move |event| {
            let event_string = serde_json::to_string(&event).expect("json to string error");
            Python::with_gil(|py| {
                let args = PyTuple::new(py, &[event_string]);
                handler.call1(py, args).expect("failed to call python callback");
            })
        })
        .await
    });

    Ok(())
}
