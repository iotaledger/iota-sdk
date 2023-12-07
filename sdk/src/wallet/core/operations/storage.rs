// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    client::storage::StorageAdapter,
    wallet::{core::builder::dto::WalletBuilderDto, storage::constants::WALLET_BUILDER_KEY, WalletBuilder},
};

impl<T: Serialize + Send + Sync> WalletBuilder<T> {
    pub(crate) async fn save(
        &self,
        storage: &impl StorageAdapter<Error = crate::wallet::Error>,
    ) -> crate::wallet::Result<()> {
        log::debug!("[save] wallet builder");
        storage.set(WALLET_BUILDER_KEY, self).await?;
        Ok(())
    }

    pub(crate) async fn load<T2: core::fmt::Debug + DeserializeOwned>(
        storage: &impl StorageAdapter<Error = crate::wallet::Error>,
    ) -> crate::wallet::Result<Option<Self>>
    where
        T: From<T2>,
    {
        log::debug!("[load] wallet builder");
        if let Some(wallet_builder_dto) = storage.get::<WalletBuilderDto<T2>>(WALLET_BUILDER_KEY).await? {
            log::debug!("[load] wallet builder dto: {wallet_builder_dto:?}");

            Ok(Some(Self::from(wallet_builder_dto)))
        } else {
            Ok(None)
        }
    }
}
