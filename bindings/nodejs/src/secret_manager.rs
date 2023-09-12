// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{ops::Deref, sync::Arc};

use iota_sdk_bindings_core::{
    call_secret_manager_method as rust_call_secret_manager_method,
    iota_sdk::client::{
        secret::{SecretManager, SecretManagerDto},
        stronghold::StrongholdAdapter,
    },
    Response, Result, SecretManagerMethod,
};
use neon::prelude::*;
use tokio::sync::RwLock;

pub struct SecretManagerMethodHandler {
    channel: Channel,
    secret_manager: Arc<RwLock<SecretManager>>,
}

impl Finalize for SecretManagerMethodHandler {}

impl SecretManagerMethodHandler {
    fn new(channel: Channel, options: String) -> Result<Arc<Self>> {
        let secret_manager_dto = serde_json::from_str::<SecretManagerDto>(&options)?;
        let secret_manager = SecretManager::try_from(secret_manager_dto)?;

        Ok(Arc::new(Self {
            channel,
            secret_manager: Arc::new(RwLock::new(secret_manager)),
        }))
    }

    pub fn new_with_secret_manager(channel: Channel, secret_manager: Arc<RwLock<SecretManager>>) -> Arc<Self> {
        Arc::new(Self {
            channel,
            secret_manager,
        })
    }

    async fn call_method(&self, method: String) -> (String, bool) {
        match serde_json::from_str::<SecretManagerMethod>(&method) {
            Ok(method) => {
                let res = rust_call_secret_manager_method(&self.secret_manager, method).await;
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

pub fn create_secret_manager(mut cx: FunctionContext) -> JsResult<JsBox<Arc<SecretManagerMethodHandler>>> {
    let options = cx.argument::<JsString>(0)?;
    let options = options.value(&mut cx);
    let channel = cx.channel();

    let method_handler = SecretManagerMethodHandler::new(channel, options)
        .or_else(|e| cx.throw_error(serde_json::to_string(&Response::Error(e)).expect("json to string error")))?;

    Ok(cx.boxed(method_handler))
}

pub fn call_secret_manager_method(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let method = cx.argument::<JsString>(0)?;
    let method = method.value(&mut cx);
    let method_handler = Arc::clone(cx.argument::<JsBox<Arc<SecretManagerMethodHandler>>>(1)?.deref());
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

pub fn migrate_stronghold_snapshot_v2_to_v3(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let current_path = cx.argument::<JsString>(0)?.value(&mut cx);
    let current_password = cx.argument::<JsString>(1)?.value(&mut cx).into();
    let salt = cx.argument::<JsString>(2)?.value(&mut cx);
    let rounds = cx.argument::<JsNumber>(3)?.value(&mut cx);
    let new_path = cx
        .argument_opt(4)
        .map(|opt| opt.downcast_or_throw::<JsString, _>(&mut cx))
        .transpose()?
        .map(|opt| opt.value(&mut cx));
    let new_password = cx
        .argument_opt(5)
        .map(|opt| opt.downcast_or_throw::<JsString, _>(&mut cx))
        .transpose()?
        .map(|opt| opt.value(&mut cx))
        .map(Into::into);

    StrongholdAdapter::migrate_snapshot_v2_to_v3(
        &current_path,
        current_password,
        salt,
        rounds as u32,
        new_path.as_ref(),
        new_password,
    )
    .or_else(|e| {
        cx.throw_error(
            serde_json::to_string(&Response::Error(e.into()))
                .expect("the response is generated manually, so unwrap is safe."),
        )
    })?;

    Ok(cx.undefined())
}
