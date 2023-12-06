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
    types::block::address::Bech32Address,
    wallet::{ClientOptions, Wallet},
};
use serde::Deserialize;

#[cfg(feature = "mqtt")]
pub use self::method_handler::listen_mqtt;
#[cfg(not(target_family = "wasm"))]
pub use self::method_handler::CallMethod;
pub use self::{
    error::{Error, Result},
    method::{ClientMethod, SecretManagerMethod, UtilsMethod, WalletMethod},
    method_handler::{call_client_method, call_secret_manager_method, call_utils_method, call_wallet_method},
    response::Response,
};

pub fn init_logger(config: String) -> std::result::Result<(), fern_logger::Error> {
    let output_config: LoggerOutputConfigBuilder = serde_json::from_str(&config).expect("invalid logger config");
    let config = LoggerConfig::build().with_output(output_config).finish();
    logger_init(config)
}

#[derive(Derivative, Deserialize, Default)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletOptions {
    pub address: Option<Bech32Address>,
    pub alias: Option<String>,
    #[serde(default)]
    pub public_key_options: Option<serde_json::Value>,
    #[serde(default)]
    pub signing_options: Option<serde_json::Value>,
    pub client_options: Option<ClientOptions>,
    #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
    pub secret_manager: Option<SecretManagerDto>,
    pub storage_path: Option<String>,
}

impl WalletOptions {
    pub fn with_address(mut self, address: impl Into<Option<Bech32Address>>) -> Self {
        self.address = address.into();
        self
    }

    pub fn with_alias(mut self, alias: impl Into<Option<String>>) -> Self {
        self.alias = alias.into();
        self
    }

    pub fn with_public_key_options(mut self, public_key_options: impl Into<Option<serde_json::Value>>) -> Self {
        self.public_key_options = public_key_options.into();
        self
    }

    pub fn with_signing_options(mut self, signing_options: impl Into<Option<serde_json::Value>>) -> Self {
        self.signing_options = signing_options.into();
        self
    }

    pub fn with_client_options(mut self, client_options: impl Into<Option<ClientOptions>>) -> Self {
        self.client_options = client_options.into();
        self
    }

    pub fn with_secret_manager(mut self, secret_manager: impl Into<Option<SecretManagerDto>>) -> Self {
        self.secret_manager = secret_manager.into();
        self
    }

    #[cfg(feature = "storage")]
    pub fn with_storage_path(mut self, storage_path: impl Into<Option<String>>) -> Self {
        self.storage_path = storage_path.into();
        self
    }

    pub async fn build(self) -> iota_sdk::wallet::Result<Wallet<SecretManager>> {
        log::debug!("wallet options: {self:?}");
        let mut builder = Wallet::builder()
            .with_address(self.address)
            .with_alias(self.alias)
            .with_public_key_options(self.public_key_options)
            .with_signing_options(self.signing_options)
            .with_client_options(self.client_options);

        #[cfg(feature = "storage")]
        if let Some(storage_path) = &self.storage_path {
            builder = builder.with_storage_path(storage_path);
        }

        if let Some(secret_manager) = self.secret_manager {
            builder = builder.with_secret_manager(SecretManager::try_from(secret_manager)?);
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
