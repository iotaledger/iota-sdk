// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use dialoguer::{console::Term, theme::ColorfulTheme, Input, Password, Select};
use iota_sdk::wallet::{AccountHandle, Wallet};

use crate::error::Error;

pub fn get_password(prompt: &str, confirmation: bool) -> Result<String, Error> {
    let mut password = Password::new();

    password.with_prompt(prompt);

    if confirmation {
        password.with_confirmation("Confirm password", "Password mismatch");
    }

    Ok(password.interact()?)
}

pub fn get_decision(prompt: &str) -> Result<bool, Error> {
    let input = Input::<String>::new()
        .with_prompt(prompt)
        .default("Yes".into())
        .interact_text()?;

    match input.as_str() {
        "Yes" | "yes" | "y" => Ok(true),
        "No" | "no" | "n" => Ok(false),
        _ => Err(Error::InvalidInput {
            expected: "Yes|yes|y or No|no|n".to_string(),
            found: input,
        }),
    }
}

pub async fn pick_account(wallet: &Wallet) -> Result<Option<AccountHandle>, Error> {
    let mut accounts = wallet.get_accounts().await?;

    match accounts.len() {
        0 => Ok(None),
        1 => Ok(Some(accounts.swap_remove(0))),
        _ => {
            // fetch all available account aliases to display to the user
            let mut aliases = Vec::with_capacity(accounts.len());
            for account_handle in &accounts {
                let alias = account_handle.read().await.alias().clone();
                aliases.push(alias);
            }
            let index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select an account:")
                .items(&aliases)
                .default(0)
                .interact_on(&Term::stderr())?;

            Ok(Some(accounts.swap_remove(index)))
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
