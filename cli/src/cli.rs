// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{path::Path, str::FromStr};

use clap::{builder::BoolishValueParser, Args, CommandFactory, Parser, Subcommand};
use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{ledger_nano::LedgerSecretManager, stronghold::StrongholdSecretManager, SecretManager},
        stronghold::StrongholdAdapter,
        utils::Password,
    },
    crypto::keys::bip44::Bip44,
    types::block::address::Bech32Address,
    wallet::{ClientOptions, Wallet},
};
use log::LevelFilter;

use crate::{
    error::Error,
    helper::{
        check_file_exists, enter_or_generate_mnemonic, generate_mnemonic, get_alias, get_decision, get_password,
        import_mnemonic, select_secret_manager, SecretManagerChoice,
    },
    println_log_error, println_log_info,
};

const DEFAULT_LOG_LEVEL: &str = "debug";
const DEFAULT_NODE_URL: &str = "http://localhost:8050";
const DEFAULT_STRONGHOLD_SNAPSHOT_PATH: &str = "./stardust-cli-wallet.stronghold";
const DEFAULT_WALLET_DATABASE_PATH: &str = "./stardust-cli-wallet-db";

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    /// Set the path to the wallet database.
    #[arg(long, value_name = "PATH", env = "WALLET_DATABASE_PATH", default_value = DEFAULT_WALLET_DATABASE_PATH)]
    pub wallet_db_path: String,
    /// Set the log level.
    #[arg(short, long, default_value = DEFAULT_LOG_LEVEL)]
    pub log_level: LevelFilter,
    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

impl Cli {
    pub fn print_help() -> Result<(), Error> {
        Self::command().bin_name("wallet").print_help()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Args)]
pub struct InitParameters {
    /// Set the secret manager to use.
    #[arg(short, long, value_name = "SECRET_MANAGER")]
    pub secret_manager: Option<SecretManagerChoice>,
    /// Set the path to the stronghold snapshot file. Ignored if the <SECRET_MANAGER> is not a Stronghold secret
    /// manager.
    #[arg(short = 't', long, value_name = "PATH", env = "STRONGHOLD_SNAPSHOT_PATH", default_value = DEFAULT_STRONGHOLD_SNAPSHOT_PATH)]
    pub stronghold_snapshot_path: String,
    /// Set the path to a file containing mnemonics. If empty, a mnemonic has to be entered or will be randomly
    /// generated. Only used by some secret managers.
    #[arg(short, long, value_name = "PATH")]
    pub mnemonic_file_path: Option<String>,
    /// Set the node to connect to with this wallet.
    #[arg(short, long, value_name = "URL", env = "NODE_URL", default_value = DEFAULT_NODE_URL)]
    pub node_url: String,
    /// Set the BIP path, `4219/0/0/0` if not provided.
    #[arg(short, long, value_parser = parse_bip_path, default_value = "4219/0/0/0")]
    pub bip_path: Option<Bip44>,
    /// Set the Bech32-encoded wallet address.
    #[arg(short, long)]
    pub address: Option<String>,
}

impl Default for InitParameters {
    fn default() -> Self {
        Self {
            secret_manager: Some(SecretManagerChoice::Stronghold),
            stronghold_snapshot_path: DEFAULT_STRONGHOLD_SNAPSHOT_PATH.to_string(),
            mnemonic_file_path: None,
            node_url: DEFAULT_NODE_URL.to_string(),
            bip_path: Some(Bip44::new(SHIMMER_COIN_TYPE)),
            address: None,
        }
    }
}

fn parse_bip_path(arg: &str) -> Result<Bip44, String> {
    let mut bip_path_enc = Vec::with_capacity(4);
    for p in arg.split_terminator('/').map(|p| p.trim()) {
        match p.parse::<u32>() {
            Ok(value) => bip_path_enc.push(value),
            Err(_) => {
                return Err(format!("cannot parse BIP path: {p}"));
            }
        }
    }

    if bip_path_enc.len() != 4 {
        return Err(
            "invalid BIP path format. Expected: `coin_type/account_index/change_address/address_index`".to_string(),
        );
    }

    let bip_path = Bip44::new(bip_path_enc[0])
        .with_account(bip_path_enc[1])
        .with_change(bip_path_enc[2])
        .with_address_index(bip_path_enc[3]);

    Ok(bip_path)
}

#[derive(Debug, Clone, Subcommand)]
pub enum CliCommand {
    /// Create a backup file. Currently only Stronghold backup supported.
    Backup {
        /// Path of the created stronghold backup file.
        backup_path: String,
    },
    /// Change the stronghold password.
    ChangePassword,
    /// Initialize the wallet.
    Init(InitParameters),
    /// Migrate a stronghold snapshot v2 to v3.
    MigrateStrongholdSnapshotV2ToV3 {
        /// Path of the to be migrated stronghold file. "./stardust-cli-wallet.stronghold" if nothing provided.
        path: Option<String>,
    },
    /// Generate a random mnemonic.
    Mnemonic {
        // Output the mnemonic to the specified file.
        #[arg(long)]
        output_file_name: Option<String>,
        // Output the mnemonic to the stdout.
        #[arg(long, num_args = 0..=1, default_missing_value = Some("true"), value_parser = BoolishValueParser::new())]
        output_stdout: Option<bool>,
    },
    /// Get information about currently set node.
    NodeInfo,
    /// Restore a backup file. Currently only Stronghold backup supported.
    Restore {
        /// Path of the to be restored stronghold backup file.
        backup_path: String,
    },
    /// Set the URL of the node to use.
    SetNodeUrl {
        /// Node URL to use for all future operations.
        url: String,
    },
    /// Synchronize wallet.
    Sync,
}

pub async fn new_wallet(cli: Cli) -> Result<Option<Wallet>, Error> {
    let storage_path = Path::new(&cli.wallet_db_path);

    enum LinkedSecretManager {
        Stronghold {
            snapshot_path: std::path::PathBuf,
            snapshot_exists: bool,
        },
        LedgerNano,
    }

    let wallet_and_secret_manager = {
        if storage_path.is_dir() {
            match Wallet::builder().with_storage_path(storage_path).finish().await {
                Ok(wallet) => {
                    let linked_secret_manager = match &mut *wallet.get_secret_manager().write().await {
                        SecretManager::Stronghold(stronghold) => {
                            let snapshot_path = stronghold.snapshot_path().to_path_buf();
                            let snapshot_exists = snapshot_path.exists();
                            LinkedSecretManager::Stronghold {
                                snapshot_path,
                                snapshot_exists,
                            }
                        }
                        SecretManager::LedgerNano(_) => LinkedSecretManager::LedgerNano,
                        _ => panic!("only Stronghold and LedgerNano supported at the moment."),
                    };
                    Some((wallet, linked_secret_manager))
                }
                Err(e) => {
                    println_log_error!("failed to load wallet db from storage: {e}");
                    return Ok(None);
                }
            }
        } else {
            None
        }
    };

    Ok(if let Some(command) = cli.command {
        match command {
            CliCommand::Backup { backup_path } => {
                if let Some((wallet, secret_manager)) = wallet_and_secret_manager {
                    match secret_manager {
                        LinkedSecretManager::Stronghold {
                            snapshot_exists: true, ..
                        } => {
                            let password = get_password("Stronghold password", false)?;
                            backup_to_stronghold_snapshot_command(&wallet, &password, Path::new(&backup_path)).await?;
                            return Ok(None);
                        }
                        LinkedSecretManager::Stronghold { snapshot_path, .. } => {
                            return Err(Error::Miscellaneous(format!(
                                "Stronghold snapshot does not exist at '{}'",
                                snapshot_path.display()
                            )));
                        }
                        _ => {
                            println_log_info!("only Stronghold backup supported");
                            return Ok(None);
                        }
                    }
                } else {
                    return Err(Error::Miscellaneous(format!(
                        "wallet db does not exist at '{}'",
                        storage_path.display()
                    )));
                }
            }
            CliCommand::ChangePassword => {
                if let Some((wallet, secret_manager)) = wallet_and_secret_manager {
                    match secret_manager {
                        LinkedSecretManager::Stronghold {
                            snapshot_exists: true, ..
                        } => {
                            let current_password = get_password("Stronghold password", false)?;
                            change_password_command(&wallet, current_password).await?;
                            Some(wallet)
                        }
                        LinkedSecretManager::Stronghold { snapshot_path, .. } => {
                            return Err(Error::Miscellaneous(format!(
                                "Stronghold snapshot does not exist at '{}'",
                                snapshot_path.display()
                            )));
                        }
                        _ => {
                            println_log_info!("only Stronghold password change supported");
                            return Ok(None);
                        }
                    }
                } else {
                    return Err(Error::Miscellaneous(format!(
                        "wallet db does not exist at '{}'",
                        storage_path.display()
                    )));
                }
            }
            CliCommand::Init(init_parameters) => {
                if wallet_and_secret_manager.is_some() {
                    return Err(Error::Miscellaneous(format!(
                        "cannot initialize: wallet db at '{}' already exists",
                        storage_path.display()
                    )));
                }
                let secret_manager = create_secret_manager(&init_parameters).await?;
                let secret_manager_variant = secret_manager.to_string();
                let wallet = init_command(storage_path, secret_manager, init_parameters).await?;
                println_log_info!("Created new wallet with '{}' secret manager.", secret_manager_variant);

                Some(wallet)
            }
            CliCommand::MigrateStrongholdSnapshotV2ToV3 { path } => {
                migrate_stronghold_snapshot_v2_to_v3_command(path).await?;
                None
            }
            CliCommand::Mnemonic {
                output_file_name,
                output_stdout,
            } => {
                mnemonic_command(output_file_name, output_stdout).await?;
                None
            }
            CliCommand::NodeInfo => {
                if let Some((wallet, _)) = wallet_and_secret_manager {
                    node_info_command(&wallet).await?;
                    return Ok(None);
                } else {
                    return Err(Error::Miscellaneous(format!(
                        "wallet db does not exist at '{}'",
                        storage_path.display()
                    )));
                }
            }
            CliCommand::Restore { backup_path } => {
                if let Some((wallet, linked_secret_manager)) = wallet_and_secret_manager {
                    match linked_secret_manager {
                        LinkedSecretManager::Stronghold { snapshot_path, .. } => {
                            // we need to explicitly drop the current wallet here to prevent:
                            // "error accessing storage: IO error: lock hold by current process"
                            drop(wallet);
                            let wallet = restore_from_stronghold_snapshot_command(
                                storage_path,
                                snapshot_path.as_path(),
                                Path::new(&backup_path),
                            )
                            .await?;
                            Some(wallet)
                        }
                        _ => {
                            println_log_info!("only Stronghold restore supported at the moment");
                            return Ok(None);
                        }
                    }
                } else {
                    // the wallet db does not exist
                    let init_params = InitParameters::default();
                    let snapshot_path = Path::new(&init_params.stronghold_snapshot_path);
                    let wallet =
                        restore_from_stronghold_snapshot_command(storage_path, snapshot_path, Path::new(&backup_path))
                            .await?;
                    Some(wallet)
                }
            }
            CliCommand::SetNodeUrl { url } => {
                if let Some((wallet, _)) = wallet_and_secret_manager {
                    set_node_url_command(&wallet, url).await?;
                    Some(wallet)
                } else {
                    return Err(Error::Miscellaneous(format!(
                        "wallet db does not exist at '{}'",
                        storage_path.display()
                    )));
                }
            }
            CliCommand::Sync => {
                if let Some((wallet, _)) = wallet_and_secret_manager {
                    sync_command(&wallet).await?;
                    Some(wallet)
                } else {
                    return Err(Error::Miscellaneous(format!(
                        "wallet db does not exist at '{}'",
                        storage_path.display()
                    )));
                }
            }
        }
    } else {
        // no wallet command provided
        if let Some((wallet, linked_secret_manager)) = wallet_and_secret_manager {
            if let LinkedSecretManager::Stronghold {
                snapshot_exists: false,
                snapshot_path,
            } = linked_secret_manager
            {
                println_log_error!(
                    "Snapshot file for Stronghold secret manager linked with the wallet not found at '{}'",
                    snapshot_path.display()
                );
                return Ok(None);
            }

            Some(wallet)
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
                    Some(wallet)
                } else {
                    Cli::print_help()?;
                    None
                }
            } else {
                println_log_error!(
                    "Inconsistent wallet: Stronghold snapshot found at '{}', but no Wallet database at '{}'.",
                    snapshot_path.display(),
                    storage_path.display()
                );
                None
            }
        }
    })
}

pub async fn backup_to_stronghold_snapshot_command(
    wallet: &Wallet,
    password: &Password,
    backup_path: &Path,
) -> Result<(), Error> {
    wallet
        .backup_to_stronghold_snapshot(backup_path.into(), password.clone())
        .await?;

    println_log_info!("Wallet has been backed up to \"{}\".", backup_path.display());

    Ok(())
}

pub async fn change_password_command(wallet: &Wallet, current_password: Password) -> Result<(), Error> {
    let new_password = get_password("New Stronghold password", true)?;
    wallet
        .change_stronghold_password(current_password, new_password)
        .await?;

    println_log_info!("The password has been changed");

    Ok(())
}

pub async fn init_command(
    storage_path: &Path,
    secret_manager: SecretManager,
    init_params: InitParameters,
) -> Result<Wallet, Error> {
    let alias = if get_decision("Do you want to assign an alias to your wallet?")? {
        Some(get_alias("New wallet alias").await?)
    } else {
        None
    };

    Ok(Wallet::builder()
        .with_secret_manager(secret_manager)
        .with_client_options(ClientOptions::new().with_node(init_params.node_url.as_str())?)
        .with_storage_path(storage_path.to_str().expect("invalid unicode"))
        .with_address(
            init_params
                .address
                .map(|addr| Bech32Address::from_str(&addr))
                .transpose()?,
        )
        .with_bip_path(init_params.bip_path)
        .with_alias(alias)
        .finish()
        .await?)
}

pub async fn migrate_stronghold_snapshot_v2_to_v3_command(path: Option<String>) -> Result<(), Error> {
    let snapshot_path = path.as_deref().unwrap_or(DEFAULT_STRONGHOLD_SNAPSHOT_PATH);
    check_file_exists(snapshot_path.as_ref()).await?;

    let password = get_password("Stronghold password", false)?;
    StrongholdAdapter::migrate_snapshot_v2_to_v3(snapshot_path, password, "wallet.rs", 100, None, None)?;

    println_log_info!("Stronghold snapshot successfully migrated from v2 to v3.");

    Ok(())
}

pub async fn mnemonic_command(output_file_name: Option<String>, output_stdout: Option<bool>) -> Result<(), Error> {
    generate_mnemonic(output_file_name, output_stdout).await?;
    Ok(())
}

pub async fn node_info_command(wallet: &Wallet) -> Result<(), Error> {
    let node_info = serde_json::to_string_pretty(&wallet.client().get_info().await?)?;

    println_log_info!("Current node info: {node_info}");

    Ok(())
}

pub async fn restore_from_stronghold_snapshot_command(
    storage_path: &Path,
    snapshot_path: &Path,
    backup_path: &Path,
) -> Result<Wallet, Error> {
    check_file_exists(backup_path).await?;

    let mut builder = Wallet::builder();

    let password = if snapshot_path.exists() {
        Some(get_password("Stronghold password", false)?)
    } else {
        None
    };

    if let Some(password) = password {
        println!("Detected a stronghold file at {}.", snapshot_path.to_str().unwrap());
        let secret_manager = SecretManager::Stronghold(
            StrongholdSecretManager::builder()
                .password(password)
                .build(snapshot_path)?,
        );
        builder = builder.with_secret_manager(secret_manager);
    } else {
        // If there is no db, set the placeholder so the wallet builder doesn't fail.
        if check_file_exists(storage_path).await.is_err() {
            builder = builder.with_secret_manager(SecretManager::Placeholder);
        }
    }

    // If the restore fails we do not want to remove an already existing wallet
    let restore_into_existing_wallet = storage_path.is_dir();
    let wallet = builder
        // Will be overwritten by the backup's value.
        .with_client_options(ClientOptions::new().with_node(DEFAULT_NODE_URL)?)
        .with_storage_path(storage_path.to_str().expect("invalid unicode"))
        // Will be overwritten by the backup's value.
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .finish()
        .await?;

    let password = get_password("Stronghold backup password", false)?;
    if let Err(e) = wallet
        .restore_from_stronghold_snapshot(backup_path.into(), password, None, None)
        .await
    {
        // Clean up the file system after a failed restore (typically produces a wallet without a secret manager).
        // TODO: a better way would be to not create any files/dirs in the first place when it's not clear yet whether
        // the restore will be successful. https://github.com/iotaledger/iota-sdk/issues/2018
        if storage_path.is_dir() && !restore_into_existing_wallet {
            std::fs::remove_dir_all(storage_path)?;
        }
        Err(e.into())
    } else {
        println_log_info!(
            "Wallet has been restored from the backup file \"{}\".",
            backup_path.display()
        );
        Ok(wallet)
    }
}

pub async fn set_node_url_command(wallet: &Wallet, url: String) -> Result<(), Error> {
    wallet.set_client_options(ClientOptions::new().with_node(&url)?).await?;

    Ok(())
}

pub async fn sync_command(wallet: &Wallet) -> Result<(), Error> {
    let total_balance = wallet.sync(None).await?;

    println_log_info!("Synchronized wallet: {:?}", total_balance);

    Ok(())
}

async fn create_secret_manager(init_params: &InitParameters) -> Result<SecretManager, Error> {
    let choice = if let Some(choice) = &init_params.secret_manager {
        *choice
    } else {
        select_secret_manager().await?
    };

    Ok(match choice {
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
