// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{secret::SecretManagerConfig, storage::StorageAdapter, stronghold::StrongholdAdapter},
    types::TryFromDto,
    wallet::{
        core::{WalletData, WalletDataDto},
        migration::{latest_backup_migration_version, migrate, MIGRATION_VERSION_KEY},
        ClientOptions, Wallet,
    },
};

pub(crate) const CLIENT_OPTIONS_KEY: &str = "client_options";
pub(crate) const SECRET_MANAGER_KEY: &str = "secret_manager";
pub(crate) const WALLET_DATA_KEY: &str = "wallet_data";

impl<S: 'static + SecretManagerConfig> Wallet<S> {
    pub(crate) async fn store_data_to_stronghold(&self, stronghold: &StrongholdAdapter) -> crate::wallet::Result<()> {
        // Set migration version
        stronghold
            .set(MIGRATION_VERSION_KEY, &latest_backup_migration_version())
            .await?;

        let client_options = self.client_options().await;
        stronghold.set(CLIENT_OPTIONS_KEY, &client_options).await?;

        if let Some(secret_manager_dto) = self.secret_manager.read().await.to_config() {
            stronghold.set(SECRET_MANAGER_KEY, &secret_manager_dto).await?;
        }

        let serialized_wallet_data = serde_json::to_value(&WalletDataDto::from(&*self.data.read().await))?;
        stronghold.set(WALLET_DATA_KEY, &serialized_wallet_data).await?;

        Ok(())
    }
}

pub(crate) async fn read_wallet_data_from_stronghold_snapshot<S: 'static + SecretManagerConfig>(
    stronghold: &StrongholdAdapter,
) -> crate::wallet::Result<(Option<ClientOptions>, Option<S::Config>, Option<WalletData>)> {
    migrate(stronghold).await?;

    // Get client_options
    let client_options = stronghold.get(CLIENT_OPTIONS_KEY).await?;

    // TODO #1279: remove
    // // Get coin_type
    // let coin_type_bytes = stronghold.get_bytes(COIN_TYPE_KEY).await?;
    // let coin_type = if let Some(coin_type_bytes) = coin_type_bytes {
    //     let coin_type = u32::from_le_bytes(
    //         coin_type_bytes
    //             .try_into()
    //             .map_err(|_| WalletError::Backup("invalid coin_type"))?,
    //     );
    //     log::debug!("[restore_backup] restored coin_type: {coin_type}");
    //     Some(coin_type)
    // } else {
    //     None
    // };

    // Get secret_manager
    let restored_secret_manager = stronghold.get(SECRET_MANAGER_KEY).await?;

    // Get wallet data
    let restored_wallet_data = stronghold
        .get::<WalletDataDto>(WALLET_DATA_KEY)
        .await?
        .map(WalletData::try_from_dto)
        .transpose()?;

    Ok((client_options, restored_secret_manager, restored_wallet_data))
}
