// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "storage")]
mod storage_stub {
    use async_trait::async_trait;

    use crate::{
        client::{
            secret::{DynSecretManagerConfig, SecretManagerConfig},
            storage::StorageAdapter,
        },
        wallet::{
            core::builder::dto::WalletBuilderDto,
            storage::constants::{SECRET_MANAGER_KEY, WALLET_INDEXATION_KEY},
            WalletBuilder,
        },
    };

    #[async_trait]
    pub trait SaveLoadWallet {
        async fn save(&self, storage: &impl StorageAdapter<Error = crate::wallet::Error>) -> crate::wallet::Result<()>;

        async fn load<S: 'static + DynSecretManagerConfig + SecretManagerConfig>(
            storage: &impl StorageAdapter<Error = crate::wallet::Error>,
        ) -> crate::wallet::Result<Option<Self>>
        where
            crate::client::Error: From<S::Error>,
            Self: Sized;
    }

    #[async_trait]
    impl SaveLoadWallet for WalletBuilder {
        async fn save(&self, storage: &impl StorageAdapter<Error = crate::wallet::Error>) -> crate::wallet::Result<()> {
            log::debug!("save_wallet_data");
            storage.set(WALLET_INDEXATION_KEY, self).await?;

            if let Some(secret_manager) = &self.secret_manager {
                let secret_manager = secret_manager.read().await;
                if let Some(config) = secret_manager.dyn_to_config() {
                    log::debug!("save_secret_manager: {config:?}");
                    storage.set(SECRET_MANAGER_KEY, &config).await?;
                }
            }
            Ok(())
        }

        async fn load<S: 'static + DynSecretManagerConfig + SecretManagerConfig>(
            storage: &impl StorageAdapter<Error = crate::wallet::Error>,
        ) -> crate::wallet::Result<Option<Self>>
        where
            crate::client::Error: From<S::Error>,
        {
            log::debug!("get_wallet_data");
            if let Some(data) = storage.get::<WalletBuilderDto>(WALLET_INDEXATION_KEY).await? {
                log::debug!("get_wallet_data {data:?}");

                let secret_manager_dto = storage.get(SECRET_MANAGER_KEY).await?;
                log::debug!("get_secret_manager {secret_manager_dto:?}");

                Ok(Some(
                    Self::from(data).with_secret_manager::<S>(
                        secret_manager_dto
                            .map(|dto| S::from_config(&dto))
                            .transpose()
                            .map_err(crate::client::Error::from)?,
                    ),
                ))
            } else {
                Ok(None)
            }
        }
    }
}
#[cfg(not(feature = "storage"))]
mod storage_stub {
    pub trait SaveLoadWallet {}
    impl<T> SaveLoadWallet for T {}
}
pub(crate) use storage_stub::*;
