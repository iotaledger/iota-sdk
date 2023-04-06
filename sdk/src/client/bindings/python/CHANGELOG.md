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

## 1.0.0-rc.3 - 2023-MM-DD

### Added

- `Client::hash_transaction_essence()`;
- `Node` class;
- `CoinType` enum class;
- `UnlockContition` class;
- `UnlockContitionType` enum class;
- `Address` class;
- `AddressType` enum class;
- `Feature` class;
- `FeatureType` enum class;
- `NativeToken` class;
- `TokenScheme` class;

### Changed

- Changes from the Rust library;
- `Client::build_alias_output()` state_metadata parameter is now a string;
- `IotaClient` now uses multiple arguments instead of a single dictionary;
- `Client::generate_addresses` now uses multiple arguments instead of a single dictionary;
- `Client::build_alias_output` now uses specific classes instead of dictionaries to pass properties and amount is properly passed as integer;
- `Client::build_basic_output` now uses specific classes instead of dictionaries to pass properties and amount is properly passed as integer;
- `Client::build_foundry_output` now uses specific classes instead of dictionaries to pass properties and amount is properly passed as integer;
- `Client::build_nft_output` now uses specific classes instead of dictionaries to pass properties and amount is properly passed as integer;


### Fixed

- Error raising;
- Don't panic for wrong messages;

## 1.0.0-rc.2 - 2023-02-09

### Added

- `NodeCoreAPI::get_included_block_metadata`;
- `SeedSecretManager` class;

### Changed

- Updated dependencies;

## 1.0.0-rc.1 - 2022-12-14

Initial release of the Python bindings.