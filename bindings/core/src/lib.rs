// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Core library for iota-sdk bindings

mod error;
mod method;
mod method_handler;
mod panic;
mod response;

use std::fmt::{Formatter, Result as FmtResult};

use derivative::Derivative;
use fern_logger::{logger_init, LoggerConfig, LoggerOutputConfigBuilder};
pub use iota_sdk;
use iota_sdk::{
    client::secret::{SecretManager, SecretManagerDto},
    wallet::{wallet::Wallet, ClientOptions},
};
use serde::Deserialize;

#[cfg(feature = "mqtt")]
pub use self::method_handler::listen_mqtt;
pub use self::{
    error::{Error, Result},
    method::{AccountMethod, ClientMethod, SecretManagerMethod, UtilsMethod, WalletMethod},
    method_handler::{
        call_account_method, call_client_method, call_secret_manager_method, call_utils_method, call_wallet_method,
    },
    response::Response,
};

pub fn init_logger(config: String) -> std::result::Result<(), fern_logger::Error> {
    let output_config: LoggerOutputConfigBuilder = serde_json::from_str(&config).expect("invalid logger config");
    let config = LoggerConfig::build().with_output(output_config).finish();
    logger_init(config)
}

#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletOptions {
    pub storage_path: Option<String>,
    pub client_options: Option<ClientOptions>,
    pub coin_type: Option<u32>,
    #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
    pub secret_manager: Option<SecretManagerDto>,
}

impl WalletOptions {
    pub async fn build_manager(&self) -> iota_sdk::wallet::Result<Wallet> {
        log::debug!("build_manager {self:?}");
        let mut builder = Wallet::builder();

        #[cfg(feature = "storage")]
        if let Some(storage_path) = &self.storage_path {
            builder = builder.with_storage_path(storage_path);
        }

        if let Some(secret_manager) = &self.secret_manager {
            builder = builder.with_secret_manager(SecretManager::try_from(secret_manager)?);
        }

        if let Some(client_options) = &self.client_options {
            builder = builder.with_client_options(client_options.clone());
        }

        if let Some(coin_type) = self.coin_type {
            builder = builder.with_coin_type(coin_type);
        }

        builder.finish().await
    }
}

pub(crate) trait OmittedDebug {
    fn omitted_fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<omitted>")
    }
}
impl OmittedDebug for String {}
impl OmittedDebug for SecretManagerDto {}
impl<T: OmittedDebug> OmittedDebug for Option<T> {
    fn omitted_fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Some(_) => f.write_str("Some(<omitted>)"),
            None => f.write_str("None"),
        }
    }
}
