// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use clap::{Args, Parser, Subcommand};
use iota_sdk::{
    client::{constants::SHIMMER_COIN_TYPE, secret::SecretManager, stronghold::StrongholdAdapter},
    wallet::{ClientOptions, Wallet},
};
use log::LevelFilter;
use zeroize::Zeroize;

use crate::{
    error::Error,
    helper::{enter_or_generate_mnemonic, generate_mnemonic, get_password, import_mnemonic},
    println_log_info,
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
    /// Create a stronghold backup file.
    Backup {
        /// Path of the created stronghold backup file.
        backup_path: String,
    },
    /// Change the stronghold password.
    ChangePassword,
    /// Initialize the wallet.
    Init(InitParameters),
    /// Migrate a stronghold v2 snapshot to v3.
    MigrateStronghold {
        /// Path of the to be migrated stronghold file. "./stardust-cli-wallet.stronghold" if nothing provided.
        path: Option<String>,
    },
    /// Generate a random mnemonic.
    Mnemonic,
    /// Create a new account.
    New {
        /// Account alias, next available account index if not provided.
        alias: Option<String>,
    },
    /// Restore a stronghold backup file.
    Restore {
        /// Path of the to be restored stronghold backup file.
        backup_path: String,
    },
    /// Set the node to use.
    SetNode {
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

pub async fn backup_command(wallet: &Wallet, path: String, password: &str) -> Result<(), Error> {
    wallet.backup(path.clone().into(), password.into()).await?;

    println_log_info!("Wallet has been backed up to \"{path}\".");

    Ok(())
}

pub async fn change_password_command(wallet: &Wallet, current: &str) -> Result<(), Error> {
    let mut new = get_password("Stronghold new password", true)?;

    wallet.change_stronghold_password(current, &new).await?;
    new.zeroize();

    Ok(())
}

pub async fn init_command(
    secret_manager: SecretManager,
    storage_path: String,
    parameters: InitParameters,
) -> Result<Wallet, Error> {
    let wallet = Wallet::builder()
        .with_secret_manager(secret_manager)
        .with_client_options(ClientOptions::new().with_node(parameters.node_url.as_str())?)
        .with_storage_path(&storage_path)
        .with_coin_type(parameters.coin_type)
        .finish()
        .await?;

    let mnemonic = match parameters.mnemonic_file_path {
        Some(path) => import_mnemonic(&path).await?,
        None => enter_or_generate_mnemonic().await?,
    };

    if let SecretManager::Stronghold(secret_manager) = &mut *wallet.get_secret_manager().write().await {
        secret_manager.store_mnemonic(mnemonic).await?;
    } else {
        panic!("cli-wallet only supports Stronghold-backed secret managers at the moment.");
    }

    Ok(wallet)
}

pub async fn migrate_command(path: Option<String>) -> Result<(), Error> {
    let mut password = get_password("Stronghold password", false)?;
    StrongholdAdapter::migrate_v2_to_v3(
        path.as_deref().unwrap_or(DEFAULT_STRONGHOLD_SNAPSHOT_PATH),
        &password,
        None,
        None,
    )?;
    password.zeroize();
    println_log_info!("Stronghold successfully migrated from v2 to v3.");

    Ok(())
}

pub async fn mnemonic_command() -> Result<(), Error> {
    generate_mnemonic().await?;

    Ok(())
}

pub async fn new_command(wallet: &Wallet, alias: Option<String>) -> Result<String, Error> {
    let mut builder = wallet.create_account();

    if let Some(alias) = alias {
        builder = builder.with_alias(alias);
    }

    let account = builder.finish().await?;
    let alias = account.read().await.alias().to_string();

    println_log_info!("Created account \"{alias}\"");

    Ok(alias)
}

pub async fn restore_command(
    secret_manager: SecretManager,
    storage_path: String,
    backup_path: String,
    password: &str,
) -> Result<Wallet, Error> {
    let wallet = Wallet::builder()
        .with_secret_manager(secret_manager)
        // Will be overwritten by the backup's value.
        .with_client_options(ClientOptions::new().with_node(DEFAULT_NODE_URL)?)
        .with_storage_path(&storage_path)
        // Will be overwritten by the backup's value.
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    wallet
        .restore_backup(backup_path.into(), password.to_string(), None)
        .await?;

    Ok(wallet)
}

pub async fn set_node_command(wallet: &Wallet, url: String) -> Result<(), Error> {
    wallet.set_client_options(ClientOptions::new().with_node(&url)?).await?;

    Ok(())
}

pub async fn sync_command(wallet: &Wallet) -> Result<(), Error> {
    let total_balance = wallet.sync(None).await?;

    println_log_info!("Synchronized all accounts: {:?}", total_balance);

    Ok(())
}
