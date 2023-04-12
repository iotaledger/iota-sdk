// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::env::var_os;

use iota_sdk::{
    client::secret::{stronghold::StrongholdSecretManager, SecretManager},
    wallet::Wallet,
};

use crate::{
    command::wallet::{
        backup_command, change_password_command, init_command, mnemonic_command, new_command, restore_command,
        set_node_command, sync_command, InitParameters, WalletCli, WalletCommand,
    },
    error::Error,
    helper::{get_decision, get_password, pick_account},
    println_log_info,
};

pub async fn new_wallet(cli: WalletCli) -> Result<(Option<Wallet>, Option<String>), Error> {
    if let Some(WalletCommand::Mnemonic) = cli.command {
        mnemonic_command().await?;
        return Ok((None, None));
    }

    let storage_path = var_os("WALLET_DATABASE_PATH").map_or_else(
        || "./stardust-cli-wallet-db".to_string(),
        |os_str| os_str.into_string().expect("invalid WALLET_DATABASE_PATH"),
    );
    let storage_exists = std::path::Path::new(&storage_path).exists();
    let snapshot_path = std::path::Path::new("./stardust-cli-wallet.stronghold");
    let snapshot_exists = snapshot_path.exists();
    let password = if let Some(WalletCommand::Restore { .. }) = &cli.command {
        get_password("Stronghold password", false)?
    } else {
        get_password("Stronghold password", !snapshot_path.exists())?
    };
    let secret_manager = SecretManager::Stronghold(
        StrongholdSecretManager::builder()
            .password(&password)
            .build(snapshot_path)?,
    );

    let (wallet, account) = if let Some(command) = cli.command {
        if let WalletCommand::Init(init_parameters) = command {
            (
                Some(init_command(secret_manager, storage_path, init_parameters).await?),
                None,
            )
        } else if let WalletCommand::Restore { backup_path } = command {
            (
                Some(restore_command(secret_manager, storage_path, backup_path, password).await?),
                None,
            )
        } else {
            let wallet = Wallet::builder()
                .with_secret_manager(secret_manager)
                .with_storage_path(&storage_path)
                .finish()
                .await?;
            let mut account = None;

            match command {
                WalletCommand::Backup { backup_path } => {
                    backup_command(&wallet, backup_path, &password).await?;
                    return Ok((None, None));
                }
                WalletCommand::ChangePassword => change_password_command(&wallet, &password).await?,
                WalletCommand::New { alias } => account = Some(new_command(&wallet, alias).await?),
                WalletCommand::SetNode { url } => set_node_command(&wallet, url).await?,
                WalletCommand::Sync => sync_command(&wallet).await?,
                // PANIC: this will never happen because these variants have already been checked.
                WalletCommand::Init(_) | WalletCommand::Mnemonic | WalletCommand::Restore { .. } => unreachable!(),
            };

            (Some(wallet), account)
        }
    } else if storage_exists && snapshot_exists {
        let wallet = Wallet::builder()
            .with_secret_manager(secret_manager)
            .with_storage_path(&storage_path)
            .finish()
            .await?;
        if wallet.get_accounts().await?.is_empty() {
            // ask the new user whether a default account should be created
            if get_decision("Initialize a default account?")? {
                println_log_info!("Initializing default account.");
                let account = new_command(&wallet, None).await?;
                (Some(wallet), Some(account))
            } else {
                (Some(wallet), None)
            }
        } else if let Some(account_index) = pick_account(&wallet).await? {
            let accounts = wallet.get_accounts().await?;
            let account = accounts[account_index as usize].alias().await;
            (Some(wallet), Some(account))
        } else {
            (Some(wallet), None)
        }
    } else {
        // first run
        if get_decision("Initialize a new wallet with default values?")? {
            println_log_info!("Initializing wallet with default values.");
            let wallet = init_command(secret_manager, storage_path, InitParameters::default()).await?;

            // ask the new user whether a default account should be created
            if get_decision("Initialize a default account?")? {
                println_log_info!("Initializing default account.");
                let account = new_command(&wallet, None).await?;
                (Some(wallet), Some(account))
            } else {
                (Some(wallet), None)
            }
        } else {
            (None, None)
        }
    };
    Ok((wallet, account))
}
