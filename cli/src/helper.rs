// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use chrono::{DateTime, NaiveDateTime, Utc};
use clap::Parser;
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use iota_sdk::{
    client::{utils::Password, verify_mnemonic},
    crypto::keys::bip39::Mnemonic,
    wallet::{Account, Wallet},
};
use tokio::{
    fs::{self, OpenOptions},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
use zeroize::Zeroize;

use crate::{
    command::{account::AccountCli, wallet::WalletCli},
    error::Error,
    println_log_error, println_log_info,
};

const DEFAULT_MNEMONIC_FILE_PATH: &str = "./mnemonic.txt";

pub fn get_password(prompt: &str, confirmation: bool) -> Result<Password, Error> {
    let mut password = dialoguer::Password::new();

    password.with_prompt(prompt);

    if confirmation {
        password.with_prompt("Provide a new Stronghold password");
        password.with_confirmation("Confirm password", "Password mismatch");
    }

    Ok(password.interact()?.into())
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

pub async fn get_account_alias(prompt: &str, wallet: &Wallet) -> Result<String, Error> {
    let account_aliases = wallet.get_account_aliases().await?;
    loop {
        let input = Input::<String>::new().with_prompt(prompt).interact_text()?;
        if input.is_empty() || !input.is_ascii() {
            println_log_error!("Invalid input, please choose a non-empty alias consisting of ASCII characters.");
        } else if account_aliases.iter().any(|alias| alias == &input) {
            println_log_error!("Account '{input}' already exists, please choose another alias.");
        } else {
            return Ok(input);
        }
    }
}

pub async fn pick_account(wallet: &Wallet) -> Result<Option<Account>, Error> {
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
    if let Err(err) = WalletCli::try_parse_from(["Wallet:", "help"]) {
        println!("{err}");
    }
}

pub fn print_account_help() {
    if let Err(err) = AccountCli::try_parse_from(["Account:", "help"]) {
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

pub async fn enter_or_generate_mnemonic() -> Result<Mnemonic, Error> {
    let choices = ["Generate a new mnemonic", "Enter a mnemonic"];
    let selected_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select how to provide a mnemonic")
        .items(&choices)
        .default(0)
        .interact_on(&Term::stderr())?;

    let mnemnonic = match selected_choice {
        0 => generate_mnemonic().await?,
        1 => enter_mnemonic()?,
        _ => unreachable!(),
    };

    Ok(mnemnonic)
}

pub async fn generate_mnemonic() -> Result<Mnemonic, Error> {
    let mnemonic = iota_sdk::client::generate_mnemonic()?;
    println_log_info!("Mnemonic has been generated.");
    let choices = [
        "Write it to the console only",
        "Write it to a file only",
        "Write it to the console and a file",
    ];

    let selected_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select how to proceed with it")
        .items(&choices)
        .default(0)
        .interact_on(&Term::stderr())?;

    if [0, 2].contains(&selected_choice) {
        println!("YOUR MNEMONIC:");
        println!("{}", mnemonic.as_ref());
    }
    if [1, 2].contains(&selected_choice) {
        write_mnemonic_to_file(DEFAULT_MNEMONIC_FILE_PATH, &mnemonic).await?;
        println_log_info!("Mnemonic has been written to '{DEFAULT_MNEMONIC_FILE_PATH}'.");
    }

    println_log_info!("IMPORTANT:");
    println_log_info!("Store this mnemonic in a secure location!");
    println_log_info!(
        "It is the only way to recover your account if you ever forget your password and/or lose the stronghold file."
    );

    Ok(mnemonic)
}

pub fn enter_mnemonic() -> Result<Mnemonic, Error> {
    loop {
        let input = Mnemonic::from(
            Input::<String>::new()
                .with_prompt("Enter your mnemonic")
                .interact_text()?,
        );
        if verify_mnemonic(&*input).is_err() {
            println_log_error!("Invalid mnemonic. Please enter a bip-39 conform mnemonic.");
        } else {
            return Ok(input);
        }
    }
}

pub async fn import_mnemonic(path: &str) -> Result<Mnemonic, Error> {
    let mut mnemonics = read_mnemonics_from_file(path).await?;
    if mnemonics.is_empty() {
        println_log_error!("No valid mnemonics found in '{path}'.");
        Err(Error::Miscellaneous("No valid mnemonics found".to_string()))
    } else if mnemonics.len() == 1 {
        Ok(mnemonics.swap_remove(0))
    } else {
        println!("Found {} mnemonics.", mnemonics.len());
        let n = mnemonics.len();
        let selected_index = loop {
            let input = Input::<usize>::new()
                .with_prompt(format!("Pick a mnemonic by its line index in the file ([1..{n}])"))
                .interact_text()?;
            if (1..=n).contains(&input) {
                break input;
            } else {
                println!("Invalid choice. Please pick a valid mnemonic by its index in the range [1..{n}].");
            }
        };
        Ok(mnemonics.swap_remove(selected_index - 1))
    }
}

async fn write_mnemonic_to_file(path: &str, mnemonic: &str) -> Result<(), Error> {
    let mut open_options = OpenOptions::new();
    open_options.create(true).append(true);

    #[cfg(unix)]
    open_options.mode(0o600);

    let mut file = open_options.open(path).await?;
    file.write_all(format!("{mnemonic}\n").as_bytes()).await?;

    Ok(())
}

async fn read_mnemonics_from_file(path: &str) -> Result<Vec<Mnemonic>, Error> {
    let file = OpenOptions::new().read(true).open(path).await?;
    let mut lines = BufReader::new(file).lines();
    let mut mnemonics = Vec::new();
    let mut line_index = 1;
    while let Some(mut line) = lines.next_line().await? {
        // we allow surrounding whitespace in the file
        let trimmed = Mnemonic::from(line.trim().to_owned());
        line.zeroize();
        if verify_mnemonic(&*trimmed).is_ok() {
            mnemonics.push(trimmed);
        } else {
            return Err(Error::Miscellaneous(format!(
                "Invalid mnemonic in file '{path}' at line '{line_index}'."
            )));
        }
        line_index += 1;
    }

    Ok(mnemonics)
}

/// Converts a unix timestamp in milliseconds to a `DateTime<Utc>`
pub fn to_utc_date_time(ts_millis: u128) -> Result<DateTime<Utc>, Error> {
    let millis = ts_millis % 1000;
    let secs = (ts_millis - millis) / 1000;

    let secs_int =
        i64::try_from(secs).map_err(|e| Error::Miscellaneous(format!("Failed to convert timestamp to i64: {e}")))?;
    let nanos = u32::try_from(millis * 1000000)
        .map_err(|e| Error::Miscellaneous(format!("Failed to convert timestamp to u32: {e}")))?;

    let naive_time = NaiveDateTime::from_timestamp_opt(secs_int, nanos).ok_or(Error::Miscellaneous(
        "Failed to convert timestamp to NaiveDateTime".to_string(),
    ))?;

    Ok(DateTime::from_utc(naive_time, Utc))
}

pub async fn check_file_exists(path: &Path) -> Result<(), Error> {
    if !fs::try_exists(path).await.map_err(|e| {
        Error::Miscellaneous(format!(
            "Error while accessing the file '{path}': '{e}'",
            path = path.display()
        ))
    })? {
        return Err(Error::Miscellaneous(format!(
            "File '{path}' does not exist.",
            path = path.display()
        )));
    }
    Ok(())
}
