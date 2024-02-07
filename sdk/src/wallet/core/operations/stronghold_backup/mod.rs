// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod stronghold_snapshot;

use std::{fs, path::PathBuf};

use self::stronghold_snapshot::read_wallet_data_from_stronghold_snapshot;
#[cfg(feature = "storage")]
use crate::wallet::WalletBuilder;
use crate::{
    client::{
        secret::{stronghold::StrongholdSecretManager, SecretManager, SecretManagerConfig, SecretManagerDto},
        utils::Password,
    },
    types::block::address::Hrp,
    wallet::Wallet,
};

impl Wallet {
    /// Backup the wallet data in a Stronghold file.
    /// `stronghold_password` must be the current one when Stronghold is used as SecretManager.
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
    /// Replaces client_options, bip_path, secret_manager and wallet. Returns an error if the wallet was already
    /// created If Stronghold is used as secret_manager, the existing Stronghold file will be overwritten. If a
    /// mnemonic was stored, it will be gone.
    /// if ignore_if_bip_path_mismatch.is_some(), client options will not be restored
    /// if ignore_if_bip_path_mismatch == Some(true), client options coin type and wallet will not be restored if the
    /// coin type doesn't match
    /// If a bech32 hrp is provided to ignore_if_bech32_hrp_mismatch, that doesn't match the one of the current address,
    /// the wallet will not be restored.
    pub async fn restore_backup(
        &self,
        backup_path: PathBuf,
        stronghold_password: impl Into<Password> + Send,
        ignore_if_bip_path_mismatch: Option<bool>,
        ignore_if_bech32_hrp_mismatch: Option<Hrp>,
    ) -> crate::wallet::Result<()> {
        let stronghold_password = stronghold_password.into();

        log::debug!("[restore_backup] loading stronghold backup");

        if !backup_path.is_file() {
            return Err(crate::wallet::Error::Backup("backup path doesn't exist"));
        }

        let wallet_data = self.data().await;

        // We don't want to overwrite a possible existing wallet
        if !wallet_data.outputs.is_empty() {
            return Err(crate::wallet::Error::Backup(
                "can't restore backup when there is already a wallet",
            ));
        }

        let curr_bip_path = wallet_data.bip_path;

        // Explicitly drop the data to avoid contention
        drop(wallet_data);

        // We'll create a new stronghold to load the backup
        let new_stronghold = StrongholdSecretManager::builder()
            .password(stronghold_password.clone())
            .build(backup_path.clone())?;

        let (read_client_options, read_secret_manager, read_wallet_data) =
            read_wallet_data_from_stronghold_snapshot::<SecretManager>(&new_stronghold).await?;

        let read_bip_path = read_wallet_data.as_ref().and_then(|data| data.bip_path);

        // If the bip path is not matching the current one, we may ignore the backup
        let ignore_backup_values = ignore_if_bip_path_mismatch.map_or(false, |ignore| {
            if ignore {
                // TODO: #1279 okay that if both are none we always load the backup values?
                curr_bip_path != read_bip_path
            } else {
                false
            }
        });

        if !ignore_backup_values {
            self.data_mut().await.bip_path = read_bip_path;
        }

        // Get the current snapshot path if set
        let new_snapshot_path = if let SecretManager::Stronghold(stronghold) = &*self.secret_manager.read().await {
            stronghold.snapshot_path.clone()
        } else {
            PathBuf::from("wallet.stronghold")
        };

        if let Some(mut read_secret_manager) = read_secret_manager {
            // We have to replace the snapshot path with the current one, when building stronghold
            if let SecretManagerDto::Stronghold(stronghold_dto) = &mut read_secret_manager {
                stronghold_dto.snapshot_path = new_snapshot_path.display().to_string();
            }

            let restored_secret_manager = SecretManager::from_config(&read_secret_manager)
                .map_err(|_| crate::wallet::Error::Backup("invalid secret_manager"))?;

            // Copy Stronghold file so the seed is available in the new location
            fs::copy(backup_path, new_snapshot_path)?;

            if let SecretManager::Stronghold(stronghold) = &restored_secret_manager {
                // Set password to restored secret manager
                stronghold.set_password(stronghold_password).await?;
            }
            *self.secret_manager.write().await = restored_secret_manager;
        } else {
            // If no secret manager data was in the backup, just copy the Stronghold file so the seed is available in
            // the new location.
            fs::copy(backup_path, new_snapshot_path)?;
        }

        if ignore_if_bip_path_mismatch.is_none() {
            if let Some(read_client_options) = read_client_options {
                self.set_client_options(read_client_options).await?;
            }
        }

        if !ignore_backup_values {
            if let Some(read_wallet_data) = read_wallet_data {
                let restore_wallet = ignore_if_bech32_hrp_mismatch.map_or(true, |expected_bech32_hrp| {
                    // Only restore if bech32 hrps match
                    read_wallet_data.address.hrp() == &expected_bech32_hrp
                });

                if restore_wallet {
                    *self.data_mut().await = read_wallet_data;
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
                .with_bip_path(self.data().await.bip_path);

            wallet_builder.save(self.storage_manager()).await?;

            // also save wallet data to db
            self.storage_manager().save_wallet_data(&*self.data().await).await?;
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
    /// Replaces client_options, bip path, secret_manager and wallet. Returns an error if the wallet was already
    /// created If Stronghold is used as secret_manager, the existing Stronghold file will be overwritten. If a
    /// mnemonic was stored, it will be gone.
    /// if ignore_if_bip_path_mismatch.is_some(), client options will not be restored
    /// if ignore_if_bip_path_mismatch == Some(true), client options bip path and wallet will not be restored if the
    /// bip path doesn't match
    /// If a bech32 hrp is provided to ignore_if_bech32_hrp_mismatch, that doesn't match the one of the current address,
    /// the wallet will not be restored.
    pub async fn restore_backup(
        &self,
        backup_path: PathBuf,
        stronghold_password: impl Into<Password> + Send,
        ignore_if_bip_path_mismatch: Option<bool>,
        ignore_if_bech32_hrp_mismatch: Option<Hrp>,
    ) -> crate::wallet::Result<()> {
        let stronghold_password = stronghold_password.into();

        log::debug!("[restore_backup] loading stronghold backup");

        if !backup_path.is_file() {
            return Err(crate::wallet::Error::Backup("backup path doesn't exist"));
        }

        let wallet_data = self.data().await;

        // We don't want to overwrite a possible existing wallet
        if !wallet_data.outputs.is_empty() {
            return Err(crate::wallet::Error::Backup(
                "can't restore backup when there is already a wallet",
            ));
        }

        let curr_bip_path = wallet_data.bip_path;

        // Explicitly drop the data to avoid contention
        drop(wallet_data);

        // We'll create a new stronghold to load the backup
        let new_stronghold = StrongholdSecretManager::builder()
            .password(stronghold_password.clone())
            .build(backup_path.clone())?;

        let (read_client_options, read_secret_manager, read_wallet_data) =
            read_wallet_data_from_stronghold_snapshot::<StrongholdSecretManager>(&new_stronghold).await?;

        let read_bip_path = read_wallet_data.as_ref().and_then(|data| data.bip_path);

        // If the bip path is not matching the current one, we may ignore the backup
        let ignore_backup_values = ignore_if_bip_path_mismatch.map_or(false, |ignore| {
            if ignore {
                // TODO: #1279 okay that if both are none we always load the backup values?
                curr_bip_path != read_bip_path
            } else {
                false
            }
        });

        if !ignore_backup_values {
            self.data_mut().await.bip_path = read_bip_path;
        }

        if let Some(mut read_secret_manager) = read_secret_manager {
            // Get the current snapshot path if set
            let new_snapshot_path = self.secret_manager.read().await.snapshot_path.clone();

            read_secret_manager.snapshot_path = new_snapshot_path.to_string_lossy().into_owned();

            let restored_secret_manager = StrongholdSecretManager::from_config(&read_secret_manager)
                .map_err(|_| crate::wallet::Error::Backup("invalid secret_manager"))?;

            // Copy Stronghold file so the seed is available in the new location
            fs::copy(backup_path, new_snapshot_path)?;

            // Set password to restored secret manager
            restored_secret_manager.set_password(stronghold_password).await?;
            *self.secret_manager.write().await = restored_secret_manager;
        }

        // Update Wallet with read data
        if ignore_if_bip_path_mismatch.is_none() {
            if let Some(read_client_options) = read_client_options {
                // If the nodes are from the same network as the current client options, then extend it
                self.set_client_options(read_client_options).await?;
            }
        }

        if !ignore_backup_values {
            if let Some(read_wallet_data) = read_wallet_data {
                let restore_wallet = ignore_if_bech32_hrp_mismatch.map_or(true, |expected_bech32_hrp| {
                    // Only restore if bech32 hrps match
                    read_wallet_data.address.hrp() == &expected_bech32_hrp
                });

                if restore_wallet {
                    *self.data_mut().await = read_wallet_data;
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
                .with_bip_path(self.data().await.bip_path);

            wallet_builder.save(self.storage_manager()).await?;

            // also save wallet data to db
            self.storage_manager().save_wallet_data(&*self.data().await).await?;
        }

        Ok(())
    }
}
