// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use dialoguer::Error as DialoguerError;
use fern_logger::Error as LoggerError;
use iota_sdk::{
    client::error::Error as ClientError, types::block::Error as BlockError, wallet::error::Error as WalletError,
};
use rustyline::error::ReadlineError;
use serde_json::Error as SerdeJsonError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("block error: {0}")]
    Block(#[from] BlockError),
    #[error("client error: {0}")]
    Client(Box<ClientError>),
    #[error("dialoguer error: {0}")]
    Dialoguer(#[from] DialoguerError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("logger error: {0}")]
    Logger(#[from] LoggerError),
    #[error("{0}")]
    Miscellaneous(String),
    #[error("generate at least one address before using the faucet")]
    NoAddressForFaucet,
    #[error("prompt error: {0}")]
    Prompt(#[from] ReadlineError),
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] SerdeJsonError),
    #[error("wallet error: {0}")]
    Wallet(#[from] WalletError),
}

impl From<ClientError> for Error {
    fn from(error: ClientError) -> Self {
        Error::Client(Box::new(error))
    }
}

impl From<iota_sdk::client::stronghold::Error> for Error {
    fn from(error: iota_sdk::client::stronghold::Error) -> Self {
        Self::Client(Box::new(iota_sdk::client::Error::Stronghold(error)))
    }
}
