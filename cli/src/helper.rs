// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, NaiveDateTime, Utc};
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Password, Select};
use iota_sdk::{client::verify_mnemonic, wallet::Wallet};
use tokio::{
    fs::OpenOptions,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};

use crate::{error::Error, println_log_error, println_log_info};

const DEFAULT_MNEMONIC_FILE_PATH: &str = "./mnemonic.txt";

pub fn get_password(prompt: &str, confirmation: bool) -> Result<String, Error> {
    let mut password = Password::new();

    password.with_prompt(prompt);

    if confirmation {
        password.with_confirmation("Confirm password", "Password mismatch");
    }

    Ok(password.interact()?)
}

pub async fn pick_account(wallet: &Wallet) -> Result<Option<u32>, Error> {
    let accounts = wallet.get_accounts().await?;

    match accounts.len() {
        0 => Ok(None),
        1 => Ok(Some(0)),
        _ => {
            let mut items = Vec::new();

            for account in accounts {
                items.push(account.read().await.alias().clone());
            }

            let index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select an account:")
                .items(&items)
                .default(0)
                .interact_on(&Term::stderr())?;

            Ok(Some(index as u32))
        }
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

pub async fn enter_or_generate_mnemonic() -> Result<String, Error> {
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

pub async fn generate_mnemonic() -> Result<String, Error> {
    let mnemonic = iota_sdk::client::generate_mnemonic()?;
    println_log_info!("Mnemonic has been generated.");

    let choices = [
        "Write to console only",
        "Write to file only",
        "Write to console and file",
    ];
    let selected_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select how to proceed with it")
        .items(&choices)
        .default(0)
        .interact_on(&Term::stderr())?;

    if [0, 2].contains(&selected_choice) {
        println!("{}", mnemonic);
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

pub fn enter_mnemonic() -> Result<String, Error> {
    loop {
        let input = Input::<String>::new()
            .with_prompt("Enter your mnemonic")
            .interact_text()?;
        if verify_mnemonic(&input).is_err() {
            println_log_error!("Invalid mnemonic. Please enter a bip-39 conform mnemonic.");
        } else {
            return Ok(input);
        }
    }
}

pub async fn import_mnemonic(path: &str) -> Result<String, Error> {
    let mut mnemonics = read_mnemonics_from_file(path).await?;
    if mnemonics.is_empty() {
        println_log_error!("No valid mnemonics found in '{path}'.");
        Err(Error::Miscellaneous("No valid mnemonics found".to_string()))
    } else if mnemonics.len() == 1 {
        Ok(mnemonics.swap_remove(0))
    } else {
        println!("Found {} mnemonics.", mnemonics.len());
        let n = mnemonics.len() - 1;
        let selected_index = loop {
            let input = Input::<usize>::new()
                .with_prompt(format!("Pick a mnemonic by its index in the range [0..{n}]"))
                .interact_text()?;
            if (0..=n).contains(&input) {
                break input;
            } else {
                println!("Invalid choice. Please pick a valid mnemonic by its index in the range [0..{n}].");
            }
        };
        Ok(mnemonics.swap_remove(selected_index))
    }
}

async fn write_mnemonic_to_file(path: &str, mnemonic: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new().create(true).append(true).open(path).await?;
    file.write_all(format!("{mnemonic}\n").as_bytes()).await?;

    Ok(())
}

async fn read_mnemonics_from_file(path: &str) -> Result<Vec<String>, Error> {
    let file = OpenOptions::new().read(true).open(path).await?;
    let mut lines = BufReader::new(file).lines();
    let mut mnemonics = Vec::new();
    while let Some(line) = lines.next_line().await? {
        if verify_mnemonic(&line).is_ok() {
            mnemonics.push(line.trim().to_string());
        }
    }

    Ok(mnemonics)
}

/// Converts a unix timestamp in milliseconds to a DateTime<Utc>
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
