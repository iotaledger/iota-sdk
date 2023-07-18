// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "storage")]
mod storage_stub {

    use async_trait::async_trait;

    use crate::{
        client::{
            secret::{mnemonic::MnemonicSecretManager, SecretManagerConfig},
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

        async fn load(
            storage: &impl StorageAdapter<Error = crate::wallet::Error>,
        ) -> crate::wallet::Result<Option<Self>>
        where
            Self: Sized;
    }

    #[async_trait]
    impl<S: 'static + SecretManagerConfig> SaveLoadWallet for WalletBuilder<S>
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
            if let Some(data) = storage.get::<WalletBuilderDto>(WALLET_INDEXATION_KEY).await? {
                log::debug!("get_wallet_data {data:?}");

                let secret_manager_dto = storage.get(SECRET_MANAGER_KEY).await?;
                log::debug!("get_secret_manager {secret_manager_dto:?}");

                Ok(Some(Self::from(data).with_secret_manager(
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
            let res = storage.get::<WalletBuilderDto>(WALLET_INDEXATION_KEY).await?;
            log::debug!("get_wallet_data {res:?}");
            Ok(res.map(Into::into))
        }
    }
}
#[cfg(not(feature = "storage"))]
mod storage_stub {
    pub trait SaveLoadWallet {}
    impl<T> SaveLoadWallet for T {}
}
pub use storage_stub::*;
