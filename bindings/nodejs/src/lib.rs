// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
mod client;
mod secret_manager;
mod wallet;

use iota_sdk_bindings_core::{
    call_utils_method as rust_call_utils_method, init_logger as rust_init_logger, Response, UtilsMethod,
};
use neon::prelude::*;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

pub static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

pub fn init_logger(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let config = cx.argument::<JsString>(0)?.value(&mut cx);
    match rust_init_logger(config) {
        Ok(_) => Ok(cx.undefined()),
        Err(err) => {
            cx.throw_error(serde_json::to_string(&Response::Panic(err.to_string())).expect("json to string error"))
        }
    }
}

pub fn call_utils_method(mut cx: FunctionContext) -> JsResult<JsString> {
    let method = cx.argument::<JsString>(0)?.value(&mut cx);
    let method = match serde_json::from_str::<UtilsMethod>(&method) {
        Ok(method) => method,
        Err(err) => {
            return Ok(cx.string(serde_json::to_string(&Response::Error(err.into())).expect("json to string error")));
        }
    };
    let response = rust_call_utils_method(method);

    Ok(cx.string(serde_json::to_string(&response).unwrap()))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("initLogger", init_logger)?;

    cx.export_function("callUtilsMethodRust", call_utils_method)?;

    // Client
    cx.export_function("callClientMethod", client::call_client_method)?;
    cx.export_function("createClient", client::create_client)?;
    cx.export_function("destroyClient", client::destroy_client)?;

    // MQTT
    cx.export_function("listenMqtt", client::listen_mqtt)?;

    cx.export_function("callSecretManagerMethod", secret_manager::call_secret_manager_method)?;
    cx.export_function("createSecretManager", secret_manager::create_secret_manager)?;
    cx.export_function(
        "migrateStrongholdSnapshotV2ToV3",
        secret_manager::migrate_stronghold_snapshot_v2_to_v3,
    )?;

    // Wallet
    cx.export_function("callWalletMethod", wallet::call_wallet_method)?;
    cx.export_function("createWallet", wallet::create_wallet)?;
    cx.export_function("destroyWallet", wallet::destroy_wallet)?;
    cx.export_function("getClientFromWallet", wallet::get_client)?;
    cx.export_function("getSecretManagerFromWallet", wallet::get_secret_manager)?;
    cx.export_function("listenWallet", wallet::listen_wallet)?;

    Ok(())
}

#[macro_export]
macro_rules! binding_glue {
    ($cx:ident, $method:ident, $method_handler:ident, $callback:ident, $name:expr) => {
        match $method_handler?.read() {
            Ok(handler) => {
                if let Some(inner) = handler.clone() {
                    crate::RUNTIME.spawn(async move {
                        let (response, is_error) = inner.call_method($method).await;
                        inner.channel.send(move |mut cx| {
                            let cb = $callback.into_inner(&mut cx);
                            let this = cx.undefined();
                    
                            let args = [
                                if is_error {
                                    cx.error(response.clone())?.upcast::<JsValue>()
                                } else {
                                    cx.undefined().upcast::<JsValue>()
                                },
                                cx.string(response).upcast::<JsValue>(),
                            ];
                    
                            cb.call(&mut cx, this, args)?;
                            Ok(())
                        });
                    }); 
                    Ok($cx.undefined())
                } else {
                    $cx.throw_error(
                        serde_json::to_string(&Response::Panic(format!("{} was destroyed", $name)))
                            .expect("json to string error"),
                    )
                }
            },
            Err(e) => $cx.throw_error(serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error")),
        }
    };
}



#[macro_export]
macro_rules! binding_glue1 {
    ($cx:ident, $method:ident, $method_handler:ident, $callback:ident, $name:expr, $code:expr) => {
        match $method_handler?.read() {
            Ok(handler) => {
                if let Some(inner) = handler.clone() {
                    crate::RUNTIME.spawn(async move {
                        $code(inner).await;
                    }); 
                    Ok($cx.undefined())
                } else {
                    $cx.throw_error(
                        serde_json::to_string(&Response::Panic(format!("{} was destroyed", $name)))
                            .expect("json to string error"),
                    )
                }
            },
            Err(e) => $cx.throw_error(serde_json::to_string(&Response::Panic(e.to_string())).expect("json to string error")),
        }
    };
}
