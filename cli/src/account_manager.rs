// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::env::var_os;

use iota_sdk::{
    client::secret::{stronghold::StrongholdSecretManager, SecretManager},
    wallet::account_manager::AccountManager,
};

use crate::{
    command::account_manager::{
        backup_command, change_password_command, init_command, mnemonic_command, new_command, restore_command,
        set_node_command, sync_command, AccountManagerCli, AccountManagerCommand, InitParameters,
    },
    error::Error,
    helper::get_password,
    println_log_info,
};

pub async fn new_account_manager(cli: AccountManagerCli) -> Result<(Option<AccountManager>, Option<String>), Error> {
    if let Some(AccountManagerCommand::Mnemonic) = cli.command {
        mnemonic_command().await?;
        return Ok((None, None));
    }

    let storage_path = var_os("WALLET_DATABASE_PATH").map_or_else(
        || "./stardust-cli-wallet-db".to_string(),
        |os_str| os_str.into_string().expect("invalid WALLET_DATABASE_PATH"),
    );
    let snapshot_path = std::path::Path::new("./stardust-cli-wallet.stronghold");
    let snapshot_exists = snapshot_path.exists();
    let password = if let Some(AccountManagerCommand::Restore { .. }) = &cli.command {
        get_password("Stronghold password", false)?
    } else {
        get_password("Stronghold password", !snapshot_path.exists())?
    };
    let secret_manager = SecretManager::Stronghold(
        StrongholdSecretManager::builder()
            .password(&password)
            .build(snapshot_path)?,
    );

    let (account_manager, account) = if let Some(command) = cli.command {
        if let AccountManagerCommand::Init(init_parameters) = command {
            (init_command(secret_manager, storage_path, init_parameters).await?, None)
        } else if let AccountManagerCommand::Restore { backup_path } = command {
            (
                restore_command(secret_manager, storage_path, backup_path, password).await?,
                None,
            )
        } else {
            let account_manager = AccountManager::builder()
                .with_secret_manager(secret_manager)
                .with_storage_path(&storage_path)
                .finish()
                .await?;
            let mut account = None;

            match command {
                AccountManagerCommand::Backup { path } => {
                    backup_command(&account_manager, path, &password).await?;
                    return Ok((None, None));
                }
                AccountManagerCommand::ChangePassword => change_password_command(&account_manager, &password).await?,
                AccountManagerCommand::New { alias } => account = Some(new_command(&account_manager, alias).await?),
                AccountManagerCommand::SetNode { url } => set_node_command(&account_manager, url).await?,
                AccountManagerCommand::Sync => sync_command(&account_manager).await?,
                // PANIC: this will never happen because these variants have already been checked.
                AccountManagerCommand::Init(_)
                | AccountManagerCommand::Mnemonic
                | AccountManagerCommand::Restore { .. } => unreachable!(),
            };

            (account_manager, account)
        }
    } else {
        if snapshot_exists {
            (
                AccountManager::builder()
                    .with_secret_manager(secret_manager)
                    .with_storage_path(&storage_path)
                    .finish()
                    .await?,
                None,
            )
        } else {
            println_log_info!("Initializing wallet with default values.");
            (
                init_command(secret_manager, storage_path, InitParameters::default()).await?,
                None,
            )
        }
    };

    Ok((Some(account_manager), account))
}
