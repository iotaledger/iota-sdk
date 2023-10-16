# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- ## Unreleased - YYYY-MM-DD

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security -->

## 1.2.0 - 2023-xx-xx

### Changed

- `AccountCommand::Output` now accepts either an index or an ID;

## 1.1.0 - 2023-09-29

### Added

- `WalletCommand::Accounts` variant to list all available accounts in a wallet;
- `addresses` now additionally prints the hex version of the address;
- `outputs`, `unspent-outputs` print a list that includes number and type of the output;
- `Account::switch` command to allow changing accounts quickly;
- UX improvements (Ctrl+l, TAB completion/suggestions and more) during interactive account management;
- `WalletCommand::SetPow` command;
- Check for existing stronghold on `restore`;
- Sync native token foundries to show their metadata;

### Changed

- `WalletCommand::Mnemonic` now takes 2 optional arguments to avoid user interaction;
- `AccountCommand::Transaction` now accepts either an index or an ID;
- Use `CommandFactory` to print help programmatically;
- `print_wallet_help` changed to `WalletCli::print_help`;
- `print_account_help` changed to `AccountCli::print_help`;
- `AccountCommand::Addresses` now prints an overview that includes NTs, NFTs, Aliases and Foundries;
- Restrict permissions of mnemonic file on Windows;

## 1.0.1 - 2023-MM-DD

### Fixed

- Potential bug in the addresses command;

## 1.0.0 - 2023-07-27

First release of the `cli-wallet`.
