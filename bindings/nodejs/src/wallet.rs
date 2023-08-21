// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, RwLock};

use iota_sdk_bindings_core::{
    call_wallet_method as rust_call_wallet_method,
    iota_sdk::wallet::{
        events::types::{Event, WalletEventType},
        Wallet,
    },
    Response, Result, WalletMethod, WalletOptions,
};
use neon::prelude::*;

use crate::{
    client::{ClientMethodHandler, SharedClientMethodHandler},
    secret_manager::SecretManagerMethodHandler,
};

pub type SharedWalletMethodHandler = Arc<RwLock<Option<WalletMethodHandler>>>;

#[derive(Clone)]
pub struct WalletMethodHandler {
    channel: Channel,
    wallet: Wallet,
}

impl Finalize for WalletMethodHandler {}

type JsCallback = Root<JsFunction<JsObject>>;

impl WalletMethodHandler {
    fn new(channel: Channel, options: String) -> Result<Self> {
        let wallet_options = serde_json::from_str::<WalletOptions>(&options)?;

        let wallet = crate::RUNTIME.block_on(async move { wallet_options.build().await })?;

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
                        serde_json::to_string(&Response::Error(e.into())).expect("json to string error")
                    }
                };

                (msg, is_err)
            }
            Err(e) => {
                log::error!("{:?}", e);
                (
                    serde_json::to_string(&Response::Error(e.into())).expect("json to string error"),
                    true,
                )
            }
        }
    }
}

fn call_event_callback(channel: &neon::event::Channel, event_data: Event, callback: Arc<JsCallback>) {
    channel.send(move |mut cx| {
        let cb = (*callback).to_inner(&mut cx);
        let this = cx.undefined();
        let args = [
            cx.undefined().upcast::<JsValue>(),
            cx.string(serde_json::to_string(&event_data).unwrap())
                .upcast::<JsValue>(),
        ];

        cb.call(&mut cx, this, args)?;

        Ok(())
    });
}

pub fn create_wallet(mut cx: FunctionContext) -> JsResult<JsBox<SharedWalletMethodHandler>> {
    let options = cx.argument::<JsString>(0)?;
    let options = options.value(&mut cx);
    let channel = cx.channel();
    let method_handler = WalletMethodHandler::new(channel, options)
        .or_else(|e| cx.throw_error(serde_json::to_string(&Response::Error(e)).expect("json to string error")))?;

    Ok(cx.boxed(Arc::new(RwLock::new(Some(method_handler)))))
}

pub fn call_wallet_method(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    match cx.argument::<JsBox<SharedWalletMethodHandler>>(1)?.read() {
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
                // Notify that the wallet was destroyed
                cx.throw_error(
                    serde_json::to_string(&Response::Panic("Wallet was destroyed".to_string()))
                        .expect("json to string error"),
                )
            }
        }
        Err(e) => cx.throw_error(serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error")),
    }
}

pub fn listen_wallet(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
    let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
    let mut event_types = Vec::with_capacity(vec.len());
    for event_string in vec {
        let event_type = event_string.downcast_or_throw::<JsNumber, FunctionContext>(&mut cx)?;
        let wallet_event_type =
            WalletEventType::try_from(event_type.value(&mut cx) as u8).or_else(|e| cx.throw_error(e))?;
        event_types.push(wallet_event_type);
    }
    let callback = Arc::new(cx.argument::<JsFunction>(1)?.root(&mut cx));

    match cx.argument::<JsBox<SharedWalletMethodHandler>>(2)?.read() {
        Ok(lock) => {
            let method_handler = lock.clone();
            if let Some(method_handler) = method_handler {
                crate::RUNTIME.spawn(async move {
                    method_handler
                        .wallet
                        .listen(event_types, move |event_data| {
                            call_event_callback(&method_handler.channel, event_data.clone(), callback.clone())
                        })
                        .await;
                });

                Ok(cx.undefined())
            } else {
                // Notify that the wallet was destroyed
                cx.throw_error(
                    serde_json::to_string(&Response::Panic("Wallet was destroyed".to_string()))
                        .expect("json to string error"),
                )
            }
        }
        Err(e) => cx.throw_error(serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error")),
    }
}

pub fn destroy_wallet(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    match cx.argument::<JsBox<SharedWalletMethodHandler>>(0)?.write() {
        Ok(mut lock) => *lock = None,
        Err(e) => {
            return cx
                .throw_error(serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error"));
        }
    }
    Ok(cx.undefined())
}

pub fn get_client(mut cx: FunctionContext) -> JsResult<JsBox<SharedClientMethodHandler>> {
    match cx.argument::<JsBox<SharedWalletMethodHandler>>(0)?.read() {
        Ok(lock) => {
            if let Some(method_handler) = &*lock {
                let client_method_handler =
                    ClientMethodHandler::new_with_client(cx.channel(), method_handler.wallet.client().clone());

                Ok(cx.boxed(Arc::new(RwLock::new(Some(client_method_handler)))))
            } else {
                // Notify that the wallet was destroyed
                cx.throw_error(
                    serde_json::to_string(&Response::Panic("Wallet was destroyed".to_string()))
                        .expect("json to string error"),
                )
            }
        }
        Err(e) => {
            return cx
                .throw_error(serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error"));
        }
    }
}

pub fn get_secret_manager(mut cx: FunctionContext) -> JsResult<JsBox<Arc<SecretManagerMethodHandler>>> {
    match cx.argument::<JsBox<SharedWalletMethodHandler>>(0)?.read() {
        Ok(lock) => {
            if let Some(method_handler) = &*lock {
                let secret_manager_method_handler = SecretManagerMethodHandler::new_with_secret_manager(
                    cx.channel(),
                    method_handler.wallet.get_secret_manager().clone(),
                );

                Ok(cx.boxed(secret_manager_method_handler))
            } else {
                // Notify that the wallet was destroyed
                cx.throw_error(
                    serde_json::to_string(&Response::Panic("Wallet was destroyed".to_string()))
                        .expect("json to string error"),
                )
            }
        }
        Err(e) => {
            return cx
                .throw_error(serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error"));
        }
    }
}
