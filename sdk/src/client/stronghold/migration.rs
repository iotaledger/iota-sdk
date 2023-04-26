// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{num::NonZeroU32, ops::Deref, path::Path};

use crypto::ciphers::chacha::{self};
use iota_stronghold::{Client, SnapshotPath, Stronghold};
use zeroize::Zeroize;

use super::{common::PRIVATE_DATA_CLIENT_PATH, Error, StrongholdAdapter};
use crate::client::stronghold::{check_or_create_snapshot, storage::insert as v3_insert};

impl StrongholdAdapter {
    /// Migrates a snapshot from version 2 to version 3.
    pub fn migrate_v2_to_v3<P: AsRef<Path>>(
        current_path: P,
        current_password: &str,
        new_path: Option<P>,
        new_password: Option<&str>,
    ) -> Result<(), Error> {
        log::debug!("migrate_v2_to_v3");
        use engine::snapshot::migration::{migrate, Version};

        const PBKDF_SALT: &[u8] = b"wallet.rs";
        // Safe as it's definitely not 0.
        const PBKDF_ITER: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100) };
        const TMP_FILE: &str = "migration_v2_to_v3.stronghold";

        let mut buffer = [0u8; 32];
        // TODO handle unwrap
        let tmp_path = current_path.as_ref().parent().unwrap().join(TMP_FILE);

        // Safe to unwrap because rounds > 0.
        crypto::keys::pbkdf::PBKDF2_HMAC_SHA512(current_password.as_bytes(), PBKDF_SALT, PBKDF_ITER, buffer.as_mut());

        let current_version = Version::V2 {
            path: current_path.as_ref(),
            key: &buffer,
            aad: &[],
        };

        let new_password = new_password.unwrap_or(current_password);
        let new_version = Version::V3 {
            path: &tmp_path,
            password: new_password.as_bytes(),
        };

        migrate(current_version, new_version)?;

        let new_path = new_path.unwrap_or(current_path);
        // TODO handle unwrap
        std::fs::rename(tmp_path, new_path.as_ref()).unwrap();

        Self::reencrypt_data_v2_to_v3(new_path, &buffer, new_password)?;
        buffer.zeroize();

        Ok(())
    }

    /// Re-encrypt data in the Stronghold store.
    fn reencrypt_data_v2_to_v3<P: AsRef<Path>>(
        path: P,
        v2_password_hash: &[u8; 32],
        new_password: &str,
    ) -> Result<(), Error> {
        log::debug!("reencrypt_data_v2_to_v3");

        let stronghold = Stronghold::default();
        let key_provider = super::common::key_provider_from_password(new_password);
        check_or_create_snapshot(&stronghold, &key_provider, &SnapshotPath::from_path(&path))?;

        // If there are keys to re-encrypt, we iterate over the requested keys and attempt to re-encrypt the
        // corresponding values.
        let stronghold_client = stronghold.get_client(PRIVATE_DATA_CLIENT_PATH)?;
        let keys_to_re_encrypt = stronghold_client.store().keys()?;

        for key in keys_to_re_encrypt {
            if let Some(value) = v2_get(&stronghold_client, &key, v2_password_hash)? {
                // insert with new encryption key
                v3_insert(&stronghold_client, &key, &value)?;
            } else {
                log::error!("didn't get data from stronghold store");
            }
        }

        // Rewrite the snapshot with the new encrypted data.
        stronghold.commit_with_keyprovider(&SnapshotPath::from_path(path), &key_provider)?;

        Ok(())
    }
}

fn v2_get(stronghold_client: &Client, k: &[u8], encryption_key: &[u8; 32]) -> Result<Option<Vec<u8>>, Error> {
    let data = match stronghold_client.store().get(k)? {
        Some(data) => data,
        None => return Ok(None),
    };
    Ok(Some(chacha::aead_decrypt(encryption_key.deref(), &data)?))
}
