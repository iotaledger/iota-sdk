// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{path::Path, str::FromStr};

use clap::{builder::BoolishValueParser, Args, CommandFactory, Parser, Subcommand};
use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
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
        import_mnemonic,
    },
    println_log_error, println_log_info,
};

const DEFAULT_LOG_LEVEL: &str = "debug";
const DEFAULT_NODE_URL: &str = "https://api.testnet.shimmer.network";
const DEFAULT_STRONGHOLD_SNAPSHOT_PATH: &str = "./stardust-cli-wallet.stronghold";
const DEFAULT_WALLET_DATABASE_PATH: &str = "./stardust-cli-wallet-db";

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    /// Set the path to the wallet database.
    #[arg(long, value_name = "PATH", env = "WALLET_DATABASE_PATH", default_value = DEFAULT_WALLET_DATABASE_PATH)]
    pub wallet_db_path: String,
    /// Set the path to the stronghold snapshot file.
    #[arg(long, value_name = "PATH", env = "STRONGHOLD_SNAPSHOT_PATH", default_value = DEFAULT_STRONGHOLD_SNAPSHOT_PATH)]
    pub stronghold_snapshot_path: String,
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
    /// Set the path to a file containing mnemonics. If empty, a mnemonic has to be entered or will be randomly
    /// generated.
    #[arg(short, long, value_name = "PATH")]
    pub mnemonic_file_path: Option<String>,
    /// Set the node to connect to with this wallet.
    #[arg(short, long, value_name = "URL", env = "NODE_URL", default_value = DEFAULT_NODE_URL)]
    pub node_url: String,
    /// Set the BIP path, `4219/0/0/0` if not provided.
    #[arg(short, long, value_parser = parse_bip_path)]
    pub bip_path: Option<Bip44>,
    /// Set the Bech32-encoded wallet address.
    #[arg(short, long)]
    pub address: Option<String>,
}

impl Default for InitParameters {
    fn default() -> Self {
        Self {
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
    /// Create a stronghold backup file.
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
    /// Restore a stronghold backup file.
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
    let snapshot_path = Path::new(&cli.stronghold_snapshot_path);

    Ok(if let Some(command) = cli.command {
        match command {
            CliCommand::Backup { backup_path } => {
                backup_command(storage_path, snapshot_path, std::path::Path::new(&backup_path)).await?;
                None
            }
            CliCommand::ChangePassword => {
                let wallet = change_password_command(storage_path, snapshot_path).await?;
                Some(wallet)
            }
            CliCommand::Init(init_parameters) => {
                let wallet = init_command(storage_path, snapshot_path, init_parameters).await?;
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
                node_info_command(storage_path).await?;
                None
            }
            CliCommand::Restore { backup_path } => {
                let wallet = restore_command(storage_path, snapshot_path, std::path::Path::new(&backup_path)).await?;
                Some(wallet)
            }
            CliCommand::SetNodeUrl { url } => {
                let wallet = set_node_url_command(storage_path, snapshot_path, url).await?;
                Some(wallet)
            }
            CliCommand::Sync => {
                let wallet = sync_command(storage_path, snapshot_path).await?;
                Some(wallet)
            }
        }
    } else {
        // no command provided, i.e. `> ./wallet`
        match (storage_path.exists(), snapshot_path.exists()) {
            (true, true) => {
                let password = get_password("Stronghold password", false)?;
                let wallet = unlock_wallet(storage_path, snapshot_path, password).await?;
                Some(wallet)
            }
            (false, false) => {
                if get_decision("Create a new wallet with default parameters?")? {
                    let wallet = init_command(storage_path, snapshot_path, InitParameters::default()).await?;
                    println_log_info!("Created new wallet.");
                    Some(wallet)
                } else {
                    Cli::print_help()?;
                    None
                }
            }
            (true, false) => {
                println_log_error!("Stronghold snapshot not found at '{}'.", snapshot_path.display());
                None
            }
            (false, true) => {
                println_log_error!("Wallet database not found at '{}'.", storage_path.display());
                None
            }
        }
    })
}

pub async fn backup_command(storage_path: &Path, snapshot_path: &Path, backup_path: &Path) -> Result<(), Error> {
    let password = get_password("Stronghold password", !snapshot_path.exists())?;
    let wallet = unlock_wallet(storage_path, snapshot_path, password.clone()).await?;
    wallet.backup(backup_path.into(), password).await?;

    println_log_info!("Wallet has been backed up to \"{}\".", backup_path.display());

    Ok(())
}

pub async fn change_password_command(storage_path: &Path, snapshot_path: &Path) -> Result<Wallet, Error> {
    let password = get_password("Stronghold password", !snapshot_path.exists())?;
    let wallet = unlock_wallet(storage_path, snapshot_path, password.clone()).await?;
    let new_password = get_password("Stronghold new password", true)?;
    wallet.change_stronghold_password(password, new_password).await?;

    println_log_info!("The password has been changed");

    Ok(wallet)
}

pub async fn init_command(
    storage_path: &Path,
    snapshot_path: &Path,
    init_params: InitParameters,
) -> Result<Wallet, Error> {
    if storage_path.exists() {
        return Err(Error::Miscellaneous(format!(
            "cannot initialize: {} already exists",
            storage_path.display()
        )));
    }
    if snapshot_path.exists() {
        return Err(Error::Miscellaneous(format!(
            "cannot initialize: {} already exists",
            snapshot_path.display()
        )));
    }
    let password = get_password("Stronghold password", true)?;
    let mnemonic = match init_params.mnemonic_file_path {
        Some(path) => import_mnemonic(&path).await?,
        None => enter_or_generate_mnemonic().await?,
    };

    let secret_manager = StrongholdSecretManager::builder()
        .password(password)
        .build(snapshot_path)?;
    secret_manager.store_mnemonic(mnemonic).await?;
    let secret_manager = SecretManager::Stronghold(secret_manager);

    let alias = if get_decision("Do you want to assign an alias to your wallet?")? {
        Some(get_alias("New wallet alias").await?)
    } else {
        None
    };

    let address = init_params
        .address
        .map(|addr| Bech32Address::from_str(&addr))
        .transpose()?;

    Ok(Wallet::builder()
        .with_secret_manager(secret_manager)
        .with_client_options(ClientOptions::new().with_node(init_params.node_url.as_str())?)
        .with_storage_path(storage_path.to_str().expect("invalid unicode"))
        .with_bip_path(init_params.bip_path)
        .with_address(address)
        .with_alias(alias.as_ref().map(|alias| alias.as_str()))
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

pub async fn node_info_command(storage_path: &Path) -> Result<Wallet, Error> {
    let wallet = unlock_wallet(storage_path, None, None).await?;
    let node_info = wallet.client().get_info().await?;

    println_log_info!("Current node info: {}", serde_json::to_string_pretty(&node_info)?);

    Ok(wallet)
}

pub async fn restore_command(storage_path: &Path, snapshot_path: &Path, backup_path: &Path) -> Result<Wallet, Error> {
    check_file_exists(backup_path).await?;

    let mut builder = Wallet::builder();
    if check_file_exists(snapshot_path).await.is_ok() {
        println!(
            "Detected a stronghold file at {}. Enter password to unlock:",
            snapshot_path.to_str().unwrap()
        );
        let password = get_password("Stronghold password", false)?;
        let secret_manager = SecretManager::Stronghold(
            StrongholdSecretManager::builder()
                .password(password.clone())
                .build(snapshot_path)?,
        );
        builder = builder.with_secret_manager(secret_manager);
    }

    let wallet = builder
        // Will be overwritten by the backup's value.
        .with_client_options(ClientOptions::new().with_node(DEFAULT_NODE_URL)?)
        .with_storage_path(storage_path.to_str().expect("invalid unicode"))
        // Will be overwritten by the backup's value.
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .finish()
        .await?;

    let password = get_password("Stronghold backup password", false)?;
    wallet.restore_backup(backup_path.into(), password, None, None).await?;

    println_log_info!(
        "Wallet has been restored from the backup file \"{}\".",
        backup_path.display()
    );

    Ok(wallet)
}

pub async fn set_node_url_command(storage_path: &Path, snapshot_path: &Path, url: String) -> Result<Wallet, Error> {
    let password = get_password("Stronghold password", !snapshot_path.exists())?;
    let wallet = unlock_wallet(storage_path, snapshot_path, password).await?;
    wallet.set_client_options(ClientOptions::new().with_node(&url)?).await?;

    Ok(wallet)
}

pub async fn sync_command(storage_path: &Path, snapshot_path: &Path) -> Result<Wallet, Error> {
    let password = get_password("Stronghold password", !snapshot_path.exists())?;
    let wallet = unlock_wallet(storage_path, snapshot_path, password).await?;
    let total_balance = wallet.sync(None).await?;

    println_log_info!("Synchronized wallet: {:?}", total_balance);

    Ok(wallet)
}

pub async fn unlock_wallet(
    storage_path: &Path,
    snapshot_path: impl Into<Option<&Path>>,
    password: impl Into<Option<Password>>,
) -> Result<Wallet, Error> {
    let secret_manager = if let Some(password) = password.into() {
        let snapshot_path = snapshot_path.into();
        Some(SecretManager::Stronghold(
            StrongholdSecretManager::builder()
                .password(password)
                .build(snapshot_path.ok_or(Error::Miscellaneous("Snapshot file path is not given".to_string()))?)?,
        ))
    } else {
        None
    };

    let maybe_wallet = Wallet::builder()
        .with_secret_manager(secret_manager)
        .with_storage_path(storage_path.to_str().expect("invalid unicode"))
        .finish()
        .await;

    if let Err(iota_sdk::wallet::Error::MissingParameter(_)) = maybe_wallet {
        println_log_error!("Please make sure the wallet is initialized.");
    }

    Ok(maybe_wallet?)
}
