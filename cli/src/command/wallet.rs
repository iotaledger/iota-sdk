// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use clap::{builder::BoolishValueParser, Args, CommandFactory, Parser, Subcommand};
use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
        stronghold::StrongholdAdapter,
        utils::Password,
    },
    wallet::{account::types::AccountIdentifier, ClientOptions, Wallet},
};
use log::LevelFilter;

use crate::{
    error::Error,
    helper::{check_file_exists, generate_mnemonic, get_password, SecretManagerChoice},
    println_log_info,
};

const DEFAULT_LOG_LEVEL: &str = "debug";
const DEFAULT_SECRET_MANAGER: &str = "stronghold";
const DEFAULT_NODE_URL: &str = "https://api.testnet.shimmer.network";
const DEFAULT_STRONGHOLD_SNAPSHOT_PATH: &str = "./stardust-cli-wallet.stronghold";
const DEFAULT_WALLET_DATABASE_PATH: &str = "./stardust-cli-wallet-db";

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct WalletCli {
    /// Set the path to the wallet database.
    #[arg(long, value_name = "PATH", env = "WALLET_DATABASE_PATH", default_value = DEFAULT_WALLET_DATABASE_PATH)]
    pub wallet_db_path: String,
    /// Set the account to enter.
    pub account: Option<AccountIdentifier>,
    /// Set the log level.
    #[arg(short, long, default_value = DEFAULT_LOG_LEVEL)]
    pub log_level: LevelFilter,
    #[command(subcommand)]
    pub command: Option<WalletCommand>,
}

impl WalletCli {
    pub fn print_help() -> Result<(), Error> {
        Self::command().bin_name("wallet").print_help()?;
        Ok(())
    }
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
    Mnemonic {
        // Output the mnemonic to the specified file.
        #[arg(long)]
        output_file_name: Option<String>,
        // Output the mnemonic to the stdout.
        #[arg(long, num_args = 0..=1, default_missing_value = Some("true"), value_parser = BoolishValueParser::new())]
        output_stdout: Option<bool>,
    },
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
    /// Set the PoW options.
    SetPow {
        /// Whether the PoW should be done locally or remotely.
        #[arg(short, long, action = clap::ArgAction::Set)]
        local_pow: bool,
        /// The amount of workers that should be used for PoW, default is num_cpus::get().
        #[arg(short, long)]
        worker_count: Option<usize>,
    },
    /// Synchronize all accounts.
    Sync,
}

#[derive(Debug, Clone, Args)]
pub struct InitParameters {
    /// Set the secret manager to use.
    #[arg(short, long, value_name = "SECRET_MANAGER", default_value = DEFAULT_SECRET_MANAGER)]
    pub secret_manager: SecretManagerChoice,
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
    /// Coin type, SHIMMER_COIN_TYPE (4219) if not provided.
    #[arg(short, long, default_value_t = SHIMMER_COIN_TYPE)]
    pub coin_type: u32,
}

impl Default for InitParameters {
    fn default() -> Self {
        Self {
            secret_manager: SecretManagerChoice::Stronghold,
            stronghold_snapshot_path: DEFAULT_STRONGHOLD_SNAPSHOT_PATH.to_string(),
            mnemonic_file_path: None,
            node_url: DEFAULT_NODE_URL.to_string(),
            coin_type: SHIMMER_COIN_TYPE,
        }
    }
}

pub async fn accounts_command(wallet: &Wallet) -> Result<(), Error> {
    let accounts = wallet.get_accounts().await?;

    println!("INDEX\tALIAS");
    for account in accounts {
        let details = &*account.details().await;
        println!("{}\t{}", details.index(), details.alias());
    }

    Ok(())
}

pub async fn backup_command_stronghold(wallet: &Wallet, password: &Password, backup_path: &Path) -> Result<(), Error> {
    wallet.backup(backup_path.into(), password.clone()).await?;

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
    Ok(Wallet::builder()
        .with_secret_manager(secret_manager)
        .with_client_options(ClientOptions::new().with_node(init_params.node_url.as_str())?)
        .with_storage_path(storage_path.to_str().expect("invalid wallet db path"))
        .with_coin_type(init_params.coin_type)
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

pub async fn new_account_command(wallet: &Wallet, alias: Option<String>) -> Result<AccountIdentifier, Error> {
    let alias = add_account(wallet, alias).await?;

    Ok(alias)
}

pub async fn node_info_command(wallet: &Wallet) -> Result<(), Error> {
    let node_info = wallet.client().get_info().await?;

    println_log_info!("Current node info: {}", serde_json::to_string_pretty(&node_info)?);

    Ok(())
}

pub async fn restore_command_stronghold(
    storage_path: &Path,
    password: Option<Password>,
    snapshot_path: &Path,
    backup_path: &Path,
) -> Result<Wallet, Error> {
    check_file_exists(backup_path).await?;

    let mut builder = Wallet::builder();
    // providing a password means that a Stronghold snapshot exists (verified by the caller)
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

    let wallet = builder
        // Will be overwritten by the backup's value.
        .with_client_options(ClientOptions::new().with_node(DEFAULT_NODE_URL)?)
        .with_storage_path(storage_path.to_str().expect("invalid unicode"))
        // Will be overwritten by the backup's value.
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    let password = get_password("Stronghold backup password", false)?;
    if let Err(e) = wallet
        .restore_from_backup(backup_path, snapshot_path, password, None, None)
        .await
    {
        // Clean up a failed restore (typically produces a wallet without a secret manager)
        // TODO: a better way would be to not create any files/dirs in the first place when it's not clear yet whether
        // the restore will be successful
        std::fs::remove_dir_all(storage_path)?;
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

pub async fn set_pow_command(wallet: &Wallet, local_pow: bool, worker_count: Option<usize>) -> Result<(), Error> {
    // Need to get the current node, so it's not removed
    let node = wallet.client().get_node().await?;
    let client_options = ClientOptions::new()
        .with_node(node.url.as_ref())?
        .with_local_pow(local_pow)
        .with_pow_worker_count(worker_count);
    wallet.set_client_options(client_options).await?;

    Ok(())
}

pub async fn sync_command(wallet: &Wallet) -> Result<(), Error> {
    let total_balance = wallet.sync(None).await?;

    println_log_info!("Synchronized all accounts: {:?}", total_balance);

    Ok(())
}

pub async fn add_account(wallet: &Wallet, alias: Option<String>) -> Result<AccountIdentifier, Error> {
    let mut account_builder = wallet.create_account();

    if let Some(alias) = alias {
        account_builder = account_builder.with_alias(alias);
    }

    let account = account_builder.finish().await?;
    let alias = AccountIdentifier::Alias(account.details().await.alias().clone());

    println_log_info!("Created account \"{alias}\"");

    Ok(alias)
}
