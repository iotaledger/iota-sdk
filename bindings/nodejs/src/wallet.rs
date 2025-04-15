// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_wallet_method as rust_call_wallet_method,
    iota_sdk::wallet::{
        events::types::{Event, WalletEventType},
        migration::migrate_db_chrysalis_to_stardust as rust_migrate_db_chrysalis_to_stardust,
        Wallet,
    },
    Response, Result, WalletMethod, WalletOptions,
};
use neon::prelude::*;
use tokio::sync::RwLock;

use crate::{
    client::{ClientMethodHandler, ClientMethodHandlerWrapper},
    secret_manager::SecretManagerMethodHandler,
};

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

impl Finalize for WalletMethodHandler {}

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

pub fn create_wallet(mut cx: FunctionContext) -> JsResult<JsBox<WalletMethodHandlerWrapper>> {
    let options = cx.argument::<JsString>(0)?;
    let options = options.value(&mut cx);
    let channel = cx.channel();
    let method_handler = WalletMethodHandler::new(channel, options)
        .or_else(|e| cx.throw_error(serde_json::to_string(&Response::Error(e)).expect("json to string error")))?;

    Ok(cx.boxed(WalletMethodHandlerWrapper(Arc::new(RwLock::new(Some(method_handler))))))
}

pub fn call_wallet_method(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let method = cx.argument::<JsString>(0)?;
    let method = method.value(&mut cx);
    let method_handler = Arc::clone(&cx.argument::<JsBox<WalletMethodHandlerWrapper>>(1)?.0);
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
            panic!("Wallet got destroyed")
        }
    });

    Ok(cx.undefined())
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
    let method_handler = Arc::clone(&cx.argument::<JsBox<WalletMethodHandlerWrapper>>(2)?.0);

    crate::RUNTIME.spawn(async move {
        if let Some(method_handler) = &*method_handler.read().await {
            let channel = method_handler.channel.clone();
            method_handler
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
    let method_handler = Arc::clone(&cx.argument::<JsBox<WalletMethodHandlerWrapper>>(0)?.0);
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();
    crate::RUNTIME.spawn(async move {
        *method_handler.write().await = None;
        deferred.settle_with(&channel, move |mut cx| Ok(cx.undefined()));
    });
    Ok(promise)
}

pub fn get_client(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let method_handler = Arc::clone(&cx.argument::<JsBox<WalletMethodHandlerWrapper>>(0)?.0);
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();
    crate::RUNTIME.spawn(async move {
        if let Some(method_handler) = &*method_handler.read().await {
            let client_method_handler =
                ClientMethodHandler::new_with_client(channel.clone(), method_handler.wallet.client().clone());
            deferred.settle_with(&channel, move |mut cx| {
                Ok(cx.boxed(ClientMethodHandlerWrapper(Arc::new(RwLock::new(Some(
                    client_method_handler,
                ))))))
            });
        } else {
            deferred.settle_with(&channel, move |mut cx| {
                cx.error(
                    serde_json::to_string(&Response::Panic("Wallet got destroyed".to_string()))
                        .expect("json to string error"),
                )
            });
        }
    });

    Ok(promise)
}

pub fn get_secret_manager(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let method_handler = Arc::clone(&cx.argument::<JsBox<WalletMethodHandlerWrapper>>(0)?.0);
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();
    crate::RUNTIME.spawn(async move {
        if let Some(method_handler) = &*method_handler.read().await {
            let secret_manager_method_handler = SecretManagerMethodHandler::new_with_secret_manager(
                channel.clone(),
                method_handler.wallet.get_secret_manager().clone(),
            );
            deferred.settle_with(&channel, move |mut cx| Ok(cx.boxed(secret_manager_method_handler)));
        } else {
            deferred.settle_with(&channel, move |mut cx| {
                cx.error(
                    serde_json::to_string(&Response::Panic("Wallet got destroyed".to_string()))
                        .expect("json to string error"),
                )
            });
        }
    });

    Ok(promise)
}

pub fn migrate_db_chrysalis_to_stardust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let storage_path = cx.argument::<JsString>(0)?.value(&mut cx);
    let password = cx
        .argument_opt(1)
        .map(|opt| opt.downcast_or_throw::<JsString, _>(&mut cx))
        .transpose()?
        .map(|opt| opt.value(&mut cx))
        .map(Into::into);

    let channel = cx.channel();
    let (deferred, promise) = cx.promise();
    crate::RUNTIME.spawn(async move {
        if let Err(err) = rust_migrate_db_chrysalis_to_stardust(storage_path, password, None).await {
            deferred.settle_with(&channel, move |mut cx| {
                cx.error(serde_json::to_string(&Response::Error(err.into())).expect("json to string error"))
            });
        } else {
            deferred.settle_with(&channel, move |mut cx| Ok(cx.boxed(())));
        }
    });

    Ok(promise)
}
