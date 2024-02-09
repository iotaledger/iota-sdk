// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod cli;
mod error;
mod helper;
mod secret;
mod wallet_cli;

use clap::Parser;
use fern_logger::{LoggerConfigBuilder, LoggerOutputConfigBuilder};
use iota_sdk::wallet::core::SecretData;
use log::LevelFilter;

use self::{
    cli::{new_wallet, Cli},
    error::Error,
};
use crate::secret::SecretManager;

pub type Wallet = iota_sdk::wallet::Wallet<SecretData<SecretManager>>;

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

fn logger_init(cli: &Cli) -> Result<(), Error> {
    std::panic::set_hook(Box::new(move |panic_info| {
        println_log_error!("{panic_info}");
    }));

    let target_exclusions = &["rustls", "rustyline"];
    let archive = LoggerOutputConfigBuilder::default()
        .name("archive.log")
        .level_filter(cli.log_level)
        .target_exclusions(target_exclusions)
        .color_enabled(false);
    let console = LoggerOutputConfigBuilder::default()
        .level_filter(LevelFilter::Error)
        .target_exclusions(target_exclusions)
        .color_enabled(true);
    let config = LoggerConfigBuilder::default()
        .with_output(archive)
        .with_output(console)
        .finish();

    fern_logger::logger_init(config)?;

    Ok(())
}

async fn run(cli: Cli) -> Result<(), Error> {
    if let Some(wallet) = new_wallet(cli).await? {
        wallet_cli::prompt(&wallet).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cli = match Cli::try_parse() {
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
