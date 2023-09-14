// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    ffi::{c_char, CStr, CString},
    ptr::null,
    sync::Arc,
};

use iota_sdk_bindings_core::{
    call_wallet_method as rust_call_wallet_method,
    iota_sdk::wallet::{events::types::WalletEventType, Wallet as RustWallet},
    Response, WalletMethod, WalletOptions,
};
use log::debug;
use tokio::sync::RwLock;

use crate::{
    client::Client,
    error::{set_last_error, Error, Result},
    SecretManager,
};

pub struct Wallet {
    pub wallet: Arc<RwLock<Option<RustWallet>>>,
}

unsafe fn internal_destroy_wallet(wallet_ptr: *mut Wallet) -> Result<()> {
    let wallet = {
        assert!(!wallet_ptr.is_null());
        &mut *wallet_ptr
    };

    crate::block_on(async {
        *wallet.wallet.write().await = None;
    });
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn destroy_wallet(wallet_ptr: *mut Wallet) -> bool {
    match internal_destroy_wallet(wallet_ptr) {
        Ok(_) => true,
        Err(e) => {
            set_last_error(e);
            false
        }
    }
}

unsafe fn internal_create_wallet(options_ptr: *const c_char) -> Result<*mut Wallet> {
    let options_string = CStr::from_ptr(options_ptr).to_str().unwrap();

    let wallet_options = serde_json::from_str::<WalletOptions>(options_string)?;
    let wallet = crate::block_on(async { wallet_options.build().await })?;

    let wallet_wrap = Wallet {
        wallet: Arc::new(RwLock::new(Some(wallet))),
    };

    let wallet_ptr = Box::into_raw(Box::new(wallet_wrap));

    Ok(wallet_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn create_wallet(options_ptr: *const c_char) -> *const Wallet {
    match internal_create_wallet(options_ptr) {
        Ok(v) => v,
        Err(e) => {
            set_last_error(e);
            null()
        }
    }
}

unsafe fn internal_call_wallet_method(wallet_ptr: *mut Wallet, method_ptr: *const c_char) -> Result<*const c_char> {
    let wallet = {
        assert!(!wallet_ptr.is_null());
        &mut *wallet_ptr
    };

    let method_string = CStr::from_ptr(method_ptr).to_str().unwrap();
    let method = serde_json::from_str::<WalletMethod>(method_string)?;

    let response = crate::block_on(async {
        match wallet.wallet.read().await.as_ref() {
            Some(wallet) => rust_call_wallet_method(wallet, method).await,
            None => Response::Panic("wallet got destroyed".into()),
        }
    });

    let response_string = serde_json::to_string(&response)?;
    let s = CString::new(response_string).unwrap();

    Ok(s.into_raw())
}

#[no_mangle]
pub unsafe extern "C" fn call_wallet_method(wallet_ptr: *mut Wallet, method_ptr: *const c_char) -> *const c_char {
    match internal_call_wallet_method(wallet_ptr, method_ptr) {
        Ok(v) => v,
        Err(e) => {
            set_last_error(e);
            null()
        }
    }
}

unsafe fn internal_listen_wallet(
    wallet_ptr: *mut Wallet,
    events_ptr: *const c_char,
    handler: extern "C" fn(*const c_char),
) -> Result<bool> {
    let wallet = {
        assert!(!wallet_ptr.is_null());
        &mut *wallet_ptr
    };

    let events_string = CStr::from_ptr(events_ptr).to_str().unwrap();
    let rust_events = serde_json::from_str::<Vec<String>>(events_string);

    if rust_events.is_err() {
        return Ok(false);
    }

    let mut wallet_events: Vec<WalletEventType> = Vec::new();
    for event in rust_events.unwrap() {
        let event = match serde_json::from_str::<WalletEventType>(&event) {
            Ok(event) => event,
            Err(e) => {
                debug!("Wrong event to listen! {e:?}");
                return Ok(false);
            }
        };
        wallet_events.push(event);
    }

    crate::block_on(async {
        wallet
            .wallet
            .read()
            .await
            .as_ref()
            .expect("wallet got destroyed")
            .listen(wallet_events, move |event_data| {
                if let Ok(event_str) = serde_json::to_string(event_data) {
                    let s = CString::new(event_str).unwrap();
                    handler(s.into_raw())
                }
            })
            .await
    });

    Ok(true)
}

#[no_mangle]
pub unsafe extern "C" fn listen_wallet(
    wallet_ptr: *mut Wallet,
    events: *const c_char,
    handler: extern "C" fn(*const c_char),
) -> bool {
    match internal_listen_wallet(wallet_ptr, events, handler) {
        Ok(v) => v,
        Err(e) => {
            set_last_error(e);
            false
        }
    }
}

unsafe fn internal_get_client_from_wallet(wallet_ptr: *mut Wallet) -> Result<*const Client> {
    let wallet = {
        assert!(!wallet_ptr.is_null());
        &mut *wallet_ptr
    };

    let client = crate::block_on(async {
        wallet
            .wallet
            .read()
            .await
            .as_ref()
            .map(|w| w.client().clone())
            .ok_or_else(|| {
                Error::from(
                    serde_json::to_string(&Response::Panic("wallet got destroyed".into()))
                        .expect("json to string error")
                        .as_str(),
                )
            })
    })?;

    let client_wrap = Client { client };
    let client_ptr = Box::into_raw(Box::new(client_wrap));

    Ok(client_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn get_client_from_wallet(wallet_ptr: *mut Wallet) -> *const Client {
    match internal_get_client_from_wallet(wallet_ptr) {
        Ok(v) => v,
        Err(e) => {
            set_last_error(e);
            null()
        }
    }
}

unsafe fn internal_get_secret_manager_from_wallet(wallet_ptr: *mut Wallet) -> Result<*const SecretManager> {
    let wallet = {
        assert!(!wallet_ptr.is_null());
        &mut *wallet_ptr
    };

    let secret_manager = crate::block_on(async {
        wallet
            .wallet
            .read()
            .await
            .as_ref()
            .map(|w| w.get_secret_manager().clone())
            .ok_or_else(|| {
                Error::from(
                    serde_json::to_string(&Response::Panic("wallet got destroyed".into()))
                        .expect("json to string error")
                        .as_str(),
                )
            })
    })?;

    let secret_manager_wrap = SecretManager { secret_manager };
    let secret_manager_ptr = Box::into_raw(Box::new(secret_manager_wrap));

    Ok(secret_manager_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn get_secret_manager_from_wallet(wallet_ptr: *mut Wallet) -> *const SecretManager {
    match internal_get_secret_manager_from_wallet(wallet_ptr) {
        Ok(v) => v,
        Err(e) => {
            set_last_error(e);
            null()
        }
    }
}
