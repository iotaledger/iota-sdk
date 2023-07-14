// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account_method;
mod message;
mod message_handler;
mod response;

use fern_logger::{logger_init, LoggerConfig, LoggerOutputConfigBuilder};
use iota_sdk::{
    client::secret::{SecretManager, SecretManagerDto},
    wallet::{ClientOptions, ClientOptionsDto, Wallet},
};
use serde::{Deserialize, Serialize, Serializer};

pub use self::{
    account_method::AccountMethod, message::Message, message_handler::WalletMessageHandler, response::Response,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManagerOptions {
    pub storage_path: Option<String>,
    pub client_options: Option<ClientOptionsDto>,
    pub coin_type: Option<u32>,
    #[serde(serialize_with = "secret_manager_serialize")]
    pub secret_manager: Option<SecretManagerDto>,
}

// Serialize secret manager with secrets removed
fn secret_manager_serialize<S>(secret_manager: &Option<SecretManagerDto>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(secret_manager) = secret_manager {
        match secret_manager {
            SecretManagerDto::HexSeed(_) => s.serialize_str("hexSeed(<omitted>)"),
            #[cfg(feature = "ledger_nano")]
            SecretManagerDto::LedgerNano(is_simulator) => s.serialize_str(&format!("ledgerNano({is_simulator})")),
            SecretManagerDto::Mnemonic(_) => s.serialize_str("mnemonic(<omitted>)"),
            SecretManagerDto::Placeholder => s.serialize_str("placeholder"),
            #[cfg(feature = "stronghold")]
            SecretManagerDto::Stronghold(stronghold) => {
                let mut stronghold_dto = stronghold.clone();
                // Remove password
                stronghold_dto.password = None;
                s.serialize_str(&format!("{stronghold_dto:?}"))
            }
        }
    } else {
        s.serialize_str("null")
    }
}

pub fn init_logger(config: String) -> Result<(), fern_logger::Error> {
    let output_config: LoggerOutputConfigBuilder = serde_json::from_str(&config).expect("invalid logger config");
    let config = LoggerConfig::build().with_output(output_config).finish();
    logger_init(config)
}

pub async fn create_message_handler(options: Option<ManagerOptions>) -> iota_sdk::wallet::Result<WalletMessageHandler> {
    log::debug!(
        "create_message_handler with options: {}",
        serde_json::to_string(&options)?,
    );
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
            builder = builder.with_client_options(ClientOptions::try_from(client_options)?);
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
