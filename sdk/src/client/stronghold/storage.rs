// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The `StorageProvider` implementation for `StrongholdAdapter`.

use async_trait::async_trait;
use crypto::ciphers::{chacha::XChaCha20Poly1305, traits::Aead};
use iota_stronghold::{
    procedures::{self, AeadCipher},
    Client, Location,
};
use zeroize::Zeroizing;

use super::{
    common::{PRIVATE_DATA_CLIENT_PATH, SECRET_VAULT_PATH, USERDATA_STORE_KEY_RECORD_PATH},
    StrongholdAdapter,
};
use crate::client::{storage::StorageProvider, stronghold::Error};

#[async_trait]
impl StorageProvider for StrongholdAdapter {
    type Error = Error;

    #[allow(clippy::significant_drop_tightening)]
    async fn get(&mut self, k: &[u8]) -> Result<Option<Vec<u8>>, Self::Error> {
        let stronghold_client = self.stronghold.lock().await.get_client(PRIVATE_DATA_CLIENT_PATH)?;

        let mut data = match stronghold_client.store().get(k)? {
            Some(data) => data,
            None => return Ok(None),
        };

        let store_key_location = Location::generic(SECRET_VAULT_PATH, USERDATA_STORE_KEY_RECORD_PATH);

        let decrypted_value = stronghold_client.execute_procedure(procedures::AeadDecrypt {
            cipher: AeadCipher::XChaCha20Poly1305,
            associated_data: Vec::new(),
            nonce: data.drain(..XChaCha20Poly1305::NONCE_LENGTH).collect(),
            tag: data.drain(..XChaCha20Poly1305::TAG_LENGTH).collect(),
            ciphertext: data,
            key: store_key_location,
        })?;

        Ok(Some(decrypted_value))
    }

    async fn insert(&mut self, k: &[u8], v: &[u8]) -> Result<Option<Vec<u8>>, Self::Error> {
        let store_key_location = Location::generic(SECRET_VAULT_PATH, USERDATA_STORE_KEY_RECORD_PATH);

        let stronghold_client = self.stronghold.lock().await.get_client(PRIVATE_DATA_CLIENT_PATH)?;

        let previous_data = insert(&stronghold_client, k, v)?;

        let decrypted_previous_data = previous_data
            .map(|mut previous_data| {
                // The value is assumed to be `nonce || tag || ciphertext`
                stronghold_client.execute_procedure(procedures::AeadDecrypt {
                    cipher: AeadCipher::XChaCha20Poly1305,
                    associated_data: Vec::new(),
                    nonce: previous_data.drain(..XChaCha20Poly1305::NONCE_LENGTH).collect(),
                    tag: previous_data.drain(..XChaCha20Poly1305::TAG_LENGTH).collect(),
                    ciphertext: previous_data,
                    key: store_key_location,
                })
            })
            .transpose()?;

        Ok(decrypted_previous_data)
    }

    async fn delete(&mut self, k: &[u8]) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(self
            .stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .store()
            .delete(k)?)
    }
}

pub(crate) fn insert(stronghold_client: &Client, k: &[u8], v: &[u8]) -> Result<Option<Vec<u8>>, Error> {
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

mod tests {
    #[tokio::test]
    async fn test_stronghold_db() {
        use std::fs;

        use super::StrongholdAdapter;
        use crate::client::storage::StorageProvider;

        let snapshot_path = "test_stronghold_db.stronghold";

        fs::remove_file(snapshot_path).unwrap_or(());

        let mut stronghold = StrongholdAdapter::builder()
            .password("drowssap")
            .build(snapshot_path)
            .unwrap();

        assert!(matches!(stronghold.get(b"test-0").await, Ok(None)));
        assert!(matches!(stronghold.get(b"test-1").await, Ok(None)));
        assert!(matches!(stronghold.get(b"test-2").await, Ok(None)));

        assert!(matches!(stronghold.insert(b"test-0", b"test-0").await, Ok(None)));
        assert!(matches!(stronghold.insert(b"test-1", b"test-1").await, Ok(None)));
        assert!(matches!(stronghold.insert(b"test-2", b"test-2").await, Ok(None)));

        assert!(matches!(stronghold.get(b"test-0").await, Ok(Some(_))));
        assert!(matches!(stronghold.get(b"test-1").await, Ok(Some(_))));
        assert!(matches!(stronghold.get(b"test-2").await, Ok(Some(_))));

        assert!(matches!(stronghold.insert(b"test-0", b"0-tset").await, Ok(Some(_))));
        let previous_value = stronghold.insert(b"test-0", b"0-tset").await.unwrap();
        assert_eq!(Some(b"test-0".to_vec()), previous_value);
        assert!(matches!(stronghold.insert(b"test-1", b"1-tset").await, Ok(Some(_))));
        assert!(matches!(stronghold.insert(b"test-2", b"2-tset").await, Ok(Some(_))));

        assert!(matches!(stronghold.delete(b"test-0").await, Ok(Some(_))));
        assert!(matches!(stronghold.delete(b"test-1").await, Ok(Some(_))));
        assert!(matches!(stronghold.delete(b"test-2").await, Ok(Some(_))));

        assert!(matches!(stronghold.get(b"test-0").await, Ok(None)));
        assert!(matches!(stronghold.get(b"test-1").await, Ok(None)));
        assert!(matches!(stronghold.get(b"test-2").await, Ok(None)));

        fs::remove_file(snapshot_path).unwrap();
    }
}
