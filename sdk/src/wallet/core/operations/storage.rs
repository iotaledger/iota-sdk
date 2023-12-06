// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{secret::SecretManagerConfig, storage::StorageAdapter},
    wallet::{
        core::builder::dto::WalletBuilderDto,
        storage::constants::{SECRET_MANAGER_KEY, WALLET_BUILDER_KEY},
        WalletBuilder,
    },
};

impl<S: 'static + SecretManagerConfig> WalletBuilder<S> {
    pub(crate) async fn save(
        &self,
        storage: &impl StorageAdapter<Error = crate::wallet::Error>,
    ) -> crate::wallet::Result<()> {
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

    pub(crate) async fn load(
        storage: &impl StorageAdapter<Error = crate::wallet::Error>,
    ) -> crate::wallet::Result<Option<Self>> {
        log::debug!("[load] wallet builder");
        if let Some(wallet_builder_dto) = storage
            .get::<WalletBuilderDto<S::GenerationOptions, S::SigningOptions>>(WALLET_BUILDER_KEY)
            .await?
        {
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
