// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_secret_manager_method,
    iota_sdk::client::secret::{SecretManager, SecretManagerDto},
    Response, SecretManagerMethod,
};
use tokio::sync::RwLock;
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

/// The SecretManager method handler.
#[wasm_bindgen(js_name = SecretManagerMethodHandler)]
pub struct SecretManagerMethodHandler {
    pub(crate) secret_manager: Arc<RwLock<SecretManager>>,
}

impl SecretManagerMethodHandler {
    pub(crate) fn new(secret_manager: Arc<RwLock<SecretManager>>) -> Self {
        Self { secret_manager }
    }
}

/// Creates a method handler with the given secret_manager options.
#[wasm_bindgen(js_name = createSecretManager)]
#[allow(non_snake_case)]
pub fn create_secret_manager(options: String) -> Result<SecretManagerMethodHandler, JsValue> {
    let secret_manager_dto = serde_json::from_str::<SecretManagerDto>(&options).map_err(|err| {
        JsError::new(&serde_json::to_string(&Response::Error(err.into())).expect("json to string error"))
    })?;
    let secret_manager = SecretManager::try_from(secret_manager_dto).map_err(|err| {
        JsError::new(&serde_json::to_string(&Response::Error(err.into())).expect("json to string error"))
    })?;

    Ok(SecretManagerMethodHandler {
        secret_manager: Arc::new(RwLock::new(secret_manager)),
    })
}

/// Handles a method, returns the response as a JSON-encoded string.
///
/// Returns an error if the response itself is an error or panic.
#[wasm_bindgen(js_name = callSecretManagerMethodAsync)]
#[allow(non_snake_case)]
pub async fn call_secret_manager_method_async(
    method: String,
    methodHandler: &SecretManagerMethodHandler,
) -> Result<String, JsError> {
    let secret_manager = &methodHandler.secret_manager;
    let method: SecretManagerMethod = serde_json::from_str(&method).map_err(|err| {
        JsError::new(&serde_json::to_string(&Response::Error(err.into())).expect("json to string error"))
    })?;

    let response = call_secret_manager_method(secret_manager, method).await;
    let ser = serde_json::to_string(&response).expect("json to string error");
    match response {
        Response::Error(_) | Response::Panic(_) => Err(JsError::new(&ser)),
        _ => Ok(ser),
    }
}

/// Stronghold snapshot migration is not supported for WebAssembly bindings.
///
/// Throws an error if called, only included for compatibility
/// with the Node.js bindings TypeScript definitions.
#[wasm_bindgen(js_name = migrateStrongholdSnapshotV2ToV3)]
pub fn migrate_stronghold_snapshot_v2_to_v3(
    _current_path: String,
    _current_password: String,
    _salt: &str,
    _rounds: u32,
    _new_path: Option<String>,
    _new_password: Option<String>,
) -> Result<(), JsError> {
    Err(JsError::new(
        &serde_json::to_string(&Response::Panic(
            "Stronghold snapshot migration is not supported for WebAssembly".to_string(),
        ))
        .expect("json to string error"),
    ))
}
