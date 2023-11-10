// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, RwLock};

use iota_sdk_bindings_core::{
    call_client_method as rust_call_client_method,
    iota_sdk::client::{mqtt::Topic, Client, ClientBuilder},
    listen_mqtt as rust_listen_mqtt, ClientMethod, Response,
};
use napi::{bindgen_prelude::External, threadsafe_function::ThreadsafeFunction, Error, Result, Status};
use napi_derive::napi;
use tokio::sync::RwLock;

use crate::NodejsError;

pub type ClientMethodHandler = Arc<RwLock<Option<Client>>>;

#[napi(js_name = "createClient")]
pub fn create_client(options: String) -> Result<External<ClientMethodHandler>> {
    let runtime = tokio::runtime::Runtime::new().map_err(NodejsError::from)?;
    let client = runtime
        .block_on(
            ClientBuilder::new()
                .from_json(&options)
                .map_err(NodejsError::from)?
                .finish(),
        )
        .map_err(NodejsError::from)?;
    Ok(External::new(Arc::new(RwLock::new(Some(client)))))
}

#[napi(js_name = "destroyClient")]
pub async fn destroy_client(client: External<ClientMethodHandler>) {
    *client.as_ref().write().await = None;
}

#[napi(js_name = "callClientMethod")]
pub async fn call_client_method(client: External<ClientMethodHandler>, method: String) -> Result<String> {
    let client_method = serde_json::from_str::<ClientMethod>(&method).map_err(NodejsError::from)?;

    if let Some(client) = &*client.as_ref().read().await {
        let res = rust_call_client_method(client, client_method).await;
        if matches!(res, Response::Error(_) | Response::Panic(_)) {
            return Err(Error::new(
                Status::GenericFailure,
                serde_json::to_string(&res).map_err(NodejsError::from)?,
            ));
        }

        Ok(serde_json::to_string(&res).map_err(NodejsError::from)?)
    } else {
        Err(Error::new(
            Status::GenericFailure,
            serde_json::to_string(&Response::Panic("Client got destroyed".to_string())).map_err(NodejsError::from)?,
        ))
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
        validated_topics.push(Topic::new(topic_string).map_err(NodejsError::from)?);
    }

    if let Some(client) = &*client.as_ref().read().await {
        rust_listen_mqtt(client, validated_topics, move |event_data| {
            callback.call(
                Ok(event_data),
                napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
            );
        })
        .await;
        Ok(())
    } else {
        Err(Error::new(
            Status::GenericFailure,
            serde_json::to_string(&Response::Panic("Client got destroyed".to_string())).map_err(NodejsError::from)?,
        ))
    }
}
