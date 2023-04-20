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

## 1.0.0-rc.2 - 2023-02-09

### Added

- `FilterOptions::outputTypes`;
- `NativeTokensBalance::metadata` field;
- `SyncOptions::syncNativeTokenFoundries` field;
- `Transaction::inputs` field;

### Changed

- `getIncomingTransactions()` to return `Transaction` instead of `Map.Entry<TransactionPayload, OutputResponse[]>`;

## 1.0.0-rc.1 - 2022-12-06

### Added

- Add `requestFundsFromFaucet()` function and example;
- Enable native library loading via Java path;
- Throw `InitializeWalletException` on failed initialization;
- Wallet.java `clearListeners`, `destroy` and `initLogger` methods;
- Event listening (`WalletEvent` and `TransactionProgressEvent`);
- `AccountHandle::retryTransactionUntilIncluded`;

### Changed

- Rename `ClientCommand` to `WalletCommand`;
- Move `WalletCommand` class from `NativeApi`;
- Rename `ClientResponse` to `WalletResponse`;

### Fixed

- `RequiredStorageDeposit` deserialization;
