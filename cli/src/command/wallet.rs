// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use clap::{Args, Parser, Subcommand};
use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
        stronghold::StrongholdAdapter,
        utils::Password,
    },
    wallet::{ClientOptions, Wallet},
};
use log::LevelFilter;

use crate::{
    error::Error,
    helper::{check_file_exists, enter_or_generate_mnemonic, generate_mnemonic, get_password, import_mnemonic},
    println_log_error, println_log_info,
};

const DEFAULT_LOG_LEVEL: &str = "debug";
const DEFAULT_NODE_URL: &str = "https://api.testnet.shimmer.network";
const DEFAULT_STRONGHOLD_SNAPSHOT_PATH: &str = "./stardust-cli-wallet.stronghold";
const DEFAULT_WALLET_DATABASE_PATH: &str = "./stardust-cli-wallet-db";

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct WalletCli {
    /// Set the path to the wallet database.
    #[arg(long, value_name = "PATH", env = "WALLET_DATABASE_PATH", default_value = DEFAULT_WALLET_DATABASE_PATH)]
    pub wallet_db_path: String,
    /// Set the path to the stronghold snapshot file.
    #[arg(long, value_name = "PATH", env = "STRONGHOLD_SNAPSHOT_PATH", default_value = DEFAULT_STRONGHOLD_SNAPSHOT_PATH)]
    pub stronghold_snapshot_path: String,
    /// Set the account to enter.
    pub account: Option<String>,
    /// Set the log level.
    #[arg(short, long, default_value = DEFAULT_LOG_LEVEL)]
    pub log_level: LevelFilter,
    #[command(subcommand)]
    pub command: Option<WalletCommand>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum WalletCommand {
    /// List all accounts.
    Accounts,
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
    Mnemonic,
    /// Create a new account.
    NewAccount {
        /// Account alias, next available account index if not provided.
        alias: Option<String>,
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
    /// Synchronize all accounts.
    Sync,
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
    /// Coin type, SHIMMER_COIN_TYPE (4219) if not provided.
    #[arg(short, long, default_value_t = SHIMMER_COIN_TYPE)]
    pub coin_type: u32,
}

impl Default for InitParameters {
    fn default() -> Self {
        Self {
            mnemonic_file_path: None,
            node_url: DEFAULT_NODE_URL.to_string(),
            coin_type: SHIMMER_COIN_TYPE,
        }
    }
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
    parameters: InitParameters,
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
    let mnemonic = match parameters.mnemonic_file_path {
        Some(path) => import_mnemonic(&path).await?,
        None => enter_or_generate_mnemonic().await?,
    };

    let secret_manager = StrongholdSecretManager::builder()
        .password(password)
        .build(snapshot_path)?;
    secret_manager.store_mnemonic(mnemonic).await?;
    let secret_manager = SecretManager::Stronghold(secret_manager);

    Ok(Wallet::builder()
        .with_secret_manager(secret_manager)
        .with_client_options(ClientOptions::new().with_node(parameters.node_url.as_str())?)
        .with_storage_path(storage_path.to_str().expect("invalid unicode"))
        .with_coin_type(parameters.coin_type)
        .finish()
        .await?)
}

pub async fn list_accounts_command(storage_path: &Path, snapshot_path: &Path) -> Result<(), Error> {
    let password = get_password("Stronghold password", false)?;
    let wallet = unlock_wallet(storage_path, snapshot_path, password).await?;

    let accounts = wallet.get_account_aliases().await?;
    println!("{accounts:?}");

    Ok(())
}

pub async fn migrate_stronghold_snapshot_v2_to_v3_command(path: Option<String>) -> Result<(), Error> {
    let snapshot_path = path.as_deref().unwrap_or(DEFAULT_STRONGHOLD_SNAPSHOT_PATH);
    check_file_exists(snapshot_path.as_ref()).await?;

    let password = get_password("Stronghold password", false)?;
    StrongholdAdapter::migrate_snapshot_v2_to_v3(snapshot_path, password, "wallet.rs", 100, None, None)?;

    println_log_info!("Stronghold snapshot successfully migrated from v2 to v3.");

    Ok(())
}

pub async fn mnemonic_command() -> Result<(), Error> {
    generate_mnemonic().await?;

    Ok(())
}

pub async fn new_account_command(
    storage_path: &Path,
    snapshot_path: &Path,
    alias: Option<String>,
) -> Result<(Wallet, String), Error> {
    let password = get_password("Stronghold password", !snapshot_path.exists())?;
    let wallet = unlock_wallet(storage_path, snapshot_path, password).await?;

    let alias = add_account(&wallet, alias).await?;

    Ok((wallet, alias))
}

pub async fn node_info_command(storage_path: &Path) -> Result<Wallet, Error> {
    let wallet = unlock_wallet(storage_path, None, None).await?;
    let node_info = wallet.client().get_info().await?;

    println_log_info!("Current node info: {}", serde_json::to_string_pretty(&node_info)?);

    Ok(wallet)
}

pub async fn restore_command(storage_path: &Path, snapshot_path: &Path, backup_path: &Path) -> Result<Wallet, Error> {
    check_file_exists(backup_path).await?;

    let password = get_password("Stronghold password", false)?;
    let secret_manager = SecretManager::Stronghold(
        StrongholdSecretManager::builder()
            .password(password.clone())
            .build(snapshot_path)?,
    );
    let wallet = Wallet::builder()
        .with_secret_manager(secret_manager)
        // Will be overwritten by the backup's value.
        .with_client_options(ClientOptions::new().with_node(DEFAULT_NODE_URL)?)
        .with_storage_path(storage_path.to_str().expect("invalid unicode"))
        // Will be overwritten by the backup's value.
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

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

    println_log_info!("Synchronized all accounts: {:?}", total_balance);

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

pub async fn add_account(wallet: &Wallet, alias: Option<String>) -> Result<String, Error> {
    let mut account_builder = wallet.create_account();

    if let Some(alias) = alias {
        account_builder = account_builder.with_alias(alias);
    }

    let account = account_builder.finish().await?;
    let alias = account.details().await.alias().to_string();

    println_log_info!("Created account \"{alias}\"");

    Ok(alias)
}
