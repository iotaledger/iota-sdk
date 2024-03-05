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
            storage::constants::{SECRET_MANAGER_KEY, WALLET_BUILDER_KEY},
            WalletBuilder, WalletError,
        },
    };

    #[async_trait]
    pub trait SaveLoadWallet {
        async fn save(&self, storage: &impl StorageAdapter<Error = WalletError>) -> Result<(), WalletError>;

        async fn load(storage: &impl StorageAdapter<Error = WalletError>) -> Result<Option<Self>, WalletError>
        where
            Self: Sized;
    }

    #[async_trait]
    impl<S: 'static + SecretManagerConfig> SaveLoadWallet for WalletBuilder<S>
    where
        WalletError: From<S::Error>,
    {
        async fn save(&self, storage: &impl StorageAdapter<Error = WalletError>) -> Result<(), WalletError> {
            log::debug!("[save] wallet builder");
            storage.set(WALLET_BUILDER_KEY, self).await?;

            if let Some(secret_manager) = &self.secret_manager {
                let secret_manager = secret_manager.read().await;
                if let Some(config) = secret_manager.to_config() {
                    log::debug!("[save] secret manager: {config:?}");
                    storage.set(SECRET_MANAGER_KEY, &config).await?;
                }
            }
            Ok(())
        }

        async fn load(storage: &impl StorageAdapter<Error = WalletError>) -> Result<Option<Self>, WalletError> {
            log::debug!("[load] wallet builder");
            if let Some(wallet_builder_dto) = storage.get::<WalletBuilderDto>(WALLET_BUILDER_KEY).await? {
                log::debug!("[load] wallet builder dto: {wallet_builder_dto:?}");

                let secret_manager_dto = storage.get(SECRET_MANAGER_KEY).await?;
                log::debug!("[load] secret manager dto: {secret_manager_dto:?}");

                Ok(Some(Self::from(wallet_builder_dto).with_secret_manager(
                    secret_manager_dto.map(|dto| S::from_config(&dto)).transpose()?,
                )))
            } else {
                Ok(None)
            }
        }
    }

    #[async_trait]
    impl SaveLoadWallet for WalletBuilder<MnemonicSecretManager> {
        async fn save(&self, storage: &impl StorageAdapter<Error = WalletError>) -> Result<(), WalletError> {
            log::debug!("[save] wallet builder");
            storage.set(WALLET_BUILDER_KEY, self).await?;
            Ok(())
        }

        async fn load(storage: &impl StorageAdapter<Error = WalletError>) -> Result<Option<Self>, WalletError> {
            log::debug!("[load] wallet builder");
            let res = storage.get::<WalletBuilderDto>(WALLET_BUILDER_KEY).await?;
            log::debug!("[load] wallet builder: {res:?}");
            Ok(res.map(Into::into))
        }
    }
}
#[cfg(not(feature = "storage"))]
mod storage_stub {
    pub trait SaveLoadWallet {}
    impl<T> SaveLoadWallet for T {}
}
pub(crate) use storage_stub::*;
