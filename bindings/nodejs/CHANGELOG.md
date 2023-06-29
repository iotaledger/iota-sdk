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

## 1.0.0-rc.2 - 2023-0x-xx

### Changed

- Rename `Account::prepareMintNativeToken` to `prepareCreateNativeToken`, `Account::prepareIncreaseNativeTokenSupply` to `prepareMintNativeToken`, `Account::prepareDecreaseNativeTokenSupply` to `prepareMeltNativeToken`;
- Rename `MintNativeTokenParams` to `CreateNativeTokenParams`;
- Rename `MintTokenTransaction` to `CreateNativeTokenTransaction` and `PreparedMintTokenTransaction` to `PreparedCreateNativeTokenTransaction` (including their corresponding `Data` types);

### Fixed

- Moved `internal` field from `IGenerateAddressesOptions` to `IGenerateAddressOptions`;
- Error handling in `Client`, `SecretManager` and `Wallet` constructors;

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
