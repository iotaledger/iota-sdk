// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    io::Read,
    path::Path,
};

use crypto::ciphers::{chacha::XChaCha20Poly1305, traits::Aead};
use rocksdb::{IteratorMode, DB};

use crate::{
    client::constants::IOTA_COIN_TYPE,
    types::block::address::Bech32Address,
    wallet::{
        account::{types::AccountAddress, AccountDetailsDto},
        Error,
    },
};

pub async fn migrate_db_from_chrysalis_to_stardust<P: AsRef<Path>>(
    chrysalis_storage_path: P,
    _stardust_storage_path: P,
    password: Option<&str>,
) -> Result<(), Error> {
    // let rocksdb = (&&*storage.inner as &dyn any::Any)
    //     .downcast_ref::<RocksdbStorageAdapter>()
    //     .unwrap()
    //     .clone();
    // let db = rocksdb.db.lock().await;
    let chrysaslis_db = DB::open_default(chrysalis_storage_path).unwrap();
    // let stardust_db = DB::open_default("stardust_storage").unwrap();

    // iterate over all rocksdb keys
    let mut map = HashMap::new();
    let encryption_key = password.map(storage_password_to_encryption_key);
    for item in chrysaslis_db.iterator(IteratorMode::Start) {
        // let (key, mut value) = item.unwrap();
        let (key, value) = item.unwrap();

        let value = if let Some(encryption_key) = encryption_key {
            // TODO: check if this is correct also with encrypted values
            match String::from_utf8(value.to_vec()) {
                Ok(value) => value,
                Err(_) => decrypt_record(&value, &encryption_key)?,
            }
        } else {
            String::from_utf8(value.to_vec()).map_err(|_| Error::Migration("invalid utf8".into()))?
        };

        map.insert(
            String::from_utf8(key.to_vec()).map_err(|_| Error::Migration("invalid utf8".into()))?,
            value,
        );
    }

    let mut new_accounts = Vec::new();
    if let Some(account_indexation) = map.get("iota-wallet-account-indexation") {
        if let Some(account_keys) = serde_json::from_str::<serde_json::Value>(account_indexation)?.as_array() {
            for account_key in account_keys {
                if let Some(account_data) = map.get(account_key["key"].as_str().expect("key must be a string")) {
                    let account_data = serde_json::from_str::<serde_json::Value>(account_data)?;

                    let mut account_addresses = Vec::new();

                    if let Some(addresses) = account_data["addresses"].as_array() {
                        for address in addresses {
                            account_addresses.push(AccountAddress {
                                address: Bech32Address::from_str(address["address"].as_str().unwrap())?,
                                key_index: address["keyIndex"].as_u64().unwrap() as u32,
                                internal: address["internal"].as_bool().unwrap(),
                                used: !address["outputs"].as_object().unwrap().is_empty(),
                            })
                        }
                    }
                    let (internal, public): (Vec<AccountAddress>, Vec<AccountAddress>) =
                        account_addresses.into_iter().partition(|a| a.internal);

                    new_accounts.push(AccountDetailsDto {
                        index: account_data["index"].as_u64().unwrap() as u32,
                        coin_type: IOTA_COIN_TYPE,
                        alias: account_data["alias"].as_str().unwrap().to_string(),
                        public_addresses: public,
                        internal_addresses: internal,
                        addresses_with_unspent_outputs: Vec::new(),
                        outputs: HashMap::new(),
                        unspent_outputs: HashMap::new(),
                        transactions: HashMap::new(),
                        pending_transactions: HashSet::new(),
                        locked_outputs: HashSet::new(),
                        incoming_transactions: HashMap::new(),
                        native_token_foundries: HashMap::new(),
                    })
                }
            }
        }
    }

    println!("{}", serde_json::to_string_pretty(&new_accounts)?);

    // TODO:
    // clear old key (remove db, create new one?)
    // store chrysalis data in a new key
    // write new accounts to db (with account indexation)

    // println!("{}", serde_json::to_string_pretty(&map)?);

    // stardust_db.put("CHRYSALIS_STORAGE", serde_json::to_string(&map)?)?;

    // std::fs::remove_dir_all("chrysalis_storage")?;

    // create new accounts base on previous data

    Ok(())
}

fn storage_password_to_encryption_key(password: &str) -> [u8; 32] {
    let mut dk = [0; 64];
    // safe to unwrap (rounds > 0)
    crypto::keys::pbkdf::PBKDF2_HMAC_SHA512(
        password.as_bytes(),
        b"wallet.rs::storage",
        core::num::NonZeroU32::new(100).unwrap(),
        &mut dk,
    );
    let key: [u8; 32] = dk[0..32][..].try_into().unwrap();
    key
}

fn decrypt_record(record: &[u8], encryption_key: &[u8; 32]) -> crate::wallet::Result<String> {
    // let record: Vec<u8> = serde_json::from_str(record)?;
    let mut record: &[u8] = &record;

    let mut nonce = [0; XChaCha20Poly1305::NONCE_LENGTH];
    record.read_exact(&mut nonce)?;

    let mut tag = vec![0; XChaCha20Poly1305::TAG_LENGTH];
    record.read_exact(&mut tag)?;

    let mut ct = Vec::new();
    record.read_to_end(&mut ct)?;

    let mut pt = vec![0; ct.len()];
    // we can unwrap here since we know the lengths are valid
    XChaCha20Poly1305::decrypt(
        encryption_key.try_into().unwrap(),
        &nonce.try_into().unwrap(),
        &[],
        &mut pt,
        &ct,
        tag.as_slice().try_into().unwrap(),
    )
    .map_err(|e| Error::Migration(format!("{:?}", e)))?;

    Ok(String::from_utf8_lossy(&pt).to_string())
}
