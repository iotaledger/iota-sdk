// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_client_method as rust_call_client_method,
    iota_sdk::client::{mqtt::Topic, Client, ClientBuilder},
    listen_mqtt as rust_listen_mqtt, ClientMethod, Response,
};
use neon::prelude::*;

type JsCallback = Root<JsFunction<JsObject>>;

pub struct ClientMethodHandler {
    channel: Channel,
    client: Client,
}

impl Finalize for ClientMethodHandler {}

impl ClientMethodHandler {
    pub fn new(channel: Channel, options: String) -> Arc<Self> {
        let runtime = tokio::runtime::Runtime::new().expect("error initializing client");
        let client = runtime
            .block_on(
                ClientBuilder::new()
                    .from_json(&options)
                    .expect("error initializing client")
                    .finish(),
            )
            .expect("error initializing client");

        Arc::new(Self { channel, client })
    }

    pub(crate) fn new_with_client(channel: Channel, client: Client) -> Arc<Self> {
        Arc::new(Self { channel, client })
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
                log::debug!("{:?}", e);
                (format!("Couldn't parse to method with error - {e:?}"), true)
            }
        }
    }
}

pub fn create_client(mut cx: FunctionContext) -> JsResult<JsBox<Arc<ClientMethodHandler>>> {
    let options = cx.argument::<JsString>(0)?;
    let options = options.value(&mut cx);
    let channel = cx.channel();
    let method_handler = ClientMethodHandler::new(channel, options);

    Ok(cx.boxed(method_handler))
}

pub fn call_client_method(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let method = cx.argument::<JsString>(0)?;
    let method = method.value(&mut cx);
    let method_handler = Arc::clone(&&cx.argument::<JsBox<Arc<ClientMethodHandler>>>(1)?);
    let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);

    crate::RUNTIME.spawn(async move {
        let (response, is_error) = method_handler.call_method(method).await;
        method_handler.channel.send(move |mut cx| {
            let cb = callback.into_inner(&mut cx);
            let this = cx.undefined();

            let args = vec![
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
}

// MQTT
pub fn listen_mqtt(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
    let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
    let mut topics = vec![];
    for topic_string in vec {
        let topic = topic_string.downcast::<JsString, FunctionContext>(&mut cx).unwrap();
        topics.push(Topic::try_from(topic.value(&mut cx).as_str().to_string()).expect("invalid MQTT topic"));
    }

    let callback = Arc::new(cx.argument::<JsFunction>(1)?.root(&mut cx));
    let method_handler = Arc::clone(&&cx.argument::<JsBox<Arc<ClientMethodHandler>>>(2)?);
    let (deferred, promise) = cx.promise();

    crate::RUNTIME.spawn(async move {
        let channel0 = method_handler.channel.clone();
        let channel1 = method_handler.channel.clone();
        rust_listen_mqtt(&method_handler.client, topics, move |event_data| {
            call_event_callback(&channel0, event_data, callback.clone())
        })
        .await;

        deferred.settle_with(&channel1, move |mut cx| Ok(cx.undefined()));
    });

    Ok(promise)
}

fn call_event_callback(channel: &neon::event::Channel, event_data: String, callback: Arc<JsCallback>) {
    channel.send(move |mut cx| {
        let cb = (*callback).to_inner(&mut cx);
        let this = cx.undefined();
        let args = vec![
            cx.undefined().upcast::<JsValue>(),
            cx.string(event_data).upcast::<JsValue>(),
        ];

        cb.call(&mut cx, this, args)?;

        Ok(())
    });
}
