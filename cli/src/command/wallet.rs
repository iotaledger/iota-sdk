// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{fs::File, io::prelude::*};

use clap::{Args, Parser, Subcommand};
use iota_sdk::{
    client::{constants::SHIMMER_COIN_TYPE, secret::SecretManager, utils::generate_mnemonic},
    wallet::{ClientOptions, Wallet},
};
use log::LevelFilter;

use crate::{error::Error, helper::get_password, println_log_info};

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct WalletCli {
    #[command(subcommand)]
    pub command: Option<WalletCommand>,
    pub account: Option<String>,
    #[arg(short, long)]
    pub log_level: Option<LevelFilter>,
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

#[derive(Debug, Default, Clone, Args)]
pub struct InitParameters {
    /// Mnemonic, randomly generated if not provided.
    #[arg(short, long)]
    pub mnemonic: Option<String>,
    /// Node URL, "https://api.testnet.shimmer.network" if not provided.
    #[arg(short, long)]
    pub node: Option<String>,
    /// Coin type, SHIMMER_COIN_TYPE (4219) if not provided.
    #[arg(short, long)]
    pub coin_type: Option<u32>,
}

pub async fn backup_command(wallet: &Wallet, path: String, password: &str) -> Result<(), Error> {
    wallet.backup(path.clone().into(), password.into()).await?;

    println_log_info!("Wallet has been backed up to \"{path}\".");

    Ok(())
}

pub async fn change_password_command(wallet: &Wallet, current: &str) -> Result<(), Error> {
    let new = get_password("Stronghold new password", true)?;

    wallet.change_stronghold_password(current, &new).await?;

    Ok(())
}

pub async fn init_command(
    secret_manager: SecretManager,
    storage_path: String,
    parameters: InitParameters,
) -> Result<Wallet, Error> {
    let wallet = Wallet::builder()
        .with_secret_manager(secret_manager)
        .with_client_options(
            ClientOptions::new().with_node(
                parameters
                    .node
                    .as_deref()
                    .unwrap_or("https://api.testnet.shimmer.network"),
            )?,
        )
        .with_storage_path(&storage_path)
        .with_coin_type(parameters.coin_type.unwrap_or(SHIMMER_COIN_TYPE))
        .finish()
        .await?;

    let mnemonic = match parameters.mnemonic {
        Some(mnemonic) => mnemonic,
        None => generate_mnemonic()?,
    };

    let mut file = File::options().create(true).append(true).open("mnemonic.txt")?;
    // Write mnemonic with new line
    file.write_all(format!("init_command: {mnemonic}\n").as_bytes())?;

    println_log_info!("IMPORTANT: mnemonic has been written to \"mnemonic.txt\", handle it safely.");
    println_log_info!(
        "It is the only way to recover your account if you ever forget your password and/or lose the stronghold file."
    );

    if let SecretManager::Stronghold(secret_manager) = &mut *wallet.get_secret_manager().write().await {
        secret_manager.store_mnemonic(mnemonic).await?;
    } else {
        panic!("cli-wallet only supports Stronghold-backed secret managers at the moment.");
    }
    println_log_info!("Mnemonic stored successfully");

    Ok(wallet)
}

pub async fn mnemonic_command() -> Result<(), Error> {
    let mnemonic = generate_mnemonic()?;

    let mut file = File::options().create(true).append(true).open("mnemonic.txt")?;
    // Write mnemonic with new line
    file.write_all(format!("mnemonic_command: {mnemonic}\n").as_bytes())?;

    println_log_info!("IMPORTANT: mnemonic has been written to \"mnemonic.txt\", handle it safely.");
    println_log_info!(
        "It is the only way to recover your account if you ever forget your password and/or lose the stronghold file."
    );

    Ok(())
}

pub async fn new_command(wallet: &Wallet, alias: Option<String>) -> Result<String, Error> {
    let mut builder = wallet.create_account();

    if let Some(alias) = alias {
        builder = builder.with_alias(alias);
    }

    let account_handle = builder.finish().await?;
    let alias = account_handle.read().await.alias().to_string();

    println_log_info!("Created account \"{alias}\"");

    Ok(alias)
}

pub async fn restore_command(
    secret_manager: SecretManager,
    storage_path: String,
    backup_path: String,
    password: String,
) -> Result<Wallet, Error> {
    let wallet = Wallet::builder()
        .with_secret_manager(secret_manager)
        // Will be overwritten by the backup's value.
        .with_client_options(ClientOptions::new().with_node("https://api.testnet.shimmer.network")?)
        .with_storage_path(&storage_path)
        // Will be overwritten by the backup's value.
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    wallet.restore_backup(backup_path.into(), password, None).await?;

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
