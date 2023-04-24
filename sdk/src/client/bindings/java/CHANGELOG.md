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

## 1.0.0-rc.3 - 2023-04-17

### Added

- `OutputIdsResponse`;
- `Client::hashTransactionEssence()`;

### Changed

- Changes from the Rust library;
- `AliasOutputBuilderParams::stateMetadata` from `byte[]` to `String`;
- `Client::{getAliasOutputIds, getBasicOutputIds, getFoundryOutputIds, getNftOutputIds}` will not do automatic pagination if `QueryParameter::Cursor(_)` is provided and return type from `OutputId[]` to `OutputIdsResponse`;

## 1.0.0-rc.2 - 2023-02-09

### Added

- `Client::{aliasIdToBech32(), nftIdToBech32()}`;
- `{Client, NodeCoreApi}::getIncludedBlockMetadata`;
- `Burn::allowBurning` to `Burn::burn`;

### Changed

- Updated dependencies;
- `BuildBlockOptions::`

## 1.0.0-rc.1 - 2022-11-30

### Added

- `org.iota.types.exception` package;
- `NoFundsReceivedFromFaucetException`;

### Changed

- Execute tests in a sequential manner;
- Rename `BaseApi` to `NativeApi`;
- Move out `WalletCommand` from `NativeApi`;
- Disable indexer tests that require a full query until a way to optimize them is found;
- Enable loading of the native library from the Java class path;
