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

## 1.1.0 - 2023-MM-DD

### Added

- `WalletCommand::Accounts` variant to list all available accounts in a wallet;
- `addresses` now additionally prints the hex version of the address;
- `outputs`, `unspent-outputs` print a list that includes number and type of the output;
- `Account::switch` command to allow changing accounts quickly;

### Changed

- `WalletCommand::Mnemonic` now takes 2 optional arguments to avoid user interaction;
- `AccountCommand::Transaction` now accepts either an index or an ID;
- Use `CommandFactory` to print help programmatically;
- `print_wallet_help` changed to `WalletCli::print_help`;
- `print_account_help` changed to `AccountCli::print_help`;
- `AccountCommand::Addresses` now prints an overview that includes NTs, NFTs, Aliases and Foundries;

## 1.0.0 - 2023-07-27

First release of the `cli-wallet`.
