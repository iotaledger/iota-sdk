// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::{From, Infallible};
use std::{
    cell::RefCell,
    ffi::{c_char, CString},
    ptr,
};

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub error: String,
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error { error: err.to_string() }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error { error: err.to_string() }
    }
}

impl From<iota_sdk_bindings_core::iota_sdk::types::block::Error> for Error {
    fn from(err: iota_sdk_bindings_core::iota_sdk::types::block::Error) -> Self {
        Error { error: err.to_string() }
    }
}

impl From<iota_sdk_bindings_core::Error> for Error {
    fn from(err: iota_sdk_bindings_core::Error) -> Self {
        Error { error: err.to_string() }
    }
}

impl From<iota_sdk_bindings_core::iota_sdk::client::Error> for Error {
    fn from(err: iota_sdk_bindings_core::iota_sdk::client::Error) -> Self {
        Error { error: err.to_string() }
    }
}

impl From<iota_sdk_bindings_core::iota_sdk::wallet::Error> for Error {
    fn from(err: iota_sdk_bindings_core::iota_sdk::wallet::Error) -> Self {
        Error { error: err.to_string() }
    }
}

impl From<Infallible> for Error {
    fn from(err: Infallible) -> Self {
        Error { error: err.to_string() }
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Self { error: err.to_string() }
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self { error: err }
    }
}

thread_local! {
    #[allow(clippy::box_collection)]
    static LAST_ERROR: RefCell<Option<Box<String>>> = RefCell::new(None);
}

pub fn set_last_error(err: Error) {
    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(Box::new(err.error));
    });
}

#[no_mangle]
pub unsafe extern "C" fn binding_get_last_error() -> *const c_char {
    let last_error = LAST_ERROR.with(|prev| prev.borrow_mut().take());

    let last_error = match last_error {
        Some(err) => err,
        None => return ptr::null_mut(),
    };

    let s = CString::new(last_error.to_string()).unwrap();
    s.into_raw()
}
