// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod stronghold_snapshot;

use std::{fs, path::PathBuf};

use self::stronghold_snapshot::read_wallet_data_from_stronghold_snapshot;
#[cfg(feature = "storage")]
use crate::wallet::WalletBuilder;
use crate::{
    client::{
        secret::{stronghold::StrongholdSecretManager, DowncastSecretManager, SecretManage},
        utils::Password,
    },
    types::block::address::Hrp,
    wallet::Wallet,
};

impl Wallet {
    /// Backup the wallet data in a Stronghold file
    /// stronghold_password must be the current one when Stronghold is used as SecretManager.
    pub async fn backup<S: 'static + SecretManage>(
        &self,
        secret_manager: &S,
        backup_path: PathBuf,
        stronghold_password: impl Into<Password> + Send,
    ) -> crate::wallet::Result<()> {
        let stronghold_password = stronghold_password.into();

        log::debug!("[backup] creating a stronghold backup");

        // Backup with existing stronghold
        if let Ok(stronghold) = secret_manager.as_stronghold() {
            stronghold.set_password(stronghold_password).await?;
            self.store_data_to_stronghold(stronghold).await?;
            // Write snapshot to backup path
            stronghold.write_stronghold_snapshot(Some(&backup_path)).await?;
        } else {
            // Backup with new stronghold
            // If the SecretManager is not Stronghold we'll create a new one for the backup
            let backup_stronghold = StrongholdSecretManager::builder()
                .password(stronghold_password)
                .build(backup_path)?;

            self.store_data_to_stronghold(&backup_stronghold).await?;

            // Write snapshot to backup path
            backup_stronghold.write_stronghold_snapshot(None).await?;
        }

        Ok(())
    }

    /// Restore a backup from a Stronghold file
    /// Replaces client_options, bip_path and wallet. Returns an error if the wallet was already
    /// created If Stronghold is used as secret_manager, the existing Stronghold file will be overwritten. If a
    /// mnemonic was stored, it will be gone.
    /// if ignore_if_bip_path_mismatch.is_some(), client options will not be restored
    /// if ignore_if_bip_path_mismatch == Some(true), client options coin type and wallet will not be restored if the
    /// coin type doesn't match
    /// If a bech32 hrp is provided to ignore_if_bech32_hrp_mismatch, that doesn't match the one of the current address,
    /// the wallet will not be restored.
    pub async fn restore_backup<S: 'static + SecretManage>(
        &self,
        secret_manager: &S,
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

        // Get the current snapshot path if set
        let new_snapshot_path = if let Ok(stronghold) = secret_manager.as_stronghold() {
            stronghold.snapshot_path.clone()
        } else {
            PathBuf::from("wallet.stronghold")
        };

        // TODO: Might want to save the snapshot path as a key somewhere
        fs::copy(&backup_path, new_snapshot_path)?;

        // We'll create a new stronghold to load the backup
        let stronghold = StrongholdSecretManager::builder()
            .password(stronghold_password)
            .build(backup_path)?;

        let (read_client_options, read_wallet_data) = read_wallet_data_from_stronghold_snapshot(&stronghold).await?;

        let read_bip_path = read_wallet_data.as_ref().and_then(|data| data.bip_path);

        let mut wallet_data = self.data.write().await;

        // If the bip path is not matching the current one, we may ignore the backup
        let ignore_backup_values = ignore_if_bip_path_mismatch.map_or(false, |ignore| {
            if ignore {
                // TODO: #1279 okay that if both are none we always load the backup values?
                wallet_data.bip_path != read_bip_path
            } else {
                false
            }
        });

        if !ignore_backup_values {
            wallet_data.bip_path = read_bip_path;
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
                    *wallet_data = read_wallet_data;
                }
            }
        }

        // store new data
        #[cfg(feature = "storage")]
        {
            let wallet_builder = WalletBuilder::new()
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

            wallet_builder.save(&*self.storage_manager.read().await).await?;

            // also save wallet data to db
            self.save(Some(&wallet_data)).await?;
        }

        Ok(())
    }
}
