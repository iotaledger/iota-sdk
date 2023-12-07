// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod stronghold_snapshot;

use std::{fs, path::PathBuf};

use self::stronghold_snapshot::read_wallet_data_from_stronghold_snapshot;
#[cfg(feature = "storage")]
use crate::wallet::WalletBuilder;
use crate::{
    client::{
        secret::{stronghold::StrongholdSecretManager, DowncastSecretManager, SecretManagerConfig},
        utils::Password,
    },
    types::block::address::Hrp,
    wallet::{core::SecretData, Wallet},
};

impl<S: 'static + SecretManagerConfig> Wallet<SecretData<S>> {
    /// Backup the wallet data in a Stronghold file
    /// stronghold_password must be the current one when Stronghold is used as SecretManager.
    pub async fn backup(
        &self,
        backup_path: PathBuf,
        stronghold_password: impl Into<Password> + Send,
    ) -> crate::wallet::Result<()> {
        let stronghold_password = stronghold_password.into();

        log::debug!("[backup] creating a stronghold backup");
        let secret_manager = self.secret_manager().read().await;

        match (&*secret_manager).as_stronghold() {
            // Backup with existing stronghold
            Ok(stronghold) => {
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

        let mut wallet_data = self.data.write().await;

        let mut secret_manager = self.secret_manager().as_ref().write().await;
        // Get the current snapshot path if set
        let new_snapshot_path = if let Ok(stronghold) = (&*secret_manager).as_stronghold() {
            stronghold.snapshot_path.clone()
        } else {
            PathBuf::from("wallet.stronghold")
        };

        // We'll create a new stronghold to load the backup
        let new_stronghold = StrongholdSecretManager::builder()
            .password(stronghold_password.clone())
            .build(backup_path.clone())?;

        let (loaded_client_options, loaded_secret_manager_config, loaded_wallet_data, loaded_secret_data) =
            read_wallet_data_from_stronghold_snapshot::<S>(&new_stronghold).await?;

        let loaded_pub_key_opts = loaded_secret_data.as_ref().map(|data| &data.public_key_options);

        // If the bip path is not matching the current one, we may ignore the backup
        let ignore_backup_values = ignore_if_bip_path_mismatch.map_or(false, |ignore| {
            if ignore {
                // TODO: #1279 okay that if both are none we always load the backup values?
                loaded_pub_key_opts.is_some_and(|opts| self.public_key_options() != opts)
            } else {
                false
            }
        });

        if !ignore_backup_values {
            if let Some(opts) = loaded_pub_key_opts {
                // TODO
                // self.secret_data.public_key_options = opts.clone();
            }
        }

        if let Some(config) = loaded_secret_manager_config {
            let mut loaded_secret_manager =
                S::from_config(&config).map_err(|_| crate::wallet::Error::Backup("invalid secret_manager"))?;
            // We have to replace the snapshot path with the current one, when building stronghold
            if let Ok(stronghold_dto) = loaded_secret_manager.as_stronghold_mut() {
                stronghold_dto.snapshot_path = new_snapshot_path.clone();
            }

            // Copy Stronghold file so the seed is available in the new location
            fs::copy(backup_path, new_snapshot_path)?;

            if let Ok(stronghold) = loaded_secret_manager.as_stronghold() {
                // Set password to restored secret manager
                stronghold.set_password(stronghold_password).await?;
            }
            *secret_manager = loaded_secret_manager;
        } else {
            // If no secret manager data was in the backup, just copy the Stronghold file so the seed is available in
            // the new location.
            fs::copy(backup_path, new_snapshot_path)?;
        }

        // drop secret manager, otherwise we get a deadlock in set_client_options() (there inside of save_wallet_data())
        drop(secret_manager);

        if ignore_if_bip_path_mismatch.is_none() {
            if let Some(read_client_options) = loaded_client_options {
                self.set_client_options(read_client_options).await?;
            }
        }

        if !ignore_backup_values {
            if let Some(read_wallet_data) = loaded_wallet_data {
                let restore_wallet = ignore_if_bech32_hrp_mismatch.map_or(true, |expected_bech32_hrp| {
                    // Only restore if bech32 hrps match
                    read_wallet_data.address.hrp() == &expected_bech32_hrp
                });

                if restore_wallet {
                    *wallet_data = read_wallet_data;
                }
            }
        }

        // store new data
        #[cfg(feature = "storage")]
        {
            let wallet_builder = WalletBuilder::new()
                .with_secret_manager_arc(self.secret_manager().clone())
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
                .with_public_key_options(self.public_key_options().clone())
                .with_signing_options(self.signing_options().clone());

            wallet_builder.save(&*self.storage_manager.read().await).await?;

            // also save wallet data to db
            self.save(Some(&wallet_data)).await?;
        }

        Ok(())
    }
}
