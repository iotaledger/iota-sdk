// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The `StorageProvider` implementation for `StrongholdAdapter`.

use std::ops::Deref;

use async_trait::async_trait;
use crypto::ciphers::chacha;

use super::{common::PRIVATE_DATA_CLIENT_PATH, StrongholdAdapter};
use crate::client::{
    storage::{StorageAdapter, StorageAdapterId},
    stronghold::Error,
};

impl StorageAdapterId for StrongholdAdapter {
    const ID: &'static str = "Stronghold";
}

#[async_trait]
impl StorageAdapter for StrongholdAdapter {
    type Error = Error;

    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, Self::Error> {
        let data = match self
            .stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .store()
            .get(key.as_bytes())?
        {
            Some(data) => data,
            None => return Ok(None),
        };

        let locked_key_provider = self.key_provider.lock().await;
        let key_provider = if let Some(key_provider) = &*locked_key_provider {
            key_provider
        } else {
            return Err(Error::KeyCleared);
        };
        let buffer = key_provider.try_unlock()?;
        let buffer_ref = buffer.borrow();

        Ok(Some(chacha::aead_decrypt(buffer_ref.deref(), &data)?))
    }

    async fn set_bytes(&self, key: &str, record: &[u8]) -> Result<(), Self::Error> {
        let encrypted_value = {
            let locked_key_provider = self.key_provider.lock().await;
            let key_provider = if let Some(key_provider) = &*locked_key_provider {
                key_provider
            } else {
                return Err(Error::KeyCleared);
            };
            let buffer = key_provider.try_unlock()?;
            let buffer_ref = buffer.borrow();

            chacha::aead_encrypt(buffer_ref.deref(), record)?
        };

        self.stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .store()
            .insert(key.as_bytes().to_vec(), encrypted_value, None)?;
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
        let stronghold = StrongholdAdapter::builder()
            .password("drowssap")
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

        assert!(matches!(stronghold.set("test-0", "0-tset").await, Ok(())));
        assert!(matches!(stronghold.set("test-1", "1-tset").await, Ok(())));
        assert!(matches!(stronghold.set("test-2", "2-tset").await, Ok(())));

        assert!(matches!(stronghold.delete("test-0").await, Ok(())));
        assert!(matches!(stronghold.delete("test-1").await, Ok(())));
        assert!(matches!(stronghold.delete("test-2").await, Ok(())));

        assert!(matches!(stronghold.get::<String>("test-0").await, Ok(None)));
        assert!(matches!(stronghold.get::<String>("test-1").await, Ok(None)));
        assert!(matches!(stronghold.get::<String>("test-2").await, Ok(None)));

        fs::remove_file(snapshot_path).unwrap();
    }
}
