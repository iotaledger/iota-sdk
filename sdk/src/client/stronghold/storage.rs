// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The `StorageAdapter` implementation for `StrongholdAdapter`.

use async_trait::async_trait;
use crypto::ciphers::{chacha::XChaCha20Poly1305, traits::Aead};
use iota_stronghold::{
    procedures::{self, AeadCipher},
    Location,
};
use zeroize::Zeroizing;

use super::{
    common::{PRIVATE_DATA_CLIENT_PATH, SECRET_VAULT_PATH, USERDATA_STORE_KEY_RECORD_PATH},
    StrongholdAdapter,
};
use crate::client::{storage::StorageAdapter, stronghold::Error};

#[async_trait]
impl StorageAdapter for StrongholdAdapter {
    type Error = Error;

    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, Self::Error> {
        let stronghold_client = self.stronghold.lock().await.get_client(PRIVATE_DATA_CLIENT_PATH)?;

        let mut data = match stronghold_client.store().get(key.as_bytes())? {
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

    async fn set_bytes(&self, key: &str, record: &[u8]) -> Result<(), Self::Error> {
        let stronghold_client = self.stronghold.lock().await.get_client(PRIVATE_DATA_CLIENT_PATH)?;
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
            plaintext: record.to_vec(),
            key: store_key_location,
        })?;

        // The value is assumed to be `nonce || tag || ciphertext`
        let final_data = [nonce.to_vec(), encrypted_value].concat();

        stronghold_client
            .store()
            .insert(key.as_bytes().to_vec(), final_data, None)?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), Self::Error> {
        self.stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .store()
            .delete(key.as_bytes())?;
        Ok(())
    }
}
mod tests {

    #[tokio::test]
    async fn test_stronghold_db() {
        use std::fs;

        use super::StrongholdAdapter;
        use crate::client::storage::StorageAdapter;

        let snapshot_path = "test_stronghold_db.stronghold";

        fs::remove_file(snapshot_path).unwrap_or(());

        let stronghold = StrongholdAdapter::builder()
            .password("drowssap".to_owned())
            .build(snapshot_path)
            .unwrap();

        assert!(matches!(stronghold.get::<String>("test-0").await, Ok(None)));
        assert!(matches!(stronghold.get::<String>("test-1").await, Ok(None)));
        assert!(matches!(stronghold.get::<String>("test-2").await, Ok(None)));

        assert!(matches!(stronghold.set("test-0", "test-0").await, Ok(())));
        assert!(matches!(stronghold.set("test-1", "test-1").await, Ok(())));
        assert!(matches!(stronghold.set("test-2", "test-2").await, Ok(())));

        assert!(matches!(stronghold.get::<String>("test-0").await, Ok(Some(s)) if s == "test-0"));
        assert!(matches!(stronghold.get::<String>("test-1").await, Ok(Some(s)) if s == "test-1"));
        assert!(matches!(stronghold.get::<String>("test-2").await, Ok(Some(s)) if s == "test-2"));

        assert!(matches!(stronghold.delete("test-0").await, Ok(())));
        assert!(matches!(stronghold.delete("test-1").await, Ok(())));
        assert!(matches!(stronghold.delete("test-2").await, Ok(())));

        assert!(matches!(stronghold.get::<String>("test-0").await, Ok(None)));
        assert!(matches!(stronghold.get::<String>("test-1").await, Ok(None)));
        assert!(matches!(stronghold.get::<String>("test-2").await, Ok(None)));

        fs::remove_file(snapshot_path).unwrap();
    }
}
