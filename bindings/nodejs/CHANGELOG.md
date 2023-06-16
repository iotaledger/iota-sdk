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

## 1.0.0-rc.1 - YYYY-MM-DD

### Added

- `Utils::verifyEd25519Signature` and `verifySecp256k1EcdsaSignature`;

### Changed

- `Account::getOutputsWithAdditionalUnlockConditions` renamed to `claimableOutputs`;
- Rename `Account::signEvm` to `signSecp256k1Ecdsa` and `EvmSignature` to `Secp256k1EcdsaSignature`;

## 1.0.0-rc.0 - 2023-06-15

Initial release of the Node.js SDK bindings.
