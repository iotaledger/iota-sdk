// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use dialoguer::{console::Term, theme::ColorfulTheme, Input, Password, Select};
use iota_sdk::wallet::{AccountHandle, Wallet};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::{
    command::{account::AccountCli, wallet::WalletCli},
    error::Error,
    println_log_error, println_log_info,
};

// TODO: make this configurable via the CLI to allow for more secure locations (e.g. encrypted usb drives etc)
const MNEMONIC_FILE_NAME: &str = "mnemonic.txt";

pub fn get_password(prompt: &str, confirmation: bool) -> Result<String, Error> {
    let mut password = Password::new();

    password.with_prompt(prompt);

    if confirmation {
        password.with_confirmation("Confirm password", "Password mismatch");
    }

    Ok(password.interact()?)
}

pub fn get_decision(prompt: &str) -> Result<bool, Error> {
    loop {
        let input = Input::<String>::new()
            .with_prompt(prompt)
            .default("yes".into())
            .interact_text()?;

        match input.to_lowercase().as_str() {
            "yes" | "y" => return Ok(true),
            "no" | "n" => return Ok(false),
            _ => {
                println_log_error!("Accepted input values are: yes|y|no|n");
            }
        }
    }
}

pub async fn get_account_name(prompt: &str, wallet: &Wallet) -> Result<String, Error> {
    let accounts = wallet.get_account_aliases().await?;
    loop {
        let input = Input::<String>::new().with_prompt(prompt).interact_text()?;
        if input.is_empty() || !input.is_ascii() {
            println_log_error!("Invalid input, please choose a non-empty name consisting of ASCII characters.");
        } else if accounts.iter().any(|alias| alias == &input) {
            println_log_error!("Account with alias '{input}' already exists, please choose another name.");
        } else {
            return Ok(input);
        }
    }
}

pub async fn pick_account(wallet: &Wallet) -> Result<Option<AccountHandle>, Error> {
    let mut accounts = wallet.get_accounts().await?;

    match accounts.len() {
        0 => Ok(None),
        1 => Ok(Some(accounts.swap_remove(0))),
        _ => {
            // fetch all available account aliases to display to the user
            let aliases = wallet.get_account_aliases().await?;
            let index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select an account:")
                .items(&aliases)
                .default(0)
                .interact_on(&Term::stderr())?;

            Ok(Some(accounts.swap_remove(index)))
        }
    }
}

pub fn print_wallet_help() {
    use clap::Parser;
    if let Err(err) = WalletCli::try_parse_from(vec!["Wallet:", "help"]) {
        println!("{err}");
    }
}

pub fn print_account_help() {
    use clap::Parser;
    if let Err(err) = AccountCli::try_parse_from(vec!["Account:", "help"]) {
        println!("{err}");
    }
}

pub async fn bytes_from_hex_or_file(hex: Option<String>, file: Option<String>) -> Result<Option<Vec<u8>>, Error> {
    Ok(if let Some(hex) = hex {
        Some(prefix_hex::decode(hex).map_err(|e| Error::Miscellaneous(e.to_string()))?)
    } else if let Some(file) = file {
        Some(tokio::fs::read(file).await?)
    } else {
        None
    })
}

pub async fn generate_mnemonic() -> Result<String, Error> {
    let mnemonic = iota_sdk::client::generate_mnemonic()?;
    println_log_info!("Mnemonic has been generated.");

    let choices = [
        "Write it to console only",
        "Write it to file only",
        "Write it to console and file",
    ];
    let selected_choice = Select::with_theme(&ColorfulTheme::default())
        .items(&choices)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    if matches!(selected_choice, Some(0 | 2)) {
        println!("{}", mnemonic);
    }
    if matches!(selected_choice, Some(1 | 2)) {
        write_mnemonic_to_file(MNEMONIC_FILE_NAME, &mnemonic).await?;
        println_log_info!("Mnemonic has been written to '{MNEMONIC_FILE_NAME}'.");
    }

    println_log_info!("IMPORTANT:");
    println_log_info!("Store this mnemonic in a secure location!");
    println_log_info!(
        "It is the only way to recover your account if you ever forget your password and/or lose the stronghold file."
    );

    Ok(mnemonic)
}

async fn write_mnemonic_to_file(path: &str, mnemonic: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new().create(true).append(true).open(path).await?;
    file.write_all(format!("{mnemonic}\n").as_bytes()).await?;

    Ok(())
}
