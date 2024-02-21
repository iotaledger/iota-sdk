// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::{From, Infallible};

use iota_sdk_bindings_core::iota_sdk::types::block::BlockError;
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
        Self {
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
        Self {
            error: PyErr::new::<exceptions::PyIOError, _>(err.to_string()),
        }
    }
}

impl From<BlockError> for Error {
    fn from(err: BlockError) -> Self {
        Self {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<iota_sdk_bindings_core::Error> for Error {
    fn from(err: iota_sdk_bindings_core::Error) -> Self {
        Self {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<iota_sdk_bindings_core::iota_sdk::client::Error> for Error {
    fn from(err: iota_sdk_bindings_core::iota_sdk::client::Error) -> Self {
        Self {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<iota_sdk_bindings_core::iota_sdk::client::mqtt::Error> for Error {
    fn from(err: iota_sdk_bindings_core::iota_sdk::client::mqtt::Error) -> Self {
        Self {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<iota_sdk_bindings_core::iota_sdk::wallet::Error> for Error {
    fn from(err: iota_sdk_bindings_core::iota_sdk::wallet::Error) -> Self {
        Self {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<Infallible> for Error {
    fn from(err: Infallible) -> Self {
        Self {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<PyErr> for Error {
    fn from(err: PyErr) -> Self {
        Self { error: err }
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Self {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self {
            error: PyErr::new::<exceptions::PyValueError, _>(err),
        }
    }
}
