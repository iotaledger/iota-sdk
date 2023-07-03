// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod client;
mod error;
mod secret_manager;
mod wallet;

use std::ffi::{c_char, CStr, CString};
use std::ptr::null;
use std::sync::Mutex;

use crate::error::set_last_error;
use iota_sdk_bindings_core::{
    call_utils_method as rust_call_utils_method, init_logger as rust_init_logger, UtilsMethod,
};
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

use self::{
    error::{Error, Result},
    secret_manager::*,
};

/// Use one runtime.
pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
    static INSTANCE: OnceCell<Mutex<Runtime>> = OnceCell::new();
    let runtime = INSTANCE.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
    runtime.lock().unwrap().block_on(cb)
}

unsafe fn internal_destroy_string(ptr: *mut c_char) -> Result<()> {
    if ptr.is_null() {
        return Ok(());
    }

    let _ = CString::from_raw(ptr);
    Ok(())
}

#[no_mangle]
unsafe extern "C" fn destroy_string(ptr: *mut c_char) -> bool {
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

#[no_mangle]
unsafe extern "C" fn init_logger(config_ptr: *const c_char) -> bool {
    match internal_init_logger(config_ptr) {
        Ok(_) => true,
        Err(e) => {
            set_last_error(e);
            false
        }
    }
}

#[no_mangle]
unsafe fn internal_call_utils_method(method_ptr: *const c_char) -> Result<*const c_char> {
    let method_str = CStr::from_ptr(method_ptr).to_str().unwrap();

    let method = serde_json::from_str::<UtilsMethod>(&method_str)?;
    let response = rust_call_utils_method(method);

    let response_string = serde_json::to_string(&response)?;
    let s = CString::new(response_string).unwrap();

    Ok(s.into_raw())
}

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

/*
/// Migrates a stronghold snapshot from v2 to v3.
#[no_mangle]
pub extern "C" fn migrate_stronghold_snapshot_v2_to_v3(
    current_path: String,
    current_password: String,
    salt: &str,
    rounds: u32,
    new_path: Option<String>,
    new_password: Option<String>,
) -> Result<()> {
    Ok(StrongholdAdapter::migrate_snapshot_v2_to_v3(
        &current_path,
        current_password.into(),
        salt,
        rounds,
        new_path.as_ref(),
        new_password.map(Into::into),
    )
    .map_err(iota_sdk_bindings_core::iota_sdk::client::Error::Stronghold)?)
}*/
