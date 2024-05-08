// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use std::path::Path;

use chrono::{DateTime, Utc};
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use eyre::{bail, eyre, Error};
use iota_sdk::{
    client::{
        constants::{IOTA_COIN_TYPE, SHIMMER_COIN_TYPE},
        utils::Password,
        verify_mnemonic,
    },
    crypto::keys::{bip39::Mnemonic, bip44::Bip44},
    types::block::address::Bech32Address,
};
use tokio::{
    fs::{self, OpenOptions},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
use zeroize::Zeroize;

use crate::{println_log_error, println_log_info};

const DEFAULT_MNEMONIC_FILE_PATH: &str = "./mnemonic.txt";

pub fn enter_password(prompt: &str, confirmation: bool) -> Result<Password, Error> {
    let mut password = dialoguer::Password::new().with_prompt(prompt);

    if confirmation {
        password = password
            .with_prompt("Provide a new Stronghold password")
            .with_confirmation("Confirm password", "Password mismatch");
    }

    Ok(password.interact()?.into())
}

pub fn enter_decision(prompt: &str, default: &str) -> Result<bool, Error> {
    loop {
        let input = Input::<String>::new()
            .with_prompt(prompt)
            .default(default.to_string())
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

pub fn enter_address() -> Result<Bech32Address, Error> {
    loop {
        let input = Input::<String>::new()
            .with_prompt("Enter a Bech32 wallet address")
            .interact_text()?;
        match Bech32Address::from_str(&input) {
            Ok(address) => {
                return Ok(address);
            }
            Err(err) => {
                println_log_error!("Invalid input, please enter a valid Bech32 address: {err}");
            }
        }
    }
}

pub fn parse_bip_path(input: &str) -> Result<Bip44, Error> {
    if input.is_empty() || !input.is_ascii() {
        bail!("invalid BIP path format. Expected: `<coin_type>/<account_index>/<change_address>/<address_index>`");
    }

    let mut segments = Vec::with_capacity(4);
    for (i, segment) in input.split_terminator('/').map(|p| p.trim()).enumerate() {
        match segment.parse::<u32>() {
            Ok(s) => segments.push(s),
            Err(err) => {
                bail!("invalid BIP path segment. {i}/`{segment}`: {err}");
            }
        }
    }

    if segments.len() != 4 {
        bail!("invalid BIP path format. Expected: `<coin_type>/<account_index>/<change_address>/<address_index>`");
    }

    let bip_path = Bip44::new(segments[0])
        .with_account(segments[1])
        .with_change(segments[2])
        .with_address_index(segments[3]);

    Ok(bip_path)
}

pub fn enter_alias() -> Result<String, Error> {
    loop {
        let input = Input::<String>::new()
            .with_prompt("Enter a wallet alias")
            .interact_text()?;
        if !input.is_empty() && input.is_ascii() {
            return Ok(input);
        } else {
            println_log_error!("Invalid input, please enter a valid alias (non-empty, ASCII).");
        }
    }
}

pub fn enter_mnemonic() -> Result<Mnemonic, Error> {
    loop {
        let mnemonic = Mnemonic::from(Input::<String>::new().with_prompt("Enter a mnemonic").interact_text()?);
        match verify_mnemonic(&*mnemonic) {
            Ok(_) => return Ok(mnemonic),
            Err(err) => {
                println_log_error!("Invalid mnemonic. Please enter a bip-39 conform mnemonic: {err}");
            }
        }
    }
}

pub async fn bytes_from_hex_or_file(hex: Option<String>, file: Option<String>) -> Result<Option<Vec<u8>>, Error> {
    Ok(if let Some(hex) = hex {
        Some(prefix_hex::decode(hex)?)
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

    let mnemonic = match selected_choice {
        0 => generate_mnemonic(None, None).await?,
        1 => enter_mnemonic()?,
        _ => panic!("invalid choice"),
    };

    Ok(mnemonic)
}

pub async fn generate_mnemonic(
    output_file_name: Option<String>,
    output_stdout: Option<bool>,
) -> Result<Mnemonic, Error> {
    let mnemonic = iota_sdk::client::generate_mnemonic()?;
    println_log_info!("Mnemonic has been generated.");

    let selected_choice = match (&output_file_name, &output_stdout) {
        // Undecided, we give the user a choice
        (None, None) | (None, Some(false)) => {
            let choices = [
                "Write it to the console only",
                "Write it to a file only",
                "Write it to the console and a file",
            ];

            Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select how to proceed with it")
                .items(&choices)
                .default(0)
                .interact_on(&Term::stderr())?
        }
        // Only console
        (None, Some(true)) => 0,
        // Only file
        (Some(_), Some(false)) | (Some(_), None) => 1,
        // File and console
        (Some(_), Some(true)) => 2,
    };

    if [0, 2].contains(&selected_choice) {
        println!("YOUR MNEMONIC:");
        println!("{}", mnemonic.as_ref());
    }
    if [1, 2].contains(&selected_choice) {
        let file_path = output_file_name.unwrap_or_else(|| DEFAULT_MNEMONIC_FILE_PATH.to_string());

        write_mnemonic_to_file(&file_path, &mnemonic).await?;
        println_log_info!("Mnemonic has been written to '{file_path}'.");
    }

    println!("IMPORTANT:");
    println!("Store this mnemonic in a secure location!");
    println!(
        "It is the only way to recover your wallet if you ever forget your password and/or lose the stronghold file."
    );

    Ok(mnemonic)
}

pub async fn import_mnemonic(path: &str) -> Result<Mnemonic, Error> {
    let mut mnemonics = read_mnemonics_from_file(path).await?;
    if mnemonics.is_empty() {
        println_log_error!("No valid mnemonics found in '{path}'.");
        bail!("No valid mnemonics found".to_string())
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

    #[cfg(windows)]
    restrict_file_permissions(path)?;

    Ok(())
}

// Slightly modified from https://github.com/sile/sloggers/blob/master/src/permissions.rs
#[cfg(windows)]
pub fn restrict_file_permissions<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    use std::io;

    use winapi::um::winnt::{FILE_GENERIC_READ, FILE_GENERIC_WRITE, PSID, STANDARD_RIGHTS_ALL};
    use windows_acl::{
        acl::{AceType, ACL},
        helper::sid_to_string,
    };

    /// This is the security identifier in Windows for the owner of a file. See:
    /// - https://docs.microsoft.com/en-us/troubleshoot/windows-server/identity/security-identifiers-in-windows#well-known-sids-all-versions-of-windows
    const OWNER_SID_STR: &str = "S-1-3-4";
    /// We don't need any of the `AceFlags` listed here:
    /// - https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header
    const OWNER_ACL_ENTRY_FLAGS: u8 = 0;
    /// Generic Rights:
    ///  - https://docs.microsoft.com/en-us/windows/win32/fileio/file-security-and-access-rights
    /// Individual Read/Write/Execute Permissions (referenced in generic rights link):
    ///  - https://docs.microsoft.com/en-us/windows/win32/wmisdk/file-and-directory-access-rights-constants
    /// STANDARD_RIGHTS_ALL
    ///  - https://docs.microsoft.com/en-us/windows/win32/secauthz/access-mask
    const OWNER_ACL_ENTRY_MASK: u32 = FILE_GENERIC_READ | FILE_GENERIC_WRITE | STANDARD_RIGHTS_ALL;

    let path_str = path
        .as_ref()
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Unable to open file path.".to_string()))?;

    let mut acl = ACL::from_file_path(path_str, false)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Unable to retrieve ACL: {:?}", e)))?;

    let owner_sid = windows_acl::helper::string_to_sid(OWNER_SID_STR)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Unable to convert SID: {:?}", e)))?;

    let entries = acl.all().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Unable to enumerate ACL entries: {:?}", e),
        )
    })?;

    // Add single entry for file owner.
    acl.add_entry(
        owner_sid.as_ptr() as PSID,
        AceType::AccessAllow,
        OWNER_ACL_ENTRY_FLAGS,
        OWNER_ACL_ENTRY_MASK,
    )
    .map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to add ACL entry for SID {} error={}", OWNER_SID_STR, e),
        )
    })?;
    // Remove all AccessAllow entries from the file that aren't the owner_sid.
    for entry in &entries {
        if let Some(ref entry_sid) = entry.sid {
            let entry_sid_str = sid_to_string(entry_sid.as_ptr() as PSID).unwrap_or_else(|_| "BadFormat".to_string());
            if entry_sid_str != OWNER_SID_STR {
                acl.remove(entry_sid.as_ptr() as PSID, Some(AceType::AccessAllow), None)
                    .map_err(|_| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            format!("Failed to remove ACL entry for SID {}", entry_sid_str),
                        )
                    })?;
            }
        }
    }
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
            bail!("Invalid mnemonic in file '{path}' at line '{line_index}'.");
        }
        line_index += 1;
    }

    Ok(mnemonics)
}

/// Converts a unix timestamp in milliseconds to a `DateTime<Utc>`
pub fn to_utc_date_time(ts_millis: u128) -> Result<DateTime<Utc>, Error> {
    let millis = ts_millis % 1000;
    let secs = (ts_millis - millis) / 1000;

    let secs_int = i64::try_from(secs).map_err(|e| eyre!("Failed to convert timestamp to i64: {e}"))?;
    let nanos = u32::try_from(millis * 1000000).map_err(|e| eyre!("Failed to convert timestamp to u32: {e}"))?;

    DateTime::from_timestamp(secs_int, nanos).ok_or(eyre!("Failed to convert timestamp to DateTime"))
}

pub async fn check_file_exists(path: &Path) -> Result<(), Error> {
    if !fs::try_exists(path)
        .await
        .map_err(|e| eyre!("Error while accessing the file '{path}': '{e}'", path = path.display()))?
    {
        bail!("File '{path}' does not exist.", path = path.display());
    }
    Ok(())
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum SecretManagerChoice {
    Stronghold,
    LedgerNano,
    LedgerNanoSimulator,
}

impl From<usize> for SecretManagerChoice {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Stronghold,
            1 => Self::LedgerNano,
            2 => Self::LedgerNanoSimulator,
            _ => panic!("invalid secret manager choice"),
        }
    }
}

impl FromStr for SecretManagerChoice {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stronghold" => Ok(Self::Stronghold),
            "ledger-nano" => Ok(Self::LedgerNano),
            "ledger-nano-sim" => Ok(Self::LedgerNanoSimulator),
            _ => Err("invalid secret manager specifier [stronghold|ledger-nano|ledger-nano-sim]"),
        }
    }
}

pub fn select_secret_manager() -> Result<SecretManagerChoice, Error> {
    let choices = ["Stronghold", "Ledger Nano", "Ledger Nano Simulator"];

    Ok(Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select secret manager")
        .items(&choices)
        .default(0)
        .interact_on(&Term::stderr())?
        .into())
}

pub fn select_or_enter_bip_path() -> Result<Bip44, Error> {
    let choices = ["IOTA [4218/0/0/0]", "Shimmer [4219/0/0/0]", "Custom"];

    Ok(
        match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select bip path")
            .items(&choices)
            .default(0)
            .interact_on(&Term::stderr())?
        {
            0 => Bip44::new(IOTA_COIN_TYPE),
            1 => Bip44::new(SHIMMER_COIN_TYPE),
            2 => enter_bip_path()?,
            _ => panic!("invalid choice"),
        },
    )
}

fn enter_bip_path() -> Result<Bip44, Error> {
    loop {
        let input = Input::<String>::new().with_prompt("Enter a bip path").interact_text()?;
        match parse_bip_path(&input) {
            Ok(bip_path) => return Ok(bip_path),
            Err(err) => {
                let s = err.to_string();
                println_log_error!("{s}");
            }
        }
    }
}
