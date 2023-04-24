// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_secret_manager_method as rust_call_secret_manager_method,
    iota_sdk::client::secret::{SecretManager as RustSecretManager, SecretManagerDto},
    Response, SecretManagerMethod,
};
use neon::prelude::*;

pub struct SecretManager {
    channel: Channel,
    secret_manager: RustSecretManager,
}

impl Finalize for SecretManager {}

impl SecretManager {
    fn new(channel: Channel, options: String) -> Arc<Self> {
        let secret_manager_dto =
            serde_json::from_str::<SecretManagerDto>(&options).expect("error initializing secret manager");
        let secret_manager =
            RustSecretManager::try_from(&secret_manager_dto).expect("error initializing secret manager");

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
                        serde_json::to_string(&Response::Error(e.into()))
                            .expect("the response is generated manually, so unwrap is safe.")
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

pub fn create_secret_manager(mut cx: FunctionContext) -> JsResult<JsBox<Arc<SecretManager>>> {
    let options = cx.argument::<JsString>(0)?;
    let options = options.value(&mut cx);
    let channel = cx.channel();

    let method_handler = SecretManager::new(channel, options);

    Ok(cx.boxed(method_handler))
}

pub fn call_secret_manager_method(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let method = cx.argument::<JsString>(0)?;
    let method = method.value(&mut cx);
    let method_handler = Arc::clone(&&cx.argument::<JsBox<Arc<SecretManager>>>(1)?);
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
