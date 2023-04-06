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

## 0.2.0 - 2023-XX-XX

### Added

- `tls` as default feature;
- `{Alias, Basic, Foundry, Nft}Output::clear_unlock_conditions` method;
- `{Alias, Basic, Foundry, Nft}Output::clear_features` method;
- `{Alias, Foundry, Nft}Output::clear_immutable_features` method;

### Changed

- `AccountManager` and `AccountManagerBuilder` renamed to `Wallet` and `WalletBuilder`;
- `save_account_manager_data` renamed to `save_wallet_data`;
- `get_account_manager_data` renamed to `get_wallet_data`;
- Builder methods `add_unlock_condition`, `replace_unlock_condition`, `with_unlock_conditions` are now generic;
- Builder methods `add_feature`, `replace_feature`, `with_features` are now generic;
- Builder methods `add_immutable_feature`, `replace_immutable_feature`, `with_immutable_features` are now generic;
- Merge `send_amount` and `send_micro_transaction`;

### Fixed

- Fallback to local PoW;
- Unlock unused inputs;

## 0.1.0 - 2023-04-03

First release of the `iota-sdk` crate.
