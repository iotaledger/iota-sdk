// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    ffi::{c_char, CStr, CString},
    ptr::null,
};

use iota_sdk_bindings_core::{
    call_client_method as rust_call_client_method,
    iota_sdk::client::{Client as RustClient, ClientBuilder},
    ClientMethod,
};

use crate::error::{set_last_error, Error, Result};

pub struct Client {
    pub client: RustClient,
}

unsafe fn internal_create_client(options_ptr: *const c_char) -> Result<*const Client> {
    let options_string = CStr::from_ptr(options_ptr).to_str().unwrap();
    let runtime = tokio::runtime::Runtime::new()?;

    let client = runtime.block_on(async move {
        if options_string.is_empty() {
            return ClientBuilder::new().finish().await;
        }

        ClientBuilder::new().from_json(options_string)?.finish().await
    })?;

    let client_wrap = Client { client };
    let client_ptr = Box::into_raw(Box::new(client_wrap));

    Ok(client_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn create_client(options_ptr: *const c_char) -> *const Client {
    match internal_create_client(options_ptr) {
        Ok(v) => v,
        Err(e) => {
            set_last_error(e);
            null()
        }
    }
}

unsafe fn internal_destroy_client(client_ptr: *mut Client) -> Result<()> {
    log::debug!("[Rust] Client destroy called");

    if client_ptr.is_null() {
        log::error!("[Rust] Client pointer was null!");
        return Err(Error::from("pointer is null"));
    }

    let _ = Box::from_raw(client_ptr);

    log::debug!("[Rust] Destroyed client");
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn destroy_client(client_ptr: *mut Client) -> bool {
    match internal_destroy_client(client_ptr) {
        Ok(_) => true,
        Err(e) => {
            set_last_error(e);
            false
        }
    }
}

unsafe fn internal_call_client_method(client_ptr: *mut Client, method_ptr: *mut c_char) -> Result<*const c_char> {
    let method_str = CStr::from_ptr(method_ptr).to_str().unwrap();

    let client = {
        assert!(!client_ptr.is_null());
        &mut *client_ptr
    };

    let method = serde_json::from_str::<ClientMethod>(method_str)?;
    let response = crate::block_on(async { rust_call_client_method(&client.client, method).await });

    let response_string = serde_json::to_string(&response)?;
    let s = CString::new(response_string).unwrap();

    Ok(s.into_raw())
}

#[no_mangle]
pub unsafe extern "C" fn call_client_method(client_ptr: *mut Client, method_ptr: *mut c_char) -> *const c_char {
    match internal_call_client_method(client_ptr, method_ptr) {
        Ok(v) => v,
        Err(e) => {
            set_last_error(e);
            null()
        }
    }
}
