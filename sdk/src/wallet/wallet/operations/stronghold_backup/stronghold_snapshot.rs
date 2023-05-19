// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::atomic::Ordering;

use crate::{
    client::{secret::SecretManagerDto, storage::StorageAdapter, stronghold::StrongholdAdapter},
    wallet::{
        account::AccountDetails,
        migration::{latest_backup_migration_version, migrate, MIGRATION_VERSION_KEY},
        ClientOptions, Wallet,
    },
};

pub(crate) const CLIENT_OPTIONS_KEY: &str = "client_options";
pub(crate) const COIN_TYPE_KEY: &str = "coin_type";
pub(crate) const SECRET_MANAGER_KEY: &str = "secret_manager";
pub(crate) const ACCOUNTS_KEY: &str = "accounts";

pub(crate) async fn store_data_to_stronghold(
    wallet: &Wallet,
    stronghold: &StrongholdAdapter,
    secret_manager_dto: SecretManagerDto,
) -> crate::wallet::Result<()> {
    // Set migration version
    stronghold
        .set(MIGRATION_VERSION_KEY, &latest_backup_migration_version())
        .await?;

    let client_options = wallet.client_options().await;
    stronghold.set(CLIENT_OPTIONS_KEY, &client_options).await?;

    let coin_type = wallet.coin_type.load(Ordering::Relaxed);
    stronghold.set_bytes(COIN_TYPE_KEY, &coin_type.to_le_bytes()).await?;

    // Only store secret_managers that aren't SecretManagerDto::Mnemonic, because there the Seed can't be serialized, so
    // we can't create the SecretManager again
    match secret_manager_dto {
        SecretManagerDto::Mnemonic(_) => {}
        _ => {
            stronghold.set(SECRET_MANAGER_KEY, &secret_manager_dto).await?;
        }
    }

    let mut serialized_accounts = Vec::new();
    for account in wallet.accounts.read().await.iter() {
        serialized_accounts.push(serde_json::to_value(&*account.details().await)?);
    }

    stronghold.set(ACCOUNTS_KEY, &serialized_accounts).await?;

    Ok(())
}

pub(crate) async fn read_data_from_stronghold_snapshot(
    stronghold: &StrongholdAdapter,
) -> crate::wallet::Result<(
    Option<ClientOptions>,
    Option<u32>,
    Option<SecretManagerDto>,
    Option<Vec<AccountDetails>>,
)> {
    migrate(stronghold).await?;

    // Get client_options
    let client_options = stronghold.get::<ClientOptions>(CLIENT_OPTIONS_KEY).await?;

    // Get coin_type
    let coin_type_bytes = stronghold.get_bytes(COIN_TYPE_KEY).await?;
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
    let restored_secret_manager = stronghold.get(SECRET_MANAGER_KEY).await?;

    // Get accounts
    let restored_accounts = stronghold.get::<Vec<AccountDetails>>(ACCOUNTS_KEY).await?;

    Ok((client_options, coin_type, restored_secret_manager, restored_accounts))
}
