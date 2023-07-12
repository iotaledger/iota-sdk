// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod stronghold_snapshot;

use std::{fs, path::PathBuf, sync::atomic::Ordering};

use futures::{future::try_join_all, FutureExt};

use self::stronghold_snapshot::read_data_from_stronghold_snapshot;
#[cfg(feature = "storage")]
use crate::wallet::WalletBuilder;
use crate::{
    client::{
        secret::{stronghold::StrongholdSecretManager, SecretManager, SecretManagerConfig, SecretManagerDto},
        utils::Password,
    },
    types::block::address::Hrp,
    wallet::{Account, Wallet},
};

impl Wallet {
    /// Backup the wallet data in a Stronghold file
    /// stronghold_password must be the current one when Stronghold is used as SecretManager.
    pub async fn backup(
        &self,
        backup_path: PathBuf,
        stronghold_password: impl Into<Password> + Send,
    ) -> crate::wallet::Result<()> {
        let stronghold_password = stronghold_password.into();

        log::debug!("[backup] creating a stronghold backup");
        let secret_manager = self.secret_manager.read().await;

        match &*secret_manager {
            // Backup with existing stronghold
            SecretManager::Stronghold(stronghold) => {
                stronghold.set_password(stronghold_password).await?;
                self.store_data_to_stronghold(stronghold).await?;
                // Write snapshot to backup path
                stronghold.write_stronghold_snapshot(Some(&backup_path)).await?;
            }
            // Backup with new stronghold
            _ => {
                // If the SecretManager is not Stronghold we'll create a new one for the backup
                let backup_stronghold = StrongholdSecretManager::builder()
                    .password(stronghold_password)
                    .build(backup_path)?;

                self.store_data_to_stronghold(&backup_stronghold).await?;

                // Write snapshot to backup path
                backup_stronghold.write_stronghold_snapshot(None).await?;
            }
        }

        Ok(())
    }

    /// Restore a backup from a Stronghold file
    /// Replaces client_options, coin_type, secret_manager and accounts. Returns an error if accounts were already
    /// created If Stronghold is used as secret_manager, the existing Stronghold file will be overwritten. If a
    /// mnemonic was stored, it will be gone.
    /// if ignore_if_coin_type_mismatch.is_some(), client options will not be restored
    /// if ignore_if_coin_type_mismatch == Some(true), client options coin type and accounts will not be restored if the
    /// coin type doesn't match
    /// if ignore_if_bech32_hrp_mismatch == Some("rms"), but addresses have something different like "smr", no accounts
    /// will be restored.
    pub async fn restore_backup(
        &self,
        backup_path: PathBuf,
        stronghold_password: impl Into<Password> + Send,
        ignore_if_coin_type_mismatch: Option<bool>,
        ignore_if_bech32_hrp_mismatch: Option<Hrp>,
    ) -> crate::wallet::Result<()> {
        let stronghold_password = stronghold_password.into();

        log::debug!("[restore_backup] loading stronghold backup");

        if !backup_path.is_file() {
            return Err(crate::wallet::Error::Backup("backup path doesn't exist"));
        }

        let mut accounts = self.accounts.write().await;
        // We don't want to overwrite possible existing accounts
        if !accounts.is_empty() {
            return Err(crate::wallet::Error::Backup(
                "can't restore backup when there are already accounts",
            ));
        }

        let mut secret_manager = self.secret_manager.as_ref().write().await;
        // Get the current snapshot path if set
        let new_snapshot_path = if let SecretManager::Stronghold(stronghold) = &mut *secret_manager {
            stronghold.snapshot_path.clone()
        } else {
            PathBuf::from("wallet.stronghold")
        };

        // We'll create a new stronghold to load the backup
        let new_stronghold = StrongholdSecretManager::builder()
            .password(stronghold_password.clone())
            .build(backup_path.clone())?;

        let (read_client_options, read_coin_type, read_secret_manager, read_accounts) =
            read_data_from_stronghold_snapshot::<SecretManager>(&new_stronghold).await?;

        // If the coin type is not matching the current one, then the addresses in the accounts will also not be
        // correct, so we will not restore them
        let ignore_backup_values = ignore_if_coin_type_mismatch.map_or(false, |ignore| {
            if ignore {
                read_coin_type.map_or(true, |read_coin_type| {
                    self.coin_type.load(Ordering::Relaxed) != read_coin_type
                })
            } else {
                false
            }
        });

        if !ignore_backup_values {
            if let Some(read_coin_type) = read_coin_type {
                self.coin_type.store(read_coin_type, Ordering::Relaxed);
            }
        }

        if let Some(mut read_secret_manager) = read_secret_manager {
            // We have to replace the snapshot path with the current one, when building stronghold
            if let SecretManagerDto::Stronghold(stronghold_dto) = &mut read_secret_manager {
                stronghold_dto.snapshot_path = new_snapshot_path.to_string_lossy().into_owned();
            }

            let restored_secret_manager = SecretManager::from_config(&read_secret_manager)
                .map_err(|_| crate::wallet::Error::Backup("invalid secret_manager"))?;

            // Copy Stronghold file so the seed is available in the new location
            fs::copy(backup_path, new_snapshot_path)?;

            if let SecretManager::Stronghold(stronghold) = &restored_secret_manager {
                // Set password to restored secret manager
                stronghold.set_password(stronghold_password).await?;
            }
            *secret_manager = restored_secret_manager;
        }

        // drop secret manager, otherwise we get a deadlock in set_client_options() (there inside of save_wallet_data())
        drop(secret_manager);

        if ignore_if_coin_type_mismatch.is_none() {
            if let Some(read_client_options) = read_client_options {
                self.set_client_options(read_client_options).await?;
            }
        }

        if !ignore_backup_values {
            if let Some(read_accounts) = read_accounts {
                let restore_accounts = ignore_if_bech32_hrp_mismatch.map_or(true, |expected_bech32_hrp| {
                    // Only restore if bech32 hrps match
                    read_accounts.first().map_or(true, |account| {
                        account
                            .public_addresses
                            .first()
                            .expect("account needs to have a public address")
                            .address()
                            .hrp()
                            == &expected_bech32_hrp
                    })
                });

                if restore_accounts {
                    let restored_account = try_join_all(
                        read_accounts
                            .into_iter()
                            .map(|a| Account::new(a, self.inner.clone()).boxed()),
                    )
                    .await?;
                    *accounts = restored_account;
                }
            }
        }

        // store new data
        #[cfg(feature = "storage")]
        {
            use crate::wallet::core::operations::storage::SaveLoadWallet;
            let wallet_builder = WalletBuilder::new()
                .with_secret_manager_arc(self.secret_manager.clone())
                .with_storage_path(
                    &self
                        .storage_options
                        .path
                        .clone()
                        .into_os_string()
                        .into_string()
                        .expect("can't convert os string"),
                )
                .with_client_options(self.client_options().await)
                .with_coin_type(self.coin_type.load(Ordering::Relaxed));
            wallet_builder.save(&*self.storage_manager.read().await).await?;
            // also save account to db
            for account in accounts.iter() {
                account.save(None).await?;
            }
        }

        Ok(())
    }
}

impl Wallet<StrongholdSecretManager> {
    /// Backup the wallet data in a Stronghold file
    /// stronghold_password must be the current one when Stronghold is used as SecretManager.
    pub async fn backup(
        &self,
        backup_path: PathBuf,
        stronghold_password: impl Into<Password> + Send,
    ) -> crate::wallet::Result<()> {
        log::debug!("[backup] creating a stronghold backup");
        let secret_manager = self.secret_manager.read().await;

        secret_manager.set_password(stronghold_password).await?;

        self.store_data_to_stronghold(&secret_manager).await?;

        // Write snapshot to backup path
        secret_manager.write_stronghold_snapshot(Some(&backup_path)).await?;

        Ok(())
    }

    /// Restore a backup from a Stronghold file
    /// Replaces client_options, coin_type, secret_manager and accounts. Returns an error if accounts were already
    /// created If Stronghold is used as secret_manager, the existing Stronghold file will be overwritten. If a
    /// mnemonic was stored, it will be gone.
    /// if ignore_if_coin_type_mismatch.is_some(), client options will not be restored
    /// if ignore_if_coin_type_mismatch == Some(true), client options coin type and accounts will not be restored if the
    /// coin type doesn't match
    /// if ignore_if_bech32_hrp_mismatch == Some("rms"), but addresses have something different like "smr", no accounts
    /// will be restored.
    pub async fn restore_backup(
        &self,
        backup_path: PathBuf,
        stronghold_password: impl Into<Password> + Send,
        ignore_if_coin_type_mismatch: Option<bool>,
        ignore_if_bech32_hrp_mismatch: Option<&str>,
    ) -> crate::wallet::Result<()> {
        let stronghold_password = stronghold_password.into();

        log::debug!("[restore_backup] loading stronghold backup");

        if !backup_path.is_file() {
            return Err(crate::wallet::Error::Backup("backup path doesn't exist"));
        }

        let mut accounts = self.accounts.write().await;
        // We don't want to overwrite possible existing accounts
        if !accounts.is_empty() {
            return Err(crate::wallet::Error::Backup(
                "can't restore backup when there are already accounts",
            ));
        }

        let mut secret_manager = self.secret_manager.as_ref().write().await;
        // Get the current snapshot path if set
        let new_snapshot_path = secret_manager.snapshot_path.clone();

        // We'll create a new stronghold to load the backup
        let new_stronghold = StrongholdSecretManager::builder()
            .password(stronghold_password.clone())
            .build(backup_path.clone())?;

        let (read_client_options, read_coin_type, read_secret_manager, read_accounts) =
            read_data_from_stronghold_snapshot::<StrongholdSecretManager>(&new_stronghold).await?;

        // If the coin type is not matching the current one, then the addresses in the accounts will also not be
        // correct, so we will not restore them
        let ignore_backup_values = ignore_if_coin_type_mismatch.map_or(false, |ignore| {
            if ignore {
                read_coin_type.map_or(true, |read_coin_type| {
                    self.coin_type.load(Ordering::Relaxed) != read_coin_type
                })
            } else {
                false
            }
        });

        if !ignore_backup_values {
            if let Some(read_coin_type) = read_coin_type {
                self.coin_type.store(read_coin_type, Ordering::Relaxed);
            }
        }

        if let Some(mut read_secret_manager) = read_secret_manager {
            read_secret_manager.snapshot_path = new_snapshot_path.to_string_lossy().into_owned();

            let restored_secret_manager = StrongholdSecretManager::from_config(&read_secret_manager)
                .map_err(|_| crate::wallet::Error::Backup("invalid secret_manager"))?;

            // Copy Stronghold file so the seed is available in the new location
            fs::copy(backup_path, new_snapshot_path)?;

            // Set password to restored secret manager
            restored_secret_manager.set_password(stronghold_password).await?;
            *secret_manager = restored_secret_manager;
        }

        // drop secret manager, otherwise we get a deadlock in set_client_options()
        drop(secret_manager);

        // Update Wallet with read data
        if ignore_if_coin_type_mismatch.is_none() {
            if let Some(read_client_options) = read_client_options {
                // If the nodes are from the same network as the current client options, then extend it
                self.set_client_options(read_client_options).await?;
            }
        }

        if !ignore_backup_values {
            if let Some(read_accounts) = read_accounts {
                let restore_accounts = ignore_if_bech32_hrp_mismatch.map_or(true, |expected_bech32_hrp| {
                    // Only restore if bech32 hrps match
                    read_accounts.first().map_or(true, |account| {
                        account
                            .public_addresses
                            .first()
                            .expect("account needs to have a public address")
                            .address()
                            .hrp()
                            == expected_bech32_hrp
                    })
                });

                if restore_accounts {
                    let restored_account = try_join_all(
                        read_accounts
                            .into_iter()
                            .map(|a| Account::new(a, self.inner.clone()).boxed()),
                    )
                    .await?;
                    *accounts = restored_account;
                }
            }
        }

        // store new data
        #[cfg(feature = "storage")]
        {
            use crate::wallet::core::operations::storage::SaveLoadWallet;
            let wallet_builder = WalletBuilder::new()
                .with_secret_manager_arc(self.secret_manager.clone())
                .with_storage_path(
                    &self
                        .storage_options
                        .path
                        .clone()
                        .into_os_string()
                        .into_string()
                        .expect("can't convert os string"),
                )
                .with_client_options(self.client_options().await)
                .with_coin_type(self.coin_type.load(Ordering::Relaxed));
            wallet_builder.save(&*self.storage_manager.read().await).await?;
            // also save account to db
            for account in accounts.iter() {
                account.save(None).await?;
            }
        }

        Ok(())
    }
}
