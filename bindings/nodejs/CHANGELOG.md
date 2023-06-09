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

### Changed

- Rename `Client::listen` to `listenMqtt`, `Client::clearListeners` to `clearMqttListeners`;
- Moved `minimumRequiredStorageDeposit()` from `Account` to `Client`;

## 1.0.0-rc.2 - 2023-07-05

### Added

- `callPluginRoute` to Client to fetch data from custom node plugins;
- `computeTokenId `, `computeOutputId`, `computeInputsCommitment` and `computeStorageDeposit` to Utils;
- Type alias for Ids which were previously just `HexEncodedString`;
- List of `ConfictReason` explanations matching the enum;
- `units-helper` class for IOTA units conversion;
- `Client::destroy` to close an open handle;

### Changed

- Rename `Account::prepareMintNativeToken` to `prepareCreateNativeToken`, `Account::prepareIncreaseNativeTokenSupply` to `prepareMintNativeToken`, `Account::prepareDecreaseNativeTokenSupply` to `prepareMeltNativeToken`;
- Rename `MintNativeTokenParams` to `CreateNativeTokenParams`;
- Rename `MintTokenTransaction` to `CreateNativeTokenTransaction` and `PreparedMintTokenTransaction` to `PreparedCreateNativeTokenTransaction` (including their corresponding `Data` types);
- Rename `SendAmountParams` to `SendParams`;
- Rename `Account::sendAmount` to `send`, `Account::prepareSendAmount` to `prepareSend`;
- Rename `Response::MilestoneRaw` to `Raw`;

### Fixed

- Moved `internal` field from `IGenerateAddressesOptions` to `IGenerateAddressOptions`;
- Error handling in `Client`, `SecretManager` and `Wallet` constructors;
- Deadlock in .sync() with incoming transactions;
- Renamed `Output.getNntId` to `Output.getNftId`;

## 1.0.0-rc.1 - 2023-06-19

### Added

- `Utils::verifySecp256k1EcdsaSignature`;

### Changed

- `Account::getOutputsWithAdditionalUnlockConditions` renamed to `claimableOutputs`;
- Rename `Account::signEvm` to `signSecp256k1Ecdsa` and `EvmSignature` to `Secp256k1EcdsaSignature`;

### Removed

- `Utils::verifyEd25519Signature`'s `address` parameter;

### Fixed

- `UTXOInput` constructs with type the proper `InputType`;

## 1.0.0-rc.0 - 2023-06-15

Initial release of the Node.js SDK bindings.
