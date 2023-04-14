// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod account_completion;
mod account_history;
mod command;
mod error;
mod helper;
mod wallet;

use clap::Parser;
use fern_logger::{LoggerConfigBuilder, LoggerOutputConfigBuilder};

use self::{command::wallet::WalletCli, error::Error, helper::pick_account, wallet::new_wallet};

#[macro_export]
macro_rules! println_log_info {
    ($($arg:tt)+) => {
        println!($($arg)+);
        log::info!($($arg)+);
    };
}

#[macro_export]
macro_rules! println_log_error {
    ($($arg:tt)+) => {
        println!($($arg)+);
        log::error!($($arg)+);
    };
}

fn logger_init(cli: &WalletCli) -> Result<(), Error> {
    let archive = LoggerOutputConfigBuilder::default()
        .name("archive.log")
        .level_filter(cli.log_level)
        .target_exclusions(&["rustls"])
        .color_enabled(false);
    let config = LoggerConfigBuilder::default().with_output(archive).finish();

    fern_logger::logger_init(config)?;

    Ok(())
}

async fn run(cli: WalletCli) -> Result<(), Error> {
    let (wallet, account) = new_wallet(cli.clone()).await?;

    if let Some(wallet) = wallet {
        match cli.account.or(account) {
            Some(account) => account::account_prompt(wallet.get_account(account).await?).await?,
            None => {
                if let Some(account) = pick_account(&wallet).await? {
                    account::account_prompt(wallet.get_account(account).await?).await?;
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cli = match WalletCli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    if let Err(e) = logger_init(&cli) {
        println!("{e}");
        return;
    }

    if let Err(e) = run(cli).await {
        println_log_error!("{e}");
    }
}
