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

### Added

- `outputs` and `unspent_outputs` print the booked milestone timestamps and sort by them;
- `outputs` and `unspent_outputs` include spent/unspent information;
- `UTC` suffix to the formatted date of `transactions`;

### Changed

- `AccountCommand::Output` accepts either a list index or an `OutputId`;

### Fixed

- `transaction` and `transactions` indexed transactions in opposite order;
- Enter doesn't show helper anymore;
- Earlier trim of the input so that pre-commands (`help`, `clear`, `accounts`, ...) work even with leading/trailing spaces;

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
