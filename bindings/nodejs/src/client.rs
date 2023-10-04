// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_client_method as rust_call_client_method,
    iota_sdk::client::{mqtt::Topic, Client, ClientBuilder},
    listen_mqtt as rust_listen_mqtt, ClientMethod, Response, Result,
};
use neon::prelude::*;
use tokio::sync::RwLock;

type JsCallback = Root<JsFunction<JsObject>>;

// Wrapper so we can destroy the ClientMethodHandler
pub type ClientMethodHandlerWrapperInner = Arc<RwLock<Option<ClientMethodHandler>>>;
// Wrapper because we can't impl Finalize on ClientMethodHandlerWrapperInner
pub struct ClientMethodHandlerWrapper(pub ClientMethodHandlerWrapperInner);
pub struct ClientMethodHandler {
    channel: Channel,
    client: Client,
}

impl Finalize for ClientMethodHandlerWrapper {}

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
            Err(e) => {
                log::error!("{:?}", e);
                (format!("Couldn't parse to method with error - {e:?}"), true)
            }
        }
    }
}

pub fn create_client(mut cx: FunctionContext) -> JsResult<JsBox<ClientMethodHandlerWrapper>> {
    let options = cx.argument::<JsString>(0)?;
    let options = options.value(&mut cx);
    let channel = cx.channel();
    let method_handler = ClientMethodHandler::new(channel, options)
        .or_else(|e| cx.throw_error(serde_json::to_string(&Response::Error(e)).expect("json to string error")))?;
    Ok(cx.boxed(ClientMethodHandlerWrapper(Arc::new(RwLock::new(Some(method_handler))))))
}

pub fn destroy_client(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let method_handler = Arc::clone(&cx.argument::<JsBox<ClientMethodHandlerWrapper>>(0)?.0);
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();
    crate::RUNTIME.spawn(async move {
        *method_handler.write().await = None;
        deferred.settle_with(&channel, move |mut cx| Ok(cx.undefined()));
    });
    Ok(promise)
}

pub fn call_client_method(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let method = cx.argument::<JsString>(0)?;
    let method = method.value(&mut cx);
    let method_handler = Arc::clone(&cx.argument::<JsBox<ClientMethodHandlerWrapper>>(1)?.0);
    let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);

    crate::RUNTIME.spawn(async move {
        if let Some(method_handler) = &*method_handler.read().await {
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
        } else {
            panic!("Client got destroyed")
        }
    });

    Ok(cx.undefined())
}

// MQTT
pub fn listen_mqtt(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
    let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
    let mut topics = Vec::with_capacity(vec.len());
    for topic_string in vec {
        let topic = topic_string.downcast::<JsString, FunctionContext>(&mut cx).unwrap();
        topics.push(Topic::new(topic.value(&mut cx).as_str()).expect("invalid MQTT topic"));
    }

    let callback = Arc::new(cx.argument::<JsFunction>(1)?.root(&mut cx));
    let method_handler = Arc::clone(&cx.argument::<JsBox<ClientMethodHandlerWrapper>>(2)?.0);
    let (deferred, promise) = cx.promise();

    crate::RUNTIME.spawn(async move {
        if let Some(method_handler) = &*method_handler.read().await {
            let channel0 = method_handler.channel.clone();
            let channel1 = method_handler.channel.clone();
            rust_listen_mqtt(&method_handler.client, topics, move |event_data| {
                call_event_callback(&channel0, event_data, callback.clone())
            })
            .await;

            deferred.settle_with(&channel1, move |mut cx| Ok(cx.undefined()));
        } else {
            panic!("Client got destroyed")
        }
    });

    Ok(promise)
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
