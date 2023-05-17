// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::atomic::Ordering;

use crate::{
    client::{
        secret::{types::StrongholdDto, SecretManagerConfig},
        storage::StorageProvider,
        stronghold::StrongholdAdapter,
    },
    wallet::{
        account::AccountDetails,
        migration::{latest_migration_version, migrate_backup, MIGRATION_VERSION_KEY},
        ClientOptions, Wallet,
    },
};

pub(crate) const CLIENT_OPTIONS_KEY: &str = "client_options";
pub(crate) const COIN_TYPE_KEY: &str = "coin_type";
pub(crate) const SECRET_MANAGER_KEY: &str = "secret_manager";
pub(crate) const ACCOUNTS_KEY: &str = "accounts";

impl<S: 'static + SecretManagerConfig> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    pub(crate) async fn store_data_to_stronghold(
        &self,
        stronghold: &StrongholdAdapter,
        secret_manager_dto: StrongholdDto,
    ) -> crate::wallet::Result<()>
    where
        crate::wallet::Error: From<S::Error>,
    {
        // Set migration version
        stronghold
            .insert(
                MIGRATION_VERSION_KEY.as_bytes(),
                serde_json::to_string(&latest_migration_version())?.as_bytes(),
            )
            .await?;

        let client_options = self.client_options().await.to_json()?;
        stronghold
            .insert(CLIENT_OPTIONS_KEY.as_bytes(), client_options.as_bytes())
            .await?;

        let coin_type = self.coin_type.load(Ordering::Relaxed);
        stronghold
            .insert(COIN_TYPE_KEY.as_bytes(), &coin_type.to_le_bytes())
            .await?;

        stronghold
            .insert(
                SECRET_MANAGER_KEY.as_bytes(),
                serde_json::to_string(&secret_manager_dto)?.as_bytes(),
            )
            .await?;

        let mut serialized_accounts = Vec::new();
        for account in self.accounts.read().await.iter() {
            serialized_accounts.push(serde_json::to_string(&*account.details().await)?);
        }

        stronghold
            .insert(
                ACCOUNTS_KEY.as_bytes(),
                serde_json::to_string(&serialized_accounts)?.as_bytes(),
            )
            .await?;

        Ok(())
    }
}

pub(crate) async fn read_data_from_stronghold_snapshot(
    stronghold: &StrongholdAdapter,
) -> crate::wallet::Result<(
    Option<ClientOptions>,
    Option<u32>,
    Option<StrongholdDto>,
    Option<Vec<AccountDetails>>,
)> {
    migrate_backup(stronghold).await?;

    // Get client_options
    let client_options_bytes = stronghold.get(CLIENT_OPTIONS_KEY.as_bytes()).await?;
    let client_options = if let Some(client_options_bytes) = client_options_bytes {
        let client_options_string = String::from_utf8(client_options_bytes)
            .map_err(|_| crate::wallet::Error::Backup("invalid client_options"))?;
        let client_options: ClientOptions = serde_json::from_str(&client_options_string)?;

        log::debug!("[restore_backup] restored client_options {client_options_string}");
        Some(client_options)
    } else {
        None
    };

    // Get coin_type
    let coin_type_bytes = stronghold.get(COIN_TYPE_KEY.as_bytes()).await?;
    let coin_type = if let Some(coin_type_bytes) = coin_type_bytes {
        let coin_type = u32::from_le_bytes(
            coin_type_bytes
                .try_into()
                .map_err(|_| crate::wallet::Error::Backup("invalid coin_type"))?,
        );
        log::debug!("[restore_backup] restored coin_type: {coin_type}");
        Some(coin_type)
    } else {
        None
    };

    // Get secret_manager
    let restored_secret_manager_bytes = stronghold.get(SECRET_MANAGER_KEY.as_bytes()).await?;
    let restored_secret_manager = if let Some(restored_secret_manager) = restored_secret_manager_bytes {
        let secret_manager_string = String::from_utf8(restored_secret_manager)
            .map_err(|_| crate::wallet::Error::Backup("invalid secret_manager"))?;

        log::debug!("[restore_backup] restored secret_manager: {}", secret_manager_string);

        let secret_manager_dto: StrongholdDto = serde_json::from_str(&secret_manager_string)?;

        Some(secret_manager_dto)
    } else {
        None
    };

    // Get accounts
    let restored_accounts_bytes = stronghold.get(ACCOUNTS_KEY.as_bytes()).await?;
    let restored_accounts = if let Some(restored_accounts) = restored_accounts_bytes {
        let restored_accounts_string =
            String::from_utf8(restored_accounts).map_err(|_| crate::wallet::Error::Backup("invalid accounts"))?;

        log::debug!("[restore_backup] restore accounts: {restored_accounts_string}");

        let restored_accounts_string: Vec<String> = serde_json::from_str(&restored_accounts_string)?;

        let restored_accounts = restored_accounts_string
            .into_iter()
            .map(|a| Ok(serde_json::from_str(&a)?))
            .collect::<crate::wallet::Result<Vec<AccountDetails>>>()?;

        Some(restored_accounts)
    } else {
        None
    };

    Ok((client_options, coin_type, restored_secret_manager, restored_accounts))
}
