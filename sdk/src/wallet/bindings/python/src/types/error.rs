// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::convert::From;

use iota_sdk::{client::Error as ClientError, wallet::Error as WalletError};
use pyo3::{exceptions, prelude::*};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub error: PyErr,
}

impl From<ClientError> for Error {
    fn from(err: ClientError) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<WalletError> for Error {
    fn from(err: WalletError) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<Error> for PyErr {
    fn from(err: Error) -> Self {
        err.error
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyIOError, _>(err.to_string()),
        }
    }
}
