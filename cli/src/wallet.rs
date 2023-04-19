// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::wallet::Wallet;

use crate::{
    command::wallet::{
        add_account, backup_command, change_password_command, init_command, mnemonic_command, new_command,
        restore_command, set_node_command, sync_command, unlock_wallet, InitParameters, WalletCli, WalletCommand,
    },
    error::Error,
    helper::{get_account_name, get_decision, get_password, pick_account, print_account_help, print_wallet_help},
    println_log_error, println_log_info,
};

pub async fn new_wallet(cli: WalletCli) -> Result<(Option<Wallet>, Option<String>), Error> {
    let storage_path = std::path::Path::new(&cli.wallet_db_path);
    let snapshot_path = std::path::Path::new(&cli.stronghold_snapshot_path);

    let (wallet, account) = if let Some(command) = cli.command {
        match command {
            WalletCommand::Init(init_parameters) => {
                let wallet = init_command(storage_path, snapshot_path, init_parameters).await?;
                (Some(wallet), None)
            }
            WalletCommand::Restore { backup_path } => {
                let wallet = restore_command(storage_path, snapshot_path, std::path::Path::new(&backup_path)).await?;
                (Some(wallet), None)
            }
            WalletCommand::Backup { backup_path } => {
                backup_command(storage_path, snapshot_path, std::path::Path::new(&backup_path)).await?;
                return Ok((None, None));
            }
            WalletCommand::ChangePassword => {
                let wallet = change_password_command(storage_path, snapshot_path).await?;
                (Some(wallet), None)
            }
            WalletCommand::New { alias } => {
                let (wallet, account) = new_command(storage_path, snapshot_path, alias).await?;
                (Some(wallet), Some(account))
            }
            WalletCommand::SetNode { url } => {
                let wallet = set_node_command(storage_path, snapshot_path, url).await?;
                (Some(wallet), None)
            }
            WalletCommand::Sync => {
                let wallet = sync_command(storage_path, snapshot_path).await?;
                (Some(wallet), None)
            }
            WalletCommand::Mnemonic => {
                mnemonic_command().await?;
                return Ok((None, None));
            }
        }
    } else {
        // no command provided, i.e. `> ./wallet`
        match (storage_path.exists(), snapshot_path.exists()) {
            (true, true) => {
                let password = get_password("Stronghold password", false)?;
                let wallet = unlock_wallet(storage_path, snapshot_path, &password).await?;
                if wallet.get_accounts().await?.is_empty() {
                    // ask the user whether a default account should be created
                    if get_decision("Create initial account?")? {
                        let alias = get_account_name("New account name", &wallet).await?;
                        let account = add_account(&wallet, Some(alias)).await?;
                        println_log_info!("Created initial account.");
                        print_account_help();
                        (Some(wallet), Some(account))
                    } else {
                        (Some(wallet), None)
                    }
                } else if let Some(account_handle) = pick_account(&wallet).await? {
                    let account = account_handle.alias().await;
                    (Some(wallet), Some(account))
                } else {
                    (Some(wallet), None)
                }
            }
            (false, false) => {
                if get_decision("Create a new wallet with default parameters?")? {
                    let wallet = init_command(storage_path, snapshot_path, InitParameters::default()).await?;
                    println_log_info!("Created new wallet.");

                    // ask the user whether a default account should be created
                    if get_decision("Create initial account?")? {
                        let alias = get_account_name("New account name", &wallet).await?;
                        let account = add_account(&wallet, Some(alias)).await?;
                        println_log_info!("Created initial account.");
                        print_account_help();
                        (Some(wallet), Some(account))
                    } else {
                        (Some(wallet), None)
                    }
                } else {
                    print_wallet_help();
                    (None, None)
                }
            }
            (true, false) => {
                println_log_error!("Stronghold snapshot not found at '{}'.", snapshot_path.display());
                (None, None)
            }
            (false, true) => {
                println_log_error!("Wallet database not found at '{}'.", storage_path.display());
                (None, None)
            }
        }
    };
    Ok((wallet, account))
}
