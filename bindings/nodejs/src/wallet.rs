// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_wallet_method as rust_call_wallet_method,
    iota_sdk::wallet::{
        events::types::{Event, WalletEventType},
        Wallet,
    },
    Response, Result, WalletMethod, WalletOptions,
};
use neon::prelude::*;
use tokio::sync::RwLock;

use crate::client::ClientMethodHandler;

// Wrapper so we can destroy the WalletMethodHandler
pub type WalletMethodHandlerWrapperInner = Arc<RwLock<Option<WalletMethodHandler>>>;
// Wrapper because we can't impl Finalize on WalletMethodHandlerWrapperInner
pub struct WalletMethodHandlerWrapper(pub WalletMethodHandlerWrapperInner);
impl Finalize for WalletMethodHandlerWrapper {}

pub struct WalletMethodHandler {
    channel: Channel,
    wallet: Wallet,
}

type JsCallback = Root<JsFunction<JsObject>>;

impl WalletMethodHandler {
    fn new(channel: Channel, options: String) -> Result<Self> {
        let wallet_options = serde_json::from_str::<WalletOptions>(&options)?;

        let wallet = crate::RUNTIME
            .block_on(async move { wallet_options.build_manager().await })
            .expect("error initializing wallet");

        Ok(Self { channel, wallet })
    }

    async fn call_method(&self, method: String) -> (String, bool) {
        match serde_json::from_str::<WalletMethod>(&method) {
            Ok(method) => {
                let res = rust_call_wallet_method(&self.wallet, method).await;
                let mut is_err = matches!(res, Response::Error(_) | Response::Panic(_));

                let msg = match serde_json::to_string(&res) {
                    Ok(msg) => msg,
                    Err(e) => {
                        is_err = true;
                        serde_json::to_string(&Response::Error(e.into()))
                            .expect("the response is generated manually, so unwrap is safe.")
                    }
                };

                (msg, is_err)
            }
            Err(e) => {
                log::debug!("{:?}", e);
                (
                    serde_json::to_string(&Response::Error(e.into()))
                        .expect("the response is generated manually, so unwrap is safe."),
                    true,
                )
            }
        }
    }
}

impl Finalize for WalletMethodHandler {}

fn call_event_callback(channel: &neon::event::Channel, event_data: Event, callback: Arc<JsCallback>) {
    channel.send(move |mut cx| {
        let cb = (*callback).to_inner(&mut cx);
        let this = cx.undefined();
        let args = vec![
            cx.undefined().upcast::<JsValue>(),
            cx.string(serde_json::to_string(&event_data).unwrap())
                .upcast::<JsValue>(),
        ];

        cb.call(&mut cx, this, args)?;

        Ok(())
    });
}

pub fn create_wallet(mut cx: FunctionContext) -> JsResult<JsBox<WalletMethodHandlerWrapper>> {
    let options = cx.argument::<JsString>(0)?;
    let options = options.value(&mut cx);
    let channel = cx.channel();
    let message_handler = WalletMethodHandler::new(channel, options).or_else(|e| {
        cx.throw_error(
            serde_json::to_string(&Response::Error(e)).expect("the response is generated manually, so unwrap is safe."),
        )
    })?;

    Ok(cx.boxed(WalletMethodHandlerWrapper(Arc::new(RwLock::new(Some(message_handler))))))
}

pub fn call_wallet_method(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let message = cx.argument::<JsString>(0)?;
    let message = message.value(&mut cx);
    let message_handler = Arc::clone(&&cx.argument::<JsBox<WalletMethodHandlerWrapper>>(1)?.0);
    let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);

    crate::RUNTIME.spawn(async move {
        if let Some(message_handler) = &*message_handler.read().await {
            let (response, is_error) = message_handler.call_method(message).await;
            message_handler.channel.send(move |mut cx| {
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
        } else {
            panic!("Wallet got destroyed")
        }
    });

    Ok(cx.undefined())
}

pub fn listen_wallet(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
    let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
    let mut event_types = vec![];
    for event_string in vec {
        let event_type = event_string.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
        let wallet_event_type =
            WalletEventType::try_from(event_type.value(&mut cx).as_str()).or_else(|e| cx.throw_error(e))?;
        event_types.push(wallet_event_type);
    }

    let callback = Arc::new(cx.argument::<JsFunction>(1)?.root(&mut cx));
    let message_handler = Arc::clone(&&cx.argument::<JsBox<WalletMethodHandlerWrapper>>(2)?.0);

    crate::RUNTIME.spawn(async move {
        if let Some(message_handler) = &*message_handler.read().await {
            let channel = message_handler.channel.clone();
            message_handler
                .wallet
                .listen(event_types, move |event_data| {
                    call_event_callback(&channel, event_data.clone(), callback.clone())
                })
                .await;
        } else {
            panic!("Wallet got destroyed")
        }
    });

    Ok(cx.undefined())
}

pub fn destroy_wallet(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let message_handler = Arc::clone(&&cx.argument::<JsBox<WalletMethodHandlerWrapper>>(0)?.0);
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();
    crate::RUNTIME.spawn(async move {
        *message_handler.write().await = None;
        deferred.settle_with(&channel, move |mut cx| Ok(cx.undefined()));
    });
    Ok(promise)
}

pub fn get_client(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let message_handler = Arc::clone(&&cx.argument::<JsBox<WalletMethodHandlerWrapper>>(0)?.0);
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();
    let (sender, receiver) = std::sync::mpsc::channel();
    crate::RUNTIME.spawn(async move {
        if let Some(message_handler) = &*message_handler.read().await {
            let client = match message_handler.wallet.get_client().await {
                Ok(client) => client,
                Err(err) => return sender.send(Some(err)),
            };

            let client_message_handler = ClientMethodHandler::new_with_client(channel.clone(), client);
            deferred.settle_with(&channel, move |mut cx| Ok(cx.boxed(client_message_handler)));
            sender.send(None)
        } else {
            panic!("Wallet got destroyed")
        }
    });

    if let Some(err) = receiver.recv().expect("channel hung up") {
        cx.throw_error(err.to_string())?;
    }

    Ok(promise)
}
