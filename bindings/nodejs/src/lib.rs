// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod client;
pub mod secret_manager;
pub mod wallet;

use std::sync::Arc;

use iota_sdk_bindings_core::{
    call_utils_method as rust_call_utils_method, init_logger as rust_init_logger,
    iota_sdk::client::storage::StorageAdapter, Response, UtilsMethod,
};
use napi::{
    bindgen_prelude::External,
    threadsafe_function::{ErrorStrategy, ThreadsafeFunction},
    Error, Result, Status,
};
use napi_derive::napi;
use tokio::sync::RwLock;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum NodejsError {
    /// Bindings errors.
    #[error("{0}")]
    Bindings(#[from] iota_sdk_bindings_core::Error),
    /// Client errors.
    #[error("{0}")]
    Client(#[from] iota_sdk_bindings_core::iota_sdk::client::Error),
    /// Mqtt errors.
    #[error("{0}")]
    Mqtt(#[from] iota_sdk_bindings_core::iota_sdk::client::node_api::mqtt::Error),
    /// SerdeJson errors.
    #[error("{0}")]
    SerdeJson(#[from] serde_json::error::Error),
    /// IO error.
    #[error("`{0}`")]
    Io(#[from] std::io::Error),
    /// Wallet errors.
    #[error("{0}")]
    Wallet(#[from] iota_sdk_bindings_core::iota_sdk::wallet::Error),
}

impl From<NodejsError> for Error {
    fn from(error: NodejsError) -> Self {
        Error::new(Status::GenericFailure, error.to_string())
    }
}

#[napi(js_name = "initLogger")]
pub fn init_logger(config: String) -> Result<()> {
    match rust_init_logger(config) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::new(
            Status::GenericFailure,
            serde_json::to_string(&Response::Panic(err.to_string())).map_err(NodejsError::from)?,
        )),
    }
}

#[napi(js_name = "callUtilsMethodRust")]
pub fn call_utils_method(method_json: String) -> Result<String> {
    let method = match serde_json::from_str::<UtilsMethod>(&method_json) {
        Ok(method) => method,
        Err(err) => {
            return Ok(serde_json::to_string(&Response::Error(err.into())).map_err(NodejsError::from)?);
        }
    };
    let response = rust_call_utils_method(method);

    Ok(serde_json::to_string(&response).map_err(NodejsError::from)?)
}

/// A storage adapter that uses closures from the JS side.
pub struct NodeJsStorage {
    pub db_methods: Arc<RwLock<DatabaseMethods>>,
}

impl std::fmt::Debug for NodeJsStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "node_js_store")
    }
}

pub struct DatabaseMethods {
    pub(crate) get: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled>,
    pub(crate) set: ThreadsafeFunction<(String, String)>,
    pub(crate) delete: ThreadsafeFunction<String>,
}

#[async_trait::async_trait]
impl StorageAdapter for NodeJsStorage {
    type Error = napi::Error;

    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let db_methods = self.db_methods.read().await;

        let promise = db_methods
            .get
            .call_async::<napi::bindgen_prelude::Promise<Option<String>>>(Ok(key.into()))
            .await?;

        Ok(promise.await?.map(|v| v.into_bytes()))
    }

    async fn set_bytes(&self, key: &str, record: &[u8]) -> Result<()> {
        let db_methods = self.db_methods.write().await;

        db_methods
            .set
            .call_async::<napi::bindgen_prelude::Promise<()>>(Ok((
                key.to_owned(),
                std::str::from_utf8(record).unwrap().into(),
            )))
            .await?
            .await
    }

    async fn delete(&self, key: &str) -> Result<()> {
        let db_methods = self.db_methods.write().await;
        db_methods
            .delete
            .call_async::<napi::bindgen_prelude::Promise<()>>(Ok(key.into()))
            .await?
            .await
    }
}

#[napi(js_name = "implCustomDatabase")]
pub fn impl_custom_database(
    get_cb: ThreadsafeFunction<String>,
    set_cb: ThreadsafeFunction<(String, String)>,
    delete_cb: ThreadsafeFunction<String>,
) -> Result<External<NodeJsStorage>> {
    let db_methods = DatabaseMethods {
        get: get_cb,
        set: set_cb,
        delete: delete_cb,
    };

    let storage = NodeJsStorage {
        db_methods: Arc::new(RwLock::new(db_methods)),
    };

    Ok(External::new(storage))
}

#[napi(js_name = "testCustomDatabase")]
pub async fn test_custom_database(db: External<NodeJsStorage>) -> Result<()> {
    db.set_bytes("testKey", b"testValue").await.unwrap();

    let _v = db.get_bytes("testKey").await.unwrap();

    db.delete("testKey").await.unwrap();

    let _v = db.get_bytes("testKey").await.unwrap();

    db.set_bytes("testKey", b"finalValue").await.unwrap();

    Ok(())
}
