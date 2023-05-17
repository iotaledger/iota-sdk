// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "storage")]
mod storage_stub {
    use alloc::sync::Arc;

    use async_trait::async_trait;
    use tokio::sync::RwLock;

    use crate::{
        client::secret::{mnemonic::MnemonicSecretManager, SecretManagerConfig},
        wallet::{
            storage::{
                constants::{SECRET_MANAGER_KEY, WALLET_INDEXATION_KEY},
                manager::StorageManager,
            },
            WalletBuilder,
        },
    };
    #[async_trait]
    pub trait SaveLoadWallet {
        async fn save_data(&self, storage: &StorageManager) -> crate::wallet::Result<()>;

        async fn get_data(storage: &StorageManager) -> crate::wallet::Result<Option<Self>>
        where
            Self: Sized;
    }

    #[async_trait]
    impl<S: SecretManagerConfig> SaveLoadWallet for WalletBuilder<S>
    where
        crate::wallet::Error: From<S::Error>,
    {
        async fn save_data(&self, storage: &StorageManager) -> crate::wallet::Result<()> {
            log::debug!("save_wallet_data");
            storage.storage.set(WALLET_INDEXATION_KEY, self).await?;

            if let Some(secret_manager) = &self.secret_manager {
                let secret_manager = secret_manager.read().await;
                let config = secret_manager.to_config();
                storage.storage.set(SECRET_MANAGER_KEY, config).await?;
            }
            Ok(())
        }

        async fn get_data(storage: &StorageManager) -> crate::wallet::Result<Option<Self>> {
            log::debug!("get_wallet_data");
            if let Some(mut builder) = storage.get::<Self>(WALLET_INDEXATION_KEY).await? {
                log::debug!("get_wallet_data {builder:?}");

                if let Some(secret_manager_dto) = storage.get(SECRET_MANAGER_KEY).await? {
                    log::debug!("get_secret_manager {secret_manager_dto:?}");

                    let secret_manager = S::from_config(&secret_manager_dto)?;
                    builder.secret_manager = Some(Arc::new(RwLock::new(secret_manager)));
                }
                Ok(Some(builder))
            } else {
                Ok(None)
            }
        }
    }

    #[async_trait]
    impl SaveLoadWallet for WalletBuilder<MnemonicSecretManager> {
        async fn save_data(&self, storage: &StorageManager) -> crate::wallet::Result<()> {
            log::debug!("save_wallet_data");
            storage.storage.set(WALLET_INDEXATION_KEY, self).await?;
            Ok(())
        }

        async fn get_data(storage: &StorageManager) -> crate::wallet::Result<Option<Self>> {
            log::debug!("get_wallet_data");
            let res = storage.get::<Self>(WALLET_INDEXATION_KEY).await?;
            log::debug!("get_wallet_data {res:?}");
            Ok(res)
        }
    }
}
#[cfg(not(feature = "storage"))]
mod storage_stub {
    pub trait SaveLoadWallet {}
    impl<T> SaveLoadWallet for T {}
}
pub use storage_stub::*;
