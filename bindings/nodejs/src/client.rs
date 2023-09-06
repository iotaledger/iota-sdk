// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, RwLock};

use iota_sdk_bindings_core::{
    call_client_method as rust_call_client_method,
    iota_sdk::client::{mqtt::Topic, Client, ClientBuilder},
    listen_mqtt as rust_listen_mqtt, ClientMethod, Response, Result,
};
use neon::prelude::*;

type JsCallback = Root<JsFunction<JsObject>>;

pub type SharedClientMethodHandler = Arc<RwLock<Option<ClientMethodHandler>>>;

#[derive(Clone)]
pub struct ClientMethodHandler {
    channel: Channel,
    client: Client,
}

impl Finalize for ClientMethodHandler {}

impl ClientMethodHandler {
    pub fn new(channel: Channel, options: String) -> Result<Self> {
        let runtime = tokio::runtime::Runtime::new().expect("error initializing client");
        let client = runtime.block_on(ClientBuilder::new().from_json(&options)?.finish())?;

        Ok(Self { channel, client })
    }

    pub(crate) fn new_with_client(channel: Channel, client: Client) -> Self {
        Self { channel, client }
    }

    async fn call_method(&self, serialized_method: String) -> (String, bool) {
        match serde_json::from_str::<ClientMethod>(&serialized_method) {
            Ok(method) => {
                let res = rust_call_client_method(&self.client, method).await;
                let mut is_err = matches!(res, Response::Error(_) | Response::Panic(_));

                let msg = match serde_json::to_string(&res) {
                    Ok(msg) => msg,
                    Err(e) => {
                        is_err = true;
                        serde_json::to_string(&Response::Error(e.into())).expect("json to string error")
                    }
                };

                (msg, is_err)
            }
            Err(e) => (
                serde_json::to_string(&Response::Error(e.into())).expect("json to string error"),
                true,
            ),
        }
    }
}

pub fn create_client(mut cx: FunctionContext) -> JsResult<JsBox<SharedClientMethodHandler>> {
    let options = cx.argument::<JsString>(0)?;
    let options = options.value(&mut cx);
    let channel = cx.channel();
    let method_handler = ClientMethodHandler::new(channel, options)
        .or_else(|e| cx.throw_error(serde_json::to_string(&Response::Error(e)).expect("json to string error")))?;
    Ok(cx.boxed(Arc::new(RwLock::new(Some(method_handler)))))
}

pub fn destroy_client(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    match cx.argument::<JsBox<SharedClientMethodHandler>>(0)?.write() {
        Ok(mut lock) => *lock = None,
        Err(e) => {
            return cx
                .throw_error(serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error"));
        }
    }
    Ok(cx.undefined())
}

pub fn call_client_method(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    match cx.argument::<JsBox<SharedClientMethodHandler>>(1)?.read() {
        Ok(lock) => {
            let method_handler = lock.clone();
            let method = cx.argument::<JsString>(0)?;
            let method = method.value(&mut cx);
            let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);
            if let Some(method_handler) = method_handler {
                crate::RUNTIME.spawn(async move {
                    let (response, is_error) = method_handler.call_method(method).await;
                    method_handler.channel.send(move |mut cx| {
                        let cb = callback.into_inner(&mut cx);
                        let this = cx.undefined();

                        let args = [
                            if is_error {
                                cx.string(response.clone()).upcast::<JsValue>()
                            } else {
                                cx.undefined().upcast::<JsValue>()
                            },
                            cx.string(response).upcast::<JsValue>(),
                        ];

                        cb.call(&mut cx, this, args)?;
                        Ok(())
                    });
                });

                Ok(cx.undefined())
            } else {
                // Notify that the client was destroyed
                cx.throw_error(
                    serde_json::to_string(&Response::Panic("Client was destroyed".to_string()))
                        .expect("json to string error"),
                )
            }
        }
        Err(e) => cx.throw_error(serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error")),
    }
}

// MQTT
pub fn listen_mqtt(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
    let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
    let mut topics = Vec::with_capacity(vec.len());
    for topic_string in vec {
        let topic = topic_string.downcast::<JsString, FunctionContext>(&mut cx).unwrap();
        topics.push(Topic::new(topic.value(&mut cx).as_str()).expect("invalid MQTT topic"));
    }

    let callback = Arc::new(cx.argument::<JsFunction>(1)?.root(&mut cx));

    match cx.argument::<JsBox<SharedClientMethodHandler>>(2)?.read() {
        Ok(lock) => {
            let method_handler = lock.clone();
            if let Some(method_handler) = method_handler {
                crate::RUNTIME.spawn(async move {
                    rust_listen_mqtt(&method_handler.client, topics, move |event_data| {
                        call_event_callback(&method_handler.channel, event_data, callback.clone())
                    })
                    .await;
                });
                Ok(cx.undefined())
            } else {
                // Notify that the client was destroyed
                cx.throw_error(
                    serde_json::to_string(&Response::Panic("Client was destroyed".to_string()))
                        .expect("json to string error"),
                )
            }
        }
        Err(e) => cx.throw_error(serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error")),
    }
}

fn call_event_callback(channel: &neon::event::Channel, event_data: String, callback: Arc<JsCallback>) {
    channel.send(move |mut cx| {
        let cb = (*callback).to_inner(&mut cx);
        let this = cx.undefined();
        let args = [
            cx.undefined().upcast::<JsValue>(),
            cx.string(event_data).upcast::<JsValue>(),
        ];

        cb.call(&mut cx, this, args)?;

        Ok(())
    });
}
