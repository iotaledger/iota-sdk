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

## 1.0.0-rc.1 - 2023-07-DD

### Added

- `Account::get_metadata()`;
- `SendParams, SendNativeTokensParams, SendNftParams, CreateNativeTokenParams, MintNftParams, CreateAliasOutputParams`;

### Changed

- Moved `minimum_required_storage_deposit()` from `Account` to `Client`;
- `Wallet::create_account()` returns `Account` now;
- Renamed `SendParams` to `AddressAndAmount`;
- `Account::prepare_create_alias_output()` now accepts `CreateAliasOutputParams`;
- `Account::prepare_create_native_token()` now accepts `CreateNativeTokenParams`;
- `Account::prepare_mint_nfts()` now accepts `MintNftParams`;
- `Account::prepare_send()` now accepts `SendParams`;
- `Account::prepare_transaction()` now accepts `Output`;
- `Account::send()` now accepts `SendParams`;
- `Account::prepare_send_native_tokens()` now accepts `SendNativeTokensParams`;
- `Account::prepare_send_nft()` now accepts `SendNftParams`;
- `Account::send_outputs()` now accepts `Output`;

## 1.0.0-rc.0 - 2023-07-11

Initial release of the Python SDK bindings.
