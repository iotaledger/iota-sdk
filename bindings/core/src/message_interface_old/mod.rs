// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account_method;
mod message;
mod message_handler;
mod response;

use fern_logger::{logger_init, LoggerConfig, LoggerOutputConfigBuilder};
use iota_sdk::{client::secret::SecretManager, wallet::Wallet};

pub use self::{
    account_method::AccountMethod, message::Message, message_handler::WalletMessageHandler, response::Response,
};
use crate::WalletOptions;

pub fn init_logger(config: String) -> Result<(), fern_logger::Error> {
    let output_config: LoggerOutputConfigBuilder = serde_json::from_str(&config).expect("invalid logger config");
    let config = LoggerConfig::build().with_output(output_config).finish();
    logger_init(config)
}

pub async fn create_message_handler(options: Option<WalletOptions>) -> iota_sdk::wallet::Result<WalletMessageHandler> {
    log::debug!("create_message_handler with options: {options:?}");
    let wallet = if let Some(options) = options {
        let mut builder = Wallet::builder();

        #[cfg(feature = "storage")]
        if let Some(storage_path) = options.storage_path {
            builder = builder.with_storage_path(&storage_path);
        }

        if let Some(secret_manager) = options.secret_manager {
            builder = builder.with_secret_manager(SecretManager::try_from(secret_manager)?);
        }

        if let Some(client_options) = options.client_options {
            builder = builder.with_client_options(client_options);
        }

        if let Some(coin_type) = options.coin_type {
            builder = builder.with_coin_type(coin_type);
        }

        builder.finish().await?
    } else {
        Wallet::builder().finish().await?
    };

    Ok(WalletMessageHandler::with_manager(wallet))
}
