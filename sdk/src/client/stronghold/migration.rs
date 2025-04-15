// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    ffi::OsStr,
    num::NonZeroU32,
    path::{Path, PathBuf},
};

use crypto::ciphers::{
    chacha::{self, XChaCha20Poly1305},
    traits::Aead,
};
use iota_stronghold::{
    procedures::{self, AeadCipher},
    Client, Location, SnapshotPath, Stronghold,
};
use zeroize::{Zeroize, Zeroizing};

use super::{
    common::{PRIVATE_DATA_CLIENT_PATH, SECRET_VAULT_PATH, USERDATA_STORE_KEY_RECORD_PATH},
    Error, StrongholdAdapter,
};
use crate::client::{
    stronghold::{check_or_create_snapshot, Error as StrongholdError},
    utils::Password,
};

impl StrongholdAdapter {
    /// Migrates a snapshot from version 2 to version 3.
    pub fn migrate_snapshot_v2_to_v3<P: AsRef<Path>>(
        current_path: P,
        current_password: Password,
        salt: impl AsRef<str>,
        rounds: u32,
        new_path: Option<P>,
        new_password: Option<Password>,
    ) -> Result<(), Error> {
        log::debug!("migrate_snapshot_v2_to_v3");
        use iota_stronghold::engine::snapshot::migration::{migrate, Version};

        let mut buffer = [0u8; 32];
        let mut tmp_path = current_path.as_ref().as_os_str().to_os_string();
        tmp_path.push(OsStr::new("-tmp"));
        let tmp_path = PathBuf::from(tmp_path);

        if tmp_path.exists() {
            return Err(Error::PathAlreadyExists(tmp_path));
        }

        crypto::keys::pbkdf::PBKDF2_HMAC_SHA512(
            current_password.as_bytes(),
            salt.as_ref().as_bytes(),
            NonZeroU32::try_from(rounds).map_err(|_| StrongholdError::InvalidRounds(rounds))?,
            buffer.as_mut(),
        );

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
        std::fs::rename(tmp_path, new_path.as_ref())?;

        Self::reencrypt_data_v2_to_v3(new_path, &buffer, new_password)?;
        buffer.zeroize();

        Ok(())
    }

    /// Re-encrypt data in the Stronghold store.
    fn reencrypt_data_v2_to_v3<P: AsRef<Path>>(
        path: P,
        v2_password_hash: &[u8; 32],
        new_password: Password,
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
    Ok(Some(chacha::aead_decrypt(encryption_key, &data)?))
}

fn v3_insert(stronghold_client: &Client, k: &[u8], v: &[u8]) -> Result<Option<Vec<u8>>, Error> {
    let store_key_location = Location::generic(SECRET_VAULT_PATH, USERDATA_STORE_KEY_RECORD_PATH);

    // Generate and store encryption key if not existent yet.
    if !stronghold_client.record_exists(&store_key_location)? {
        let mut key = Zeroizing::new(vec![0_u8; 32]);
        crypto::utils::rand::fill(key.as_mut())?;
        let vault_path = store_key_location.vault_path();
        let vault = stronghold_client.vault(vault_path);
        vault.write_secret(store_key_location.clone(), key)?;
    }

    let mut nonce = [0; XChaCha20Poly1305::NONCE_LENGTH];
    crypto::utils::rand::fill(&mut nonce)?;

    let encrypted_value = stronghold_client.execute_procedure(procedures::AeadEncrypt {
        cipher: AeadCipher::XChaCha20Poly1305,
        associated_data: Vec::new(),
        nonce: nonce.to_vec(),
        plaintext: v.to_vec(),
        key: store_key_location,
    })?;

    // The value is assumed to be `nonce || tag || ciphertext`
    let final_data = [nonce.to_vec(), encrypted_value].concat();

    Ok(stronghold_client.store().insert(k.to_vec(), final_data, None)?)
}
