// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod error;
mod helper;
mod wallet_cli;
mod wallet_operation_cli;

use clap::Parser;
use fern_logger::{LoggerConfigBuilder, LoggerOutputConfigBuilder};

use self::{
    error::Error,
    wallet_cli::{new_wallet, WalletCli},
};

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

async fn run(wallet_cli: WalletCli) -> Result<(), Error> {
    if let Some(wallet) = new_wallet(wallet_cli).await? {
        wallet_operation_cli::prompt(&wallet).await?;
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
