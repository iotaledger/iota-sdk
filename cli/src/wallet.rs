// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use iota_sdk::{
    client::secret::{ledger_nano::LedgerSecretManager, stronghold::StrongholdSecretManager, SecretManager},
    wallet::{account::types::AccountIdentifier, Wallet},
};

use crate::{
    command::wallet::{
        accounts_command, add_account, backup_command_stronghold, change_password_command, init_command,
        migrate_stronghold_snapshot_v2_to_v3_command, mnemonic_command, new_account_command, node_info_command,
        restore_command_stronghold, set_node_url_command, set_pow_command, sync_command, unlock_wallet_ledgernano,
        unlock_wallet_stronghold, InitParameters, WalletCli, WalletCommand,
    },
    error::Error,
    helper::{
        enter_or_generate_mnemonic, get_account_alias, get_decision, get_password, import_mnemonic, pick_account,
        select_secret_manager, SecretManagerChoice,
    },
    println_log_error, println_log_info,
};

pub async fn new_wallet(cli: WalletCli) -> Result<(Option<Wallet>, Option<AccountIdentifier>), Error> {
    let storage_path = Path::new(&cli.wallet_db_path);
    let (wallet, account_id) = if let Some(command) = cli.command {
        if command.is_unlock_command() {
            let (unlocked_wallet, password) = match &cli.secret_manager {
                SecretManagerChoice::Stronghold => {
                    let snapshot_path = Path::new(&cli.stronghold_snapshot_path);
                    let password = get_password("Stronghold password", false)?;
                    (
                        unlock_wallet_stronghold(storage_path, snapshot_path, password.clone()).await?,
                        Some(password),
                    )
                }
                SecretManagerChoice::LedgerNano => (unlock_wallet_ledgernano(storage_path, false).await?, None),
                SecretManagerChoice::LedgerNanoSimulator => (unlock_wallet_ledgernano(storage_path, true).await?, None),
            };

            match command {
                WalletCommand::Accounts => {
                    accounts_command(&unlocked_wallet).await?;
                    return Ok((None, None));
                }
                WalletCommand::Backup { backup_path } => match cli.secret_manager {
                    SecretManagerChoice::Stronghold => {
                        // Panic: save to unwrap since this is a Stronghold secret manager
                        let password = password.unwrap();
                        backup_command_stronghold(&unlocked_wallet, &password, Path::new(&backup_path)).await?;
                        return Ok((None, None));
                    }
                    _ => {
                        println_log_info!("only Stronghold backup supported");
                        return Ok((None, None));
                    }
                },
                WalletCommand::ChangePassword => match cli.secret_manager {
                    SecretManagerChoice::Stronghold => {
                        // Panic: save to unwrap since this is a Stronghold secret manager
                        let current_password = password.unwrap();
                        change_password_command(&unlocked_wallet, current_password).await?;
                        (Some(unlocked_wallet), None)
                    }
                    _ => {
                        println_log_info!("only Stronghold password change supported");
                        return Ok((None, None));
                    }
                },
                WalletCommand::NewAccount { alias } => {
                    let account = new_account_command(&unlocked_wallet, alias).await?;
                    (Some(unlocked_wallet), Some(account))
                }
                WalletCommand::SetNodeUrl { url } => {
                    set_node_url_command(&unlocked_wallet, url).await?;
                    (Some(unlocked_wallet), None)
                }
                WalletCommand::SetPow {
                    local_pow,
                    worker_count,
                } => {
                    set_pow_command(&unlocked_wallet, local_pow, worker_count).await?;
                    (Some(unlocked_wallet), None)
                }
                WalletCommand::Sync => {
                    sync_command(&unlocked_wallet).await?;
                    (Some(unlocked_wallet), None)
                }
                WalletCommand::NodeInfo => {
                    node_info_command(&unlocked_wallet).await?;
                    return Ok((None, None));
                }
                _ => {
                    panic!("invalid wallet command");
                }
            }
        } else {
            match command {
                WalletCommand::Init(init_parameters) => {
                    if storage_path.exists() {
                        return Err(Error::Miscellaneous(format!(
                            "cannot initialize: {} already exists",
                            storage_path.display()
                        )));
                    }
                    let secret_manager = match select_secret_manager().await? {
                        SecretManagerChoice::Stronghold => {
                            let snapshot_path = Path::new(&cli.stronghold_snapshot_path);

                            if snapshot_path.exists() {
                                return Err(Error::Miscellaneous(format!(
                                    "cannot initialize: {} already exists",
                                    snapshot_path.display()
                                )));
                            }

                            let password = get_password("Stronghold password", true)?;
                            let mnemonic = match &init_parameters.mnemonic_file_path {
                                Some(path) => import_mnemonic(path).await?,
                                None => enter_or_generate_mnemonic().await?,
                            };

                            let secret_manager = StrongholdSecretManager::builder()
                                .password(password)
                                .build(snapshot_path)?;
                            secret_manager.store_mnemonic(mnemonic).await?;

                            SecretManager::Stronghold(secret_manager)
                        }
                        SecretManagerChoice::LedgerNano => SecretManager::LedgerNano(LedgerSecretManager::new(false)),
                        SecretManagerChoice::LedgerNanoSimulator => {
                            SecretManager::LedgerNano(LedgerSecretManager::new(true))
                        }
                    };

                    let wallet = init_command(storage_path, secret_manager, init_parameters).await?;
                    println_log_info!("Created new wallet.");
                    (Some(wallet), None)
                }
                WalletCommand::Restore { backup_path } => match cli.secret_manager {
                    SecretManagerChoice::Stronghold => {
                        let snapshot_path = Path::new(&cli.stronghold_snapshot_path);
                        let wallet =
                            restore_command_stronghold(storage_path, snapshot_path, Path::new(&backup_path)).await?;
                        (Some(wallet), None)
                    }
                    _ => {
                        println_log_info!("only Stronghold restore supported at the moment");
                        return Ok((None, None));
                    }
                },
                WalletCommand::MigrateStrongholdSnapshotV2ToV3 { path } => {
                    migrate_stronghold_snapshot_v2_to_v3_command(path).await?;
                    return Ok((None, None));
                }
                WalletCommand::Mnemonic {
                    output_file_name,
                    output_stdout,
                } => {
                    mnemonic_command(output_file_name, output_stdout).await?;
                    return Ok((None, None));
                }
                _ => {
                    panic!("invalid wallet command");
                }
            }
        }
    } else {
        // no wallet command provided
        match cli.secret_manager {
            SecretManagerChoice::Stronghold => {
                let snapshot_path = Path::new(&cli.stronghold_snapshot_path);

                match (storage_path.exists(), snapshot_path.exists()) {
                    (false, false) => {
                        if get_decision("Create a new Stronghold wallet with default parameters?")? {
                            let password = get_password("Stronghold password", true)?;
                            let mnemonic = enter_or_generate_mnemonic().await?;
                            let secret_manager = StrongholdSecretManager::builder()
                                .password(password)
                                .build(snapshot_path)?;
                            secret_manager.store_mnemonic(mnemonic).await?;
                            let secret_manager = SecretManager::Stronghold(secret_manager);

                            let wallet = init_command(storage_path, secret_manager, InitParameters::default()).await?;
                            println_log_info!("Created new wallet.");
                            create_initial_account(wallet).await?
                        } else {
                            WalletCli::print_help()?;
                            (None, None)
                        }
                    }
                    (false, true) => {
                        println_log_error!("Wallet database not found at '{}'.", storage_path.display());
                        (None, None)
                    }
                    (true, true) => {
                        let password = get_password("Stronghold password", false)?;
                        let wallet = unlock_wallet_stronghold(storage_path, snapshot_path, password).await?;
                        if wallet.get_accounts().await?.is_empty() {
                            create_initial_account(wallet).await?
                        } else if let Some(alias) = cli.account {
                            (Some(wallet), Some(alias))
                        } else if let Some(account) = pick_account(&wallet).await? {
                            (Some(wallet), Some(account.alias().await.into()))
                        } else {
                            (Some(wallet), None)
                        }
                    }
                    (true, false) => {
                        println_log_error!("Stronghold snapshot not found at '{}'.", snapshot_path.display());
                        (None, None)
                    }
                }
            }
            SecretManagerChoice::LedgerNano => {
                if !storage_path.exists() {
                    if get_decision("Create a new Ledger Nano wallet with default parameters?")? {
                        let secret_manager = SecretManager::LedgerNano(LedgerSecretManager::new(false));
                        let wallet = init_command(storage_path, secret_manager, InitParameters::default()).await?;
                        println_log_info!("Created new wallet.");
                        create_initial_account(wallet).await?
                    } else {
                        (None, None)
                    }
                } else {
                    let wallet = unlock_wallet_ledgernano(storage_path, false).await?;
                    if wallet.get_accounts().await?.is_empty() {
                        create_initial_account(wallet).await?
                    } else if let Some(alias) = cli.account {
                        (Some(wallet), Some(alias))
                    } else if let Some(account) = pick_account(&wallet).await? {
                        (Some(wallet), Some(account.alias().await.into()))
                    } else {
                        (Some(wallet), None)
                    }
                }
            }
            SecretManagerChoice::LedgerNanoSimulator => {
                if !storage_path.exists() {
                    if get_decision("Create a new Ledger Nano Simulator wallet with default parameters?")? {
                        let secret_manager = SecretManager::LedgerNano(LedgerSecretManager::new(true));
                        let wallet = init_command(storage_path, secret_manager, InitParameters::default()).await?;
                        println_log_info!("Created new wallet.");
                        create_initial_account(wallet).await?
                    } else {
                        (None, None)
                    }
                } else {
                    let wallet = unlock_wallet_ledgernano(storage_path, true).await?;
                    if wallet.get_accounts().await?.is_empty() {
                        create_initial_account(wallet).await?
                    } else if let Some(alias) = cli.account {
                        (Some(wallet), Some(alias))
                    } else if let Some(account) = pick_account(&wallet).await? {
                        (Some(wallet), Some(account.alias().await.into()))
                    } else {
                        (Some(wallet), None)
                    }
                }
            }
        }
    };
    Ok((wallet, account_id))
}

async fn create_initial_account(wallet: Wallet) -> Result<(Option<Wallet>, Option<AccountIdentifier>), Error> {
    // Ask the user whether an initial account should be created.
    if get_decision("Create initial account?")? {
        let alias = get_account_alias("New account alias", &wallet).await?;
        let account_id = add_account(&wallet, Some(alias)).await?;
        println_log_info!("Created initial account.\nType `help` to see all available account commands.");
        Ok((Some(wallet), Some(account_id)))
    } else {
        Ok((Some(wallet), None))
    }
}
