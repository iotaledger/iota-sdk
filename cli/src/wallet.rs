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
        restore_command_stronghold, set_node_url_command, set_pow_command, sync_command, InitParameters, WalletCli,
        WalletCommand,
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

    enum LinkedSecretManager {
        Stronghold {
            snapshot_path: std::path::PathBuf,
            snapshot_exists: Option<iota_sdk::client::Password>,
        },
        LedgerNano {
            is_connected: bool,
        },
    }

    let wallet_and_secret_manager = {
        if storage_path.is_dir() {
            match Wallet::builder().with_storage_path(storage_path).finish().await {
                Ok(wallet) => {
                    let linked_secret_manager = match &mut *wallet.get_secret_manager().write().await {
                        SecretManager::Stronghold(stronghold) => {
                            let snapshot_path = stronghold.snapshot_path.clone();
                            // `set_password` will trigger writing the snapshot file, so we need to make sure it already
                            // exists, otherwise we could run into an inconsistent wallet.
                            if snapshot_path.exists() {
                                let password = get_password("Stronghold password", false)?;
                                stronghold.set_password(password.clone()).await?;
                                LinkedSecretManager::Stronghold {
                                    snapshot_path,
                                    snapshot_exists: Some(password),
                                }
                            } else {
                                LinkedSecretManager::Stronghold {
                                    snapshot_path,
                                    snapshot_exists: None,
                                }
                            }
                        }
                        SecretManager::LedgerNano(ledger_nano) => LinkedSecretManager::LedgerNano {
                            is_connected: ledger_nano.get_ledger_nano_status().await.connected(),
                        },
                        _ => panic!("only Stronghold and LedgerNano supported at the moment."),
                    };
                    Some((wallet, linked_secret_manager))
                }
                Err(e) => {
                    println_log_error!("failed to load wallet db from storage: {e}");
                    return Ok((None, None));
                }
            }
        } else {
            None
        }
    };

    let (wallet, account_id) = if let Some(command) = cli.command {
        match command {
            WalletCommand::Init(init_params) => {
                if wallet_and_secret_manager.is_some() {
                    return Err(Error::Miscellaneous(format!(
                        "cannot initialize: wallet db at '{}' already exists",
                        storage_path.display()
                    )));
                }
                let secret_manager = create_secret_manager(&init_params).await?;
                let secret_manager_variant = secret_manager.to_string();
                let wallet = init_command(storage_path, secret_manager, init_params).await?;
                println_log_info!("Created new wallet with '{}' secret manager.", secret_manager_variant);
                (Some(wallet), None)
            }
            WalletCommand::Accounts => {
                if let Some((wallet, _)) = wallet_and_secret_manager {
                    accounts_command(&wallet).await?;
                    return Ok((None, None));
                } else {
                    return Err(Error::Miscellaneous(format!(
                        "wallet db does not exist at '{}'",
                        storage_path.display()
                    )));
                }
            }
            WalletCommand::Backup { backup_path } => {
                if let Some((wallet, secret_manager)) = wallet_and_secret_manager {
                    match secret_manager {
                        LinkedSecretManager::Stronghold {
                            snapshot_exists: Some(password),
                            ..
                        } => {
                            backup_command_stronghold(&wallet, &password, Path::new(&backup_path)).await?;
                            return Ok((None, None));
                        }
                        LinkedSecretManager::Stronghold { snapshot_path, .. } => {
                            return Err(Error::Miscellaneous(format!(
                                "Stronghold snapshot does not exist at '{}'",
                                snapshot_path.display()
                            )));
                        }
                        _ => {
                            println_log_info!("only Stronghold backup supported");
                            return Ok((None, None));
                        }
                    }
                } else {
                    return Err(Error::Miscellaneous(format!(
                        "wallet db does not exist at '{}'",
                        storage_path.display()
                    )));
                }
            }
            WalletCommand::ChangePassword => {
                if let Some((wallet, secret_manager)) = wallet_and_secret_manager {
                    match secret_manager {
                        LinkedSecretManager::Stronghold {
                            snapshot_exists: Some(password),
                            ..
                        } => {
                            change_password_command(&wallet, password).await?;
                            (Some(wallet), None)
                        }
                        LinkedSecretManager::Stronghold { snapshot_path, .. } => {
                            return Err(Error::Miscellaneous(format!(
                                "Stronghold snapshot does not exist at '{}'",
                                snapshot_path.display()
                            )));
                        }
                        _ => {
                            println_log_info!("only Stronghold password change supported");
                            return Ok((None, None));
                        }
                    }
                } else {
                    return Err(Error::Miscellaneous(format!(
                        "wallet db does not exist at '{}'",
                        storage_path.display()
                    )));
                }
            }
            WalletCommand::NewAccount { alias } => {
                if let Some((wallet, _)) = wallet_and_secret_manager {
                    let account = new_account_command(&wallet, alias).await?;
                    (Some(wallet), Some(account))
                } else {
                    return Err(Error::Miscellaneous(format!(
                        "wallet db does not exist at '{}'",
                        storage_path.display()
                    )));
                }
            }
            WalletCommand::SetNodeUrl { url } => {
                if let Some((wallet, _)) = wallet_and_secret_manager {
                    set_node_url_command(&wallet, url).await?;
                    (Some(wallet), None)
                } else {
                    return Err(Error::Miscellaneous(format!(
                        "wallet db does not exist at '{}'",
                        storage_path.display()
                    )));
                }
            }
            WalletCommand::SetPow {
                local_pow,
                worker_count,
            } => {
                if let Some((wallet, _)) = wallet_and_secret_manager {
                    set_pow_command(&wallet, local_pow, worker_count).await?;
                    (Some(wallet), None)
                } else {
                    return Err(Error::Miscellaneous(format!(
                        "wallet db does not exist at '{}'",
                        storage_path.display()
                    )));
                }
            }
            WalletCommand::Sync => {
                if let Some((wallet, _)) = wallet_and_secret_manager {
                    sync_command(&wallet).await?;
                    (Some(wallet), None)
                } else {
                    return Err(Error::Miscellaneous(format!(
                        "wallet db does not exist at '{}'",
                        storage_path.display()
                    )));
                }
            }
            WalletCommand::NodeInfo => {
                if let Some((wallet, _)) = wallet_and_secret_manager {
                    node_info_command(&wallet).await?;
                    return Ok((None, None));
                } else {
                    return Err(Error::Miscellaneous(format!(
                        "wallet db does not exist at '{}'",
                        storage_path.display()
                    )));
                }
            }
            WalletCommand::Restore { backup_path } => {
                if let Some((wallet, linked_secret_manager)) = wallet_and_secret_manager {
                    match linked_secret_manager {
                        LinkedSecretManager::Stronghold {
                            snapshot_path,
                            snapshot_exists,
                        } => {
                            // we need to explicitly drop the current wallet here to prevent:
                            // "error accessing storage: IO error: lock hold by current process"
                            drop(wallet);
                            let wallet = restore_command_stronghold(
                                storage_path,
                                snapshot_exists,
                                snapshot_path.as_path(),
                                Path::new(&backup_path),
                            )
                            .await?;
                            (Some(wallet), None)
                        }
                        _ => {
                            println_log_info!("only Stronghold restore supported at the moment");
                            return Ok((None, None));
                        }
                    }
                } else {
                    // the wallet db does not exist
                    let init_params = InitParameters::default();
                    let snapshot_path = Path::new(&init_params.stronghold_snapshot_path);
                    let wallet =
                        restore_command_stronghold(storage_path, None, snapshot_path, Path::new(&backup_path)).await?;
                    (Some(wallet), None)
                }
            }
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
        }
    } else {
        // no wallet command provided
        if let Some((wallet, linked_secret_manager)) = wallet_and_secret_manager {
            match linked_secret_manager {
                LinkedSecretManager::Stronghold {
                    snapshot_exists: None,
                    snapshot_path,
                } => {
                    println_log_error!(
                        "Snapshot file for Stronghold secret manager linked with the wallet not found at '{}'",
                        snapshot_path.display()
                    );
                    return Ok((None, None));
                }
                LinkedSecretManager::LedgerNano { is_connected: false } => {
                    println_log_error!("Ledger Nano linked with the wallet not connected");
                    return Ok((None, None));
                }
                _ => {}
            }

            if wallet.get_accounts().await?.is_empty() {
                create_initial_account(wallet).await?
            } else if let Some(alias) = cli.account {
                (Some(wallet), Some(alias))
            } else if let Some(account) = pick_account(&wallet).await? {
                (Some(wallet), Some(account.alias().await.into()))
            } else {
                (Some(wallet), None)
            }
        } else {
            // init new wallet with default init parameters
            let init_params = InitParameters::default();
            let snapshot_path = Path::new(&init_params.stronghold_snapshot_path);
            if !snapshot_path.exists() {
                if get_decision("Create a new wallet with default parameters?")? {
                    let secret_manager = create_secret_manager(&init_params).await?;
                    let secret_manager_variant = secret_manager.to_string();
                    let wallet = init_command(storage_path, secret_manager, init_params).await?;
                    println_log_info!("Created new wallet with '{}' secret manager.", secret_manager_variant);
                    create_initial_account(wallet).await?
                } else {
                    WalletCli::print_help()?;
                    (None, None)
                }
            } else {
                println_log_error!(
                    "Inconsistent wallet: Stronghold snapshot found at '{}', but no Wallet database at '{}'.",
                    snapshot_path.display(),
                    storage_path.display()
                );
                (None, None)
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

async fn create_secret_manager(init_params: &InitParameters) -> Result<SecretManager, Error> {
    Ok(match select_secret_manager().await? {
        SecretManagerChoice::Stronghold => {
            let snapshot_path = Path::new(&init_params.stronghold_snapshot_path);

            if snapshot_path.exists() {
                return Err(Error::Miscellaneous(format!(
                    "cannot initialize: {} already exists",
                    snapshot_path.display()
                )));
            }

            let password = get_password("Stronghold password", true)?;
            let mnemonic = match &init_params.mnemonic_file_path {
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
        SecretManagerChoice::LedgerNanoSimulator => SecretManager::LedgerNano(LedgerSecretManager::new(true)),
    })
}
