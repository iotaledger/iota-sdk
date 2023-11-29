// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::storage::StorageAdapter,
    wallet::{core::builder::dto::WalletBuilderDto, storage::constants::WALLET_BUILDER_KEY, WalletBuilder},
};

impl WalletBuilder {
    pub async fn save(&self, storage: &impl StorageAdapter<Error = crate::wallet::Error>) -> crate::wallet::Result<()> {
        log::debug!("[save] wallet builder");
        storage.set(WALLET_BUILDER_KEY, self).await?;
        Ok(())
    }

    pub async fn load(
        storage: &impl StorageAdapter<Error = crate::wallet::Error>,
    ) -> crate::wallet::Result<Option<Self>> {
        log::debug!("[load] wallet builder");
        if let Some(wallet_builder_dto) = storage.get::<WalletBuilderDto>(WALLET_BUILDER_KEY).await? {
            log::debug!("[load] wallet builder dto: {wallet_builder_dto:?}");

            Ok(Some(wallet_builder_dto.into()))
        } else {
            Ok(None)
        }
    }
}
