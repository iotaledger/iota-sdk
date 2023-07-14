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
- `Bip44` type;

### Changed

- Moved `minimum_required_storage_deposit()` from `Account` to `Client`;
- `Wallet::create_account()` returns `Account` now;
- `SecretManager:: sign_ed25519`, `sign_secp256k1_ecdsa`, and `signature_unlock` now accept `Bip44` type chains;

## 1.0.0-rc.0 - 2023-07-11

Initial release of the Python SDK bindings.
