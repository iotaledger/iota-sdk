// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod stronghold_snapshot;

use std::{fs, path::PathBuf};

use self::stronghold_snapshot::read_fields_from_stronghold_snapshot;
#[cfg(feature = "storage")]
use crate::wallet::WalletBuilder;
use crate::{
    client::{
        secret::{stronghold::StrongholdSecretManager, SecretManager, SecretManagerConfig, SecretManagerDto},
        utils::Password,
    },
    types::block::address::Hrp,
    wallet::{core::WalletLedgerDto, Wallet, WalletError},
};

impl Wallet {
    /// Backup the wallet in a Stronghold snapshot file.
    ///
    /// `stronghold_password` must be the current one when Stronghold is used as SecretManager.
    pub async fn backup_to_stronghold_snapshot(
        &self,
        backup_path: PathBuf,
        stronghold_password: impl Into<Password> + Send,
    ) -> Result<(), WalletError> {
        let stronghold_password = stronghold_password.into();

        log::debug!("[backup] creating a stronghold backup");
        let secret_manager = self.secret_manager.read().await;

        match &*secret_manager {
            // Backup with existing stronghold
            SecretManager::Stronghold(stronghold) => {
                stronghold.set_password(stronghold_password).await?;
                self.write_fields_to_stronghold_snapshot(stronghold).await?;
                // Write snapshot to backup path
                stronghold.write_stronghold_snapshot(Some(&backup_path)).await?;
            }
            // Backup with new stronghold
            _ => {
                // If the SecretManager is not Stronghold we'll create a new one for the backup
                let backup_stronghold = StrongholdSecretManager::builder()
                    .password(stronghold_password)
                    .build(backup_path)?;

                self.write_fields_to_stronghold_snapshot(&backup_stronghold).await?;

                // Write snapshot to backup path
                backup_stronghold.write_stronghold_snapshot(None).await?;
            }
        }

        Ok(())
    }

    /// Restore a backup from a Stronghold snapshot file.
    ///
    /// Replaces client_options, bip_path, secret_manager and wallet. Returns an error if the wallet was already
    /// created If Stronghold is used as secret_manager, the existing Stronghold file will be overwritten. If a
    /// mnemonic was stored, it will be gone.
    /// if ignore_if_bip_path_mismatch.is_some(), client options will not be restored
    /// if ignore_if_bip_path_mismatch == Some(true), client options coin type and wallet will not be restored if the
    /// coin type doesn't match
    /// If a bech32 hrp is provided to ignore_if_bech32_hrp_mismatch, that doesn't match the one of the current address,
    /// the wallet will not be restored.
    pub async fn restore_from_stronghold_snapshot(
        &self,
        backup_path: PathBuf,
        stronghold_password: impl Into<Password> + Send,
        ignore_if_bip_path_mismatch: Option<bool>,
        ignore_if_bech32_hrp_mismatch: Option<Hrp>,
    ) -> Result<(), WalletError> {
        let stronghold_password = stronghold_password.into();

        log::debug!("[restore_backup] loading stronghold backup");

        if !backup_path.is_file() {
            return Err(WalletError::Backup("backup path doesn't exist"));
        }

        let wallet_ledger = self.ledger().await;

        // We don't want to overwrite a possible existing wallet
        if !wallet_ledger.outputs.is_empty() {
            return Err(WalletError::Backup(
                "can't restore backup when there is already a wallet",
            ));
        }

        let curr_bip_path = self.bip_path().await;

        // Explicitly drop the data to avoid contention
        drop(wallet_ledger);

        // We'll create a new stronghold to load the backup
        let new_stronghold = StrongholdSecretManager::builder()
            .password(stronghold_password.clone())
            .build(backup_path.clone())?;

        let (read_address, read_bip_path, read_alias, read_client_options, read_secret_manager, read_wallet_ledger) =
            read_fields_from_stronghold_snapshot::<SecretManager>(&new_stronghold).await?;

        // If the bip path is not matching the current one, we may ignore the backup
        let ignore_backup_values = ignore_if_bip_path_mismatch.map_or(false, |ignore| {
            if ignore {
                // TODO: is it okay that if both are none we always load the backup values?
                curr_bip_path != read_bip_path
            } else {
                false
            }
        });

        if !ignore_backup_values {
            *self.bip_path_mut().await = read_bip_path;
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
                .map_err(|_| WalletError::Backup("invalid secret_manager"))?;

            // Copy Stronghold file so the seed is available in the new location
            if backup_path.canonicalize()? != new_snapshot_path.canonicalize()? {
                fs::copy(backup_path, new_snapshot_path)?;
            }

            if let SecretManager::Stronghold(stronghold) = &restored_secret_manager {
                // Set password to restored secret manager
                stronghold.set_password(stronghold_password).await?;
            }
            *self.secret_manager.write().await = restored_secret_manager;
        } else {
            // If no secret manager data was in the backup, just copy the Stronghold file so the seed is available in
            // the new location.
            if backup_path.canonicalize()? != new_snapshot_path.canonicalize()? {
                fs::copy(backup_path, new_snapshot_path)?;
            }
        }

        if ignore_if_bip_path_mismatch.is_none() {
            if let Some(read_client_options) = read_client_options {
                self.set_client_options(read_client_options).await?;
            }
        }

        if !ignore_backup_values {
            if let Some(read_wallet_ledger) = read_wallet_ledger {
                let restore_wallet = ignore_if_bech32_hrp_mismatch.map_or(true, |expected_bech32_hrp| {
                    // Only restore if bech32 hrps match
                    read_address.hrp() == &expected_bech32_hrp
                });

                if restore_wallet {
                    *self.address_mut().await = read_address;
                    *self.bip_path_mut().await = read_bip_path;
                    *self.alias_mut().await = read_alias;
                    *self.ledger_mut().await = read_wallet_ledger;
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
                .with_address(self.address().await)
                .with_bip_path(self.bip_path().await)
                .with_alias(self.alias().await);

            wallet_builder.save(self.storage_manager()).await?;

            // also save wallet ledger to db
            self.storage_manager()
                .save_wallet_ledger(&WalletLedgerDto::from(&*self.ledger().await))
                .await?;
        }

        Ok(())
    }
}

impl Wallet<StrongholdSecretManager> {
    /// Backup the wallet in a Stronghold snapshot file.
    ///
    /// `stronghold_password` must be the current one when Stronghold is used as SecretManager.
    pub async fn backup_to_stronghold_snapshot(
        &self,
        backup_path: PathBuf,
        stronghold_password: impl Into<Password> + Send,
    ) -> Result<(), WalletError> {
        log::debug!("[backup] creating a stronghold backup");
        let secret_manager = self.secret_manager.read().await;

        secret_manager.set_password(stronghold_password).await?;

        self.write_fields_to_stronghold_snapshot(&secret_manager).await?;

        // Write snapshot to backup path
        secret_manager.write_stronghold_snapshot(Some(&backup_path)).await?;

        Ok(())
    }

    /// Restore a backup from a Stronghold file.
    ///
    /// Replaces client_options, bip path, secret_manager and wallet. Returns an error if the wallet was already
    /// created If Stronghold is used as secret_manager, the existing Stronghold file will be overwritten. If a
    /// mnemonic was stored, it will be gone.
    /// if ignore_if_bip_path_mismatch.is_some(), client options will not be restored
    /// if ignore_if_bip_path_mismatch == Some(true), client options bip path and wallet will not be restored if the
    /// bip path doesn't match
    /// If a bech32 hrp is provided to ignore_if_bech32_hrp_mismatch, that doesn't match the one of the current address,
    /// the wallet will not be restored.
    pub async fn restore_from_stronghold_snapshot(
        &self,
        backup_path: PathBuf,
        stronghold_password: impl Into<Password> + Send,
        ignore_if_bip_path_mismatch: Option<bool>,
        ignore_if_bech32_hrp_mismatch: Option<Hrp>,
    ) -> Result<(), WalletError> {
        let stronghold_password = stronghold_password.into();

        log::debug!("[restore_backup] loading stronghold backup");

        if !backup_path.is_file() {
            return Err(WalletError::Backup("backup path doesn't exist"));
        }

        let wallet_ledger = self.ledger().await;

        // We don't want to overwrite a possible existing wallet
        if !wallet_ledger.outputs.is_empty() {
            return Err(WalletError::Backup(
                "can't restore backup when there is already a wallet",
            ));
        }

        let curr_bip_path = self.bip_path().await;

        // Explicitly drop the data to avoid contention
        drop(wallet_ledger);

        // We'll create a new stronghold to load the backup
        let new_stronghold = StrongholdSecretManager::builder()
            .password(stronghold_password.clone())
            .build(backup_path.clone())?;

        let (read_address, read_bip_path, read_alias, read_client_options, read_secret_manager, read_wallet_ledger) =
            read_fields_from_stronghold_snapshot::<StrongholdSecretManager>(&new_stronghold).await?;

        // If the bip path is not matching the current one, we may ignore the backup
        let ignore_backup_values = ignore_if_bip_path_mismatch.map_or(false, |ignore| {
            if ignore {
                // TODO: is it okay that if both are none we always load the backup values?
                curr_bip_path != read_bip_path
            } else {
                false
            }
        });

        if !ignore_backup_values {
            *self.bip_path_mut().await = read_bip_path;
        }

        if let Some(mut read_secret_manager) = read_secret_manager {
            // Get the current snapshot path if set
            let new_snapshot_path = self.secret_manager.read().await.snapshot_path.clone();

            read_secret_manager.snapshot_path = new_snapshot_path.to_string_lossy().into_owned();

            let restored_secret_manager = StrongholdSecretManager::from_config(&read_secret_manager)
                .map_err(|_| WalletError::Backup("invalid secret_manager"))?;

            // Copy Stronghold file so the seed is available in the new location
            if backup_path.canonicalize()? != new_snapshot_path.canonicalize()? {
                fs::copy(backup_path, new_snapshot_path)?;
            }

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
            if let Some(read_wallet_ledger) = read_wallet_ledger {
                let restore_wallet = ignore_if_bech32_hrp_mismatch.map_or(true, |expected_bech32_hrp| {
                    // Only restore if bech32 hrps match
                    read_address.hrp() == &expected_bech32_hrp
                });

                if restore_wallet {
                    *self.address_mut().await = read_address;
                    *self.bip_path_mut().await = read_bip_path;
                    *self.alias_mut().await = read_alias;
                    *self.ledger_mut().await = read_wallet_ledger;
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
                .with_address(self.address().await)
                .with_bip_path(self.bip_path().await)
                .with_alias(self.alias().await);

            wallet_builder.save(self.storage_manager()).await?;

            // also save wallet ledger to db
            self.storage_manager()
                .save_wallet_ledger(&WalletLedgerDto::from(&*self.ledger().await))
                .await?;
        }

        Ok(())
    }
}
