// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    io::Read,
    path::Path,
};

use crypto::{
    ciphers::{chacha::XChaCha20Poly1305, traits::Aead},
    macs::hmac::HMAC_SHA512,
};
use rocksdb::{IteratorMode, Options, DB};
use serde::Serialize;
use serde_json::Value;
use zeroize::Zeroizing;

use crate::{
    client::{constants::IOTA_COIN_TYPE, Password},
    types::block::address::Bech32Address,
    wallet::{
        migration::{MigrationVersion, MIGRATION_VERSION_KEY},
        storage::constants::{
            ACCOUNTS_INDEXATION_KEY, ACCOUNT_INDEXATION_KEY, CHRYSALIS_STORAGE_KEY, SECRET_MANAGER_KEY,
            WALLET_INDEXATION_KEY,
        },
        Error, Result,
    },
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct AccountAddress {
    address: Bech32Address,
    key_index: u32,
    internal: bool,
    used: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccountDetailsDto {
    pub(crate) index: u32,
    coin_type: u32,
    alias: String,
    public_addresses: Vec<AccountAddress>,
    internal_addresses: Vec<AccountAddress>,
    addresses_with_unspent_outputs: Vec<Value>,
    outputs: HashMap<String, Value>,
    locked_outputs: HashSet<String>,
    unspent_outputs: HashMap<String, Value>,
    transactions: HashMap<String, Value>,
    pending_transactions: HashSet<String>,
    incoming_transactions: HashMap<String, Value>,
    native_token_foundries: HashMap<String, Value>,
}

pub async fn migrate_db_from_chrysalis_to_stardust(storage_path: String, password: Option<Password>) -> Result<()> {
    let storage_path = Path::new(&storage_path);
    // `/db` will be appended to the chrysalis storage path, because that's how it was done in the chrysalis wallet
    let chrysalis_storage_path = &(*storage_path).join("db");

    let chrysalis_data = get_chrysalis_data(chrysalis_storage_path, password)?;

    // create new accounts base on previous data
    let (new_accounts, secret_manager_dto) = migrate_from_chrysalis_data(&chrysalis_data, storage_path, false)?;

    // convert to string keys
    let chrysalis_data_with_string_keys = chrysalis_data
        .into_iter()
        .map(|(k, v)| {
            Ok((
                String::from_utf8(k).map_err(|_| Error::Migration("invalid utf8".into()))?,
                v,
            ))
        })
        .collect::<Result<HashMap<String, String>>>()?;

    log::debug!(
        "Chrysalis data: {}",
        serde_json::to_string_pretty(&chrysalis_data_with_string_keys)?
    );

    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);
    let stardust_db = DB::open(&opts, storage_path).unwrap();

    // store chrysalis data in a new key
    stardust_db.put(
        CHRYSALIS_STORAGE_KEY,
        serde_json::to_string(&chrysalis_data_with_string_keys)?,
    )?;
    // write new accounts to db (with account indexation)
    let accounts_indexation_data: Vec<u32> = new_accounts.iter().map(|account| account.index).collect();
    stardust_db.put(
        ACCOUNTS_INDEXATION_KEY,
        serde_json::to_string(&accounts_indexation_data)?,
    )?;
    for new_account in new_accounts {
        stardust_db.put(
            &format!("{ACCOUNT_INDEXATION_KEY}{}", new_account.index),
            serde_json::to_string(&new_account)?,
        )?;
    }

    if let Some(secret_manager_dto) = secret_manager_dto {
        // This is required for the secret manager to be loaded
        stardust_db.put(WALLET_INDEXATION_KEY, format!("{{ \"coinType\": {IOTA_COIN_TYPE}}}"))?;
        stardust_db.put(SECRET_MANAGER_KEY, secret_manager_dto)?;
    }

    // set db migration version
    let migration_version = MigrationVersion {
        id: 4,
        sdk_version: "1.0.0-rc.0".to_string(),
        date: time::macros::date!(2023 - 07 - 19),
    };
    stardust_db.put(MIGRATION_VERSION_KEY, serde_json::to_string(&migration_version)?)?;

    drop(stardust_db);

    // remove old db
    std::fs::remove_dir_all(chrysalis_storage_path)?;

    Ok(())
}

pub(crate) fn migrate_from_chrysalis_data(
    chrysalis_data: &HashMap<Vec<u8>, String>,
    storage_path: &Path,
    // in stronghold the keys are hashed first
    stronghold: bool,
) -> Result<(Vec<AccountDetailsDto>, Option<String>)> {
    let mut new_accounts: Vec<AccountDetailsDto> = Vec::new();
    let mut secret_manager_dto: Option<String> = None;

    let account_indexation_key = if stronghold {
        key_to_chrysalis_key(b"iota-wallet-account-indexation")
    } else {
        b"iota-wallet-account-indexation".to_vec()
    };
    if let Some(account_indexation) = chrysalis_data.get(&account_indexation_key) {
        if let Some(account_keys) = serde_json::from_str::<serde_json::Value>(account_indexation)?.as_array() {
            for account_key in account_keys {
                let account_key = if stronghold {
                    key_to_chrysalis_key(account_key["key"].as_str().expect("key must be a string").as_bytes())
                } else {
                    account_key["key"]
                        .as_str()
                        .expect("key must be a string")
                        .as_bytes()
                        .to_vec()
                };

                if let Some(account_data) = chrysalis_data.get(&account_key) {
                    let account_data = serde_json::from_str::<serde_json::Value>(account_data)?;
                    if secret_manager_dto.is_none() {
                        let dto = match &account_data["signerType"]["type"].as_str() {
                            Some("Stronghold") => format!(
                                r#"{{"Stronghold": {{"password": null, "timeout": null, "snapshotPath": "{}/wallet.stronghold"}} }}"#,
                                storage_path.to_string_lossy()
                            ),
                            Some("LedgerNano") => r#"{{"LedgerNano": false }}"#.into(),
                            Some("LedgerNanoSimulator") => r#"{{"LedgerNano": true }}"#.into(),
                            _ => return Err(Error::Migration("Missing signerType".into())),
                        };
                        secret_manager_dto = Some(dto);
                    }

                    let mut account_addresses = Vec::new();

                    // Migrate addresses, skips all above potential gaps (for example: index 0, 1, 3 -> 0, 1), public
                    // and internal addresses on their own
                    if let Some(addresses) = account_data["addresses"].as_array() {
                        let mut highest_public_address_index = 0;
                        let mut highest_internal_address_index = 0;
                        for address in addresses {
                            let internal = address["internal"].as_bool().unwrap();
                            let key_index = address["keyIndex"].as_u64().unwrap() as u32;
                            let bech32_address = Bech32Address::from_str(address["address"].as_str().unwrap())?;
                            if internal {
                                if key_index != highest_internal_address_index {
                                    log::warn!(
                                        "Skip migrating internal address because of gap: {bech32_address}, index {key_index}"
                                    );
                                    continue;
                                }
                                highest_internal_address_index += 1;
                            } else {
                                if key_index != highest_public_address_index {
                                    log::warn!(
                                        "Skip migrating public address because of gap: {bech32_address}, index {key_index}"
                                    );
                                    continue;
                                }
                                highest_public_address_index += 1;
                            }
                            account_addresses.push(AccountAddress {
                                address: bech32_address,
                                key_index,
                                internal,
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
    // Accounts must be ordered by index
    new_accounts.sort_unstable_by_key(|a| a.index);
    Ok((new_accounts, secret_manager_dto))
}

fn get_chrysalis_data(chrysalis_storage_path: &Path, password: Option<Password>) -> Result<HashMap<Vec<u8>, String>> {
    let encryption_key = password.map(storage_password_to_encryption_key);
    let chrysalis_db = DB::open_default(chrysalis_storage_path).unwrap();
    // iterate over all rocksdb keys
    let mut chrysalis_data = HashMap::new();
    for item in chrysalis_db.iterator(IteratorMode::Start) {
        let (key, value) = item.unwrap();

        let key_utf8 = String::from_utf8(key.to_vec()).map_err(|_| Error::Migration("invalid utf8".into()))?;
        let value = if let Some(encryption_key) = &encryption_key {
            let value_utf8 = String::from_utf8(value.to_vec()).map_err(|_| Error::Migration("invalid utf8".into()))?;
            // "iota-wallet-key-checksum_value" is never an encrypted value
            if serde_json::from_str::<Vec<u8>>(&value_utf8).is_ok() && key_utf8 != "iota-wallet-key-checksum_value" {
                decrypt_record(&value_utf8, encryption_key)?
            } else {
                value_utf8
            }
        } else {
            String::from_utf8(value.to_vec()).map_err(|_| Error::Migration("invalid utf8".into()))?
        };

        chrysalis_data.insert(key.to_vec(), value);
    }
    Ok(chrysalis_data)
}

fn storage_password_to_encryption_key(password: Password) -> Zeroizing<[u8; 32]> {
    let mut dk = [0; 64];
    // safe to unwrap (rounds > 0)
    crypto::keys::pbkdf::PBKDF2_HMAC_SHA512(
        password.as_bytes(),
        b"wallet.rs::storage",
        core::num::NonZeroU32::new(100).unwrap(),
        &mut dk,
    );
    let key: [u8; 32] = dk[0..32][..].try_into().unwrap();
    Zeroizing::new(key)
}

fn decrypt_record(record: &str, encryption_key: &[u8; 32]) -> crate::wallet::Result<String> {
    let record: Vec<u8> = serde_json::from_str(record)?;
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

pub(crate) fn key_to_chrysalis_key(key: &[u8]) -> Vec<u8> {
    let mut buf = [0; 64];
    HMAC_SHA512(key, key, &mut buf);

    let (id, _) = buf.split_at(24);

    id.try_into().unwrap()
}
