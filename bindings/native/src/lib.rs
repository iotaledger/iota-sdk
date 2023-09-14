// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod client;
mod error;
mod secret_manager;
mod wallet;

use std::{
    ffi::{c_char, CStr, CString},
    ptr::null,
    sync::Mutex,
};

use iota_sdk_bindings_core::{
    call_utils_method as rust_call_utils_method, init_logger as rust_init_logger, UtilsMethod,
};
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;
use zeroize::Zeroize;

use self::{
    error::{Error, Result},
    secret_manager::*,
};
use crate::error::set_last_error;

/// Use one runtime.
pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
    static INSTANCE: OnceCell<Mutex<Runtime>> = OnceCell::new();
    let runtime = INSTANCE.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
    runtime.lock().unwrap().block_on(cb)
}

unsafe fn internal_destroy_string(ptr: *mut c_char) -> Result<()> {
    if ptr.is_null() {
        log::error!("[Rust] String pointer was null!");
        return Err(Error::from("pointer is null"));
    }

    let mut str = CString::from_raw(ptr);
    str.zeroize();

    Ok(())
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn destroy_string(ptr: *mut c_char) -> bool {
    match internal_destroy_string(ptr) {
        Ok(_) => true,
        Err(e) => {
            set_last_error(e);
            false
        }
    }
}

/// Init the Rust logger.
unsafe fn internal_init_logger(config_ptr: *const c_char) -> Result<()> {
    let method_str = CStr::from_ptr(config_ptr).to_str().unwrap();
    rust_init_logger(method_str.to_string()).map_err(|err| Error::from(format!("{:?}", err)))?;
    Ok(())
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn init_logger(config_ptr: *const c_char) -> bool {
    match internal_init_logger(config_ptr) {
        Ok(_) => true,
        Err(e) => {
            set_last_error(e);
            false
        }
    }
}

unsafe fn internal_call_utils_method(method_ptr: *const c_char) -> Result<*const c_char> {
    let method_str = CStr::from_ptr(method_ptr).to_str().unwrap();

    let method = serde_json::from_str::<UtilsMethod>(method_str)?;
    let response = rust_call_utils_method(method);

    let response_string = serde_json::to_string(&response)?;
    let s = CString::new(response_string).unwrap();

    Ok(s.into_raw())
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn call_utils_method(config_ptr: *const c_char) -> *const c_char {
    match internal_call_utils_method(config_ptr) {
        Ok(v) => v,
        Err(e) => {
            set_last_error(e);
            null()
        }
    }
}
