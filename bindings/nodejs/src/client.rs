// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_client_method as rust_call_client_method,
    iota_sdk::client::{mqtt::Topic, Client, ClientBuilder},
    listen_mqtt as rust_listen_mqtt, ClientMethod, Response,
};
use napi::{bindgen_prelude::External, threadsafe_function::ThreadsafeFunction, Result};
use napi_derive::napi;
use tokio::sync::RwLock;

use crate::{build_js_error, destroyed_err, NodejsError};

pub type ClientMethodHandler = Arc<RwLock<Option<Client>>>;

#[napi(js_name = "createClient")]
pub async fn create_client(options: String) -> Result<External<ClientMethodHandler>> {
    let client = ClientBuilder::new()
        .from_json(&options)
        .map_err(NodejsError::new)?
        .finish()
        .await
        .map_err(NodejsError::new)?;
    Ok(External::new(Arc::new(RwLock::new(Some(client)))))
}

#[napi(js_name = "destroyClient")]
pub async fn destroy_client(client: External<ClientMethodHandler>) {
    *client.as_ref().write().await = None;
}

#[napi(js_name = "callClientMethod")]
pub async fn call_client_method(client: External<ClientMethodHandler>, method: String) -> Result<String> {
    let method = serde_json::from_str::<ClientMethod>(&method).map_err(NodejsError::new)?;

    match &*client.as_ref().read().await {
        Some(client) => {
            let response = rust_call_client_method(&client, method).await;
            match response {
                Response::Error(_) | Response::Panic(_) => Err(build_js_error(response)),
                _ => Ok(serde_json::to_string(&response).map_err(NodejsError::new)?),
            }
        }
        None => Err(destroyed_err("Client")),
    }
}

#[napi(js_name = "listenMqtt")]
pub async fn listen_mqtt(
    client: External<ClientMethodHandler>,
    topics: Vec<String>,
    callback: ThreadsafeFunction<String>,
) -> Result<()> {
    let mut validated_topics = Vec::with_capacity(topics.len());
    for topic_string in topics {
        validated_topics.push(Topic::new(topic_string).map_err(NodejsError::new)?);
    }

    match &*client.as_ref().read().await {
        Some(client) => {
            rust_listen_mqtt(client, validated_topics, move |event_data| {
                callback.call(
                    Ok(event_data),
                    napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
                );
            })
            .await;
            Ok(())
        }
        None => Err(destroyed_err("Client")),
    }
}
