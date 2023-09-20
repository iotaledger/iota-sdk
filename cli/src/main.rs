// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod command;
mod error;
mod helper;
mod wallet;

use clap::Parser;
use fern_logger::{LoggerConfigBuilder, LoggerOutputConfigBuilder};

use self::{command::wallet::WalletCli, error::Error, wallet::new_wallet};

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
    std::panic::set_hook(Box::new(move |panic_info| {
        println_log_error!("{panic_info}");
    }));

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
    if let (Some(wallet), Some(account)) = new_wallet(cli).await? {
        let account = wallet.get_account(account).await?;
        account::account_prompt(&wallet, account).await?;
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

    log::info!(
        "Starting {} v{}",
        std::env!("CARGO_PKG_NAME"),
        std::env!("CARGO_PKG_VERSION")
    );

    if let Err(e) = run(cli).await {
        println_log_error!("{e}");
    }
}
