// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::{From, Infallible};

use pyo3::{exceptions, prelude::*};

/// The `Result` structure to wrap the error type for python binding.
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// The Error type.
#[derive(Debug)]
pub struct Error {
    /// The error exposed to python.
    pub error: PyErr,
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

impl From<iota_sdk_bindings_core::iota_sdk::types::block::Error> for Error {
    fn from(err: iota_sdk_bindings_core::iota_sdk::types::block::Error) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<iota_sdk_bindings_core::Error> for Error {
    fn from(err: iota_sdk_bindings_core::Error) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<iota_sdk_bindings_core::iota_sdk::client::Error> for Error {
    fn from(err: iota_sdk_bindings_core::iota_sdk::client::Error) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<iota_sdk_bindings_core::iota_sdk::wallet::Error> for Error {
    fn from(err: iota_sdk_bindings_core::iota_sdk::wallet::Error) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<Infallible> for Error {
    fn from(err: Infallible) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}
