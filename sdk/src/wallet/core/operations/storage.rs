// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "storage")]
mod storage_stub {
    use alloc::sync::Arc;

    use async_trait::async_trait;
    use serde::{Deserialize, Serialize};
    use tokio::sync::RwLock;

    use crate::{
        client::{
            secret::{mnemonic::MnemonicSecretManager, SecretManage, SecretManagerConfig},
            storage::StorageAdapter,
            ClientBuilder,
        },
        wallet::{
            storage::{
                constants::{SECRET_MANAGER_KEY, WALLET_INDEXATION_KEY},
                StorageOptions,
            },
            WalletBuilder,
        },
    };

    #[derive(Debug, Serialize, Deserialize)]
    struct WalletData {
        client_options: Option<ClientBuilder>,
        coin_type: Option<u32>,
        storage_options: Option<StorageOptions>,
    }

    impl WalletData {
        fn into_builder<S: SecretManage>(self, secret_manager: Option<S>) -> WalletBuilder<S> {
            WalletBuilder {
                client_options: self.client_options,
                coin_type: self.coin_type,
                storage_options: self.storage_options,
                secret_manager: secret_manager.map(|s| Arc::new(RwLock::new(s))),
            }
        }
    }

    #[async_trait]
    pub trait SaveLoadWallet {
        async fn save(&self, storage: &impl StorageAdapter<Error = crate::wallet::Error>) -> crate::wallet::Result<()>;

        async fn load(
            storage: &impl StorageAdapter<Error = crate::wallet::Error>,
        ) -> crate::wallet::Result<Option<Self>>
        where
            Self: Sized;
    }

    #[async_trait]
    impl<S: SecretManagerConfig> SaveLoadWallet for WalletBuilder<S>
    where
        crate::wallet::Error: From<S::Error>,
    {
        async fn save(&self, storage: &impl StorageAdapter<Error = crate::wallet::Error>) -> crate::wallet::Result<()> {
            log::debug!("save_wallet_data");
            storage.set(WALLET_INDEXATION_KEY, self).await?;

            if let Some(secret_manager) = &self.secret_manager {
                let secret_manager = secret_manager.read().await;
                if let Some(config) = secret_manager.to_config() {
                    log::debug!("save_secret_manager: {config:?}");
                    storage.set(SECRET_MANAGER_KEY, &config).await?;
                }
            }
            Ok(())
        }

        async fn load(
            storage: &impl StorageAdapter<Error = crate::wallet::Error>,
        ) -> crate::wallet::Result<Option<Self>> {
            log::debug!("get_wallet_data");
            if let Some(data) = storage.get::<WalletData>(WALLET_INDEXATION_KEY).await? {
                log::debug!("get_wallet_data {data:?}");

                let secret_manager_dto = storage.get(SECRET_MANAGER_KEY).await?;
                log::debug!("get_secret_manager {secret_manager_dto:?}");

                Ok(Some(data.into_builder(
                    secret_manager_dto.map(|dto| S::from_config(&dto)).transpose()?,
                )))
            } else {
                Ok(None)
            }
        }
    }

    #[async_trait]
    impl SaveLoadWallet for WalletBuilder<MnemonicSecretManager> {
        async fn save(&self, storage: &impl StorageAdapter<Error = crate::wallet::Error>) -> crate::wallet::Result<()> {
            log::debug!("save_wallet_data");
            storage.set(WALLET_INDEXATION_KEY, self).await?;
            Ok(())
        }

        async fn load(
            storage: &impl StorageAdapter<Error = crate::wallet::Error>,
        ) -> crate::wallet::Result<Option<Self>> {
            log::debug!("get_wallet_data");
            let res = storage.get::<WalletData>(WALLET_INDEXATION_KEY).await?;
            log::debug!("get_wallet_data {res:?}");
            Ok(res.map(|data| data.into_builder(None)))
        }
    }
}
#[cfg(not(feature = "storage"))]
mod storage_stub {
    pub trait SaveLoadWallet {}
    impl<T> SaveLoadWallet for T {}
}
pub use storage_stub::*;
