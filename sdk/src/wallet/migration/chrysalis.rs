// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use rocksdb::{IteratorMode, DB};

use crate::wallet::{
    storage::{adapter::rocksdb::RocksdbStorageAdapter, Storage},
    Error,
};

fn storage_password_to_encryption_key(password: &str) -> [u8; 32] {
    let mut dk = [0; 64];
    // safe to unwrap (rounds > 0)
    crypto::keys::pbkdf::PBKDF2_HMAC_SHA512(password.as_bytes(), b"wallet.rs::storage", 100, &mut dk).unwrap();
    let key: [u8; 32] = dk[0..32][..].try_into().unwrap();
    key
}

pub async fn migrate(storage: &Storage, password: Option<&str>) -> Result<(), Error> {
    // let rocksdb = (&&*storage.inner as &dyn any::Any)
    //     .downcast_ref::<RocksdbStorageAdapter>()
    //     .unwrap()
    //     .clone();
    // let db = rocksdb.db.lock().await;
    let chrysaslis_db = DB::open_default("chrysalis_storage").unwrap();
    let stardust_db = DB::open_default("stardust_storage").unwrap();

    // iterate over all rocksdb keys
    let mut map = HashMap::new();
    for item in chrysaslis_db.iterator(IteratorMode::Start) {
        let (key, mut value) = item.unwrap();
        if let Some(encryption_key) = password.map(storage_password_to_encryption_key) {
            // value = decrypt(value, encryption_key);
        }
        println!("Saw {:?} {:?}", key, value);
        map.insert(key, value);
    }

    stardust_db.put("CHRYSALIS_STORAGE", serde_json::to_string(&map)?)?;

    std::fs::remove_dir_all("chrysalis_storage")?;

    // create new accounts base on previous data

    Ok(())
}
