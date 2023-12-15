// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_secret_manager_method as rust_call_secret_manager_method,
    iota_sdk::client::{
        secret::{SecretManager, SecretManagerDto},
        stronghold::StrongholdAdapter,
    },
    Response, SecretManagerMethod,
};
use napi::{bindgen_prelude::External, Error, Result, Status};
use napi_derive::napi;
use tokio::sync::RwLock;

use crate::NodejsError;

pub type SecretManagerMethodHandler = Arc<RwLock<SecretManager>>;

#[napi(js_name = "createSecretManager")]
pub fn create_secret_manager(options: String) -> Result<External<SecretManagerMethodHandler>> {
    let secret_manager_dto = serde_json::from_str::<SecretManagerDto>(&options).map_err(NodejsError::new)?;
    let secret_manager = SecretManager::try_from(secret_manager_dto).map_err(NodejsError::new)?;

    Ok(External::new(Arc::new(RwLock::new(secret_manager))))
}

#[napi(js_name = "callSecretManagerMethod")]
pub async fn call_secret_manager_method(
    secret_manager: External<SecretManagerMethodHandler>,
    method: String,
) -> Result<String> {
    let secret_manager_method = serde_json::from_str::<SecretManagerMethod>(&method).map_err(NodejsError::new)?;

    let res = rust_call_secret_manager_method(&*secret_manager.as_ref().read().await, secret_manager_method).await;
    if matches!(res, Response::Error(_) | Response::Panic(_)) {
        return Err(Error::new(
            Status::GenericFailure,
            serde_json::to_string(&res).map_err(NodejsError::new)?,
        ));
    }

    Ok(serde_json::to_string(&res).map_err(NodejsError::new)?)
}

#[napi(js_name = "migrateStrongholdSnapshotV2ToV3")]
pub fn migrate_stronghold_snapshot_v2_to_v3(
    current_path: String,
    current_password: String,
    salt: String,
    rounds: u32,
    new_path: Option<String>,
    new_password: Option<String>,
) -> Result<()> {
    let current_password = current_password.into();
    let new_password = new_password.map(Into::into);

    StrongholdAdapter::migrate_snapshot_v2_to_v3(
        &current_path,
        current_password,
        salt,
        rounds,
        new_path.as_ref(),
        new_password,
    )
    .map_err(iota_sdk_bindings_core::iota_sdk::client::Error::from)
    .map_err(NodejsError::new)?;

    Ok(())
}
