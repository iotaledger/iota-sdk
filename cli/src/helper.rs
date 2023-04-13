// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use dialoguer::{console::Term, theme::ColorfulTheme, Password, Select};
use iota_sdk::{client::generate_mnemonic, wallet::Wallet};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::{error::Error, println_log_info};

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

pub async fn pick_account(wallet: &Wallet) -> Result<Option<u32>, Error> {
    let accounts = wallet.get_accounts().await?;

    match accounts.len() {
        0 => Ok(None),
        1 => Ok(Some(0)),
        _ => {
            let mut items = Vec::new();

            for account_handle in accounts {
                items.push(account_handle.read().await.alias().clone());
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

pub async fn get_mnemonic() -> Result<String, Error> {
    let mnemonic = generate_mnemonic()?;
    println_log_info!("Mnemonic has been generated.");

    let choices = [
        "Write To Console Only",
        "Write To File Only",
        "Write To Console and File",
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
