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

## 1.1.5 - 2024-01-29

### Added

- `Utils::{verifyTransactionSyntax(), blockBytes(), blockHashWithoutNonce()}`;

### Fixed

- `Output` and `SimpleTokenScheme` (sub)fields not setting the `bigint` properly;

## 1.1.4 - 2023-12-07

### Added

- `Utils::verifyTransactionSemantic()`;
- `Account::prepareClaimOutputs()` method;

### Fixed

- `StateMetadataOutput`'s constructor not setting the `stateMetadata` field;

## 1.1.3 - 2023-10-27

### Fixed

- Prebuild binaries;

## 1.1.2 - 2023-10-26

### Fixed

- Slow syncing with many claimable outputs;
- Potential deadlock in syncing;

## 1.1.1 - 2023-10-11

### Added

- `UnlockableByAddress` to `AliasQueryParameter, NftQueryParameter, QueryParameter`;

### Fixed

- Added `SeedSecretManager` to `SecretManagerType`;
- `migrateDbChrysalisToStardust()` for some ledger nano wallets;

### Removed

- `AliasAddress` from `NftQueryParameter`;

## 1.1.0 - 2023-09-29

### Changed

- More type hints;

## 1.1.0-rc.2 - 2023-09-28

Changes from the 1.0 track.

## 1.1.0-rc.1 - 2023-09-26

### Fixed

- Import of bindings through types;

## 1.1.0-rc.0 - 2023-09-25

### Added

- `Account::{burn(), consolidateOutputs(), createAliasOutput(), meltNativeToken(), mintNativeToken(), createNativeToken(), mintNfts(), sendTransaction(), sendNativeTokens(), sendNft()}` methods;
- `Client::outputIds()` method;
- `GenericQueryParameter, UnlockableByAddress` types;
- `Irc27Metadata` and `Irc30Metadata` helpers;
- `Utils::outputHexBytes`;
- `PrivateKeySecretManager`;

## 1.0.13 - 2023-09-28

### Fixed

- `migrateDbChrysalisToStardust()` when ledger nano was used as signer type;

## 1.0.12 - 2023-09-25

### Changed

- Made `TransactionOptions.allowMicroAmount` optional;

### Fixed

- Parsing of `RegularTransactionEssence.payload`;
- Don't error if custom remainder address is provided with ledger nano;

## 1.0.11 - 2023-09-14

### Fixed

- `Client::getNetworkId()` return type;
- `RegularTransactionEssence::networkId` type;
- `FilterOptions::outputTypes` type;

## 1.0.10 - 2023-09-12

### Changed

- `migrateDbChrysalisToStardust()` returns an error if no chrysalis data was found;

### Fixed

- Type of `value` property in `CustomAddress`;

## 1.0.9 - 2023-09-07

### Added

- `IClientOptions::maxParallelApiRequests`;

### Fixed

- The main thread gets blocked when calling client or wallet methods;

## 1.0.8 - 2023-09-05

### Added

- `migrateDbChrysalisToStardust` function;
- `Wallet::getChrysalisData` method;

## 1.0.7 - 2023-08-29

### Fixed

- Migration mismatch from `iota-rs` version;

## 1.0.6 - 2023-08-25

### Fixed

- `Account::prepareBurn()` return type;
- `Wallet::restoreBackup()` when no secret manager data is stored inside;

## 1.0.5 - 2023-08-18

### Added

- Export `ILoggerConfig` from types;
- Added `Account::prepareIncreaseVotingPower`;

## Changed

- Deprecate `Account::prepareVotingPower`;

### Fixed

- `Account::prepareOutput()` when `ReturnStrategy::Gift` is used with an existing NFT output;

## 1.0.4 - 2023-08-08

### Fixed

- Missing production profile when no prebuild binary is available;
- Ledger Nano events properly created when preparing transactions;
- Prevent loading of bindings when importing UTXOInput type (changed UTXOInput.fromOutputId implementation);

## 1.0.3 - 2023-07-31

### Fixed

- `Balance::{baseCoin, requiredStorageDeposit}` amounts;
- `Utils::computeStorageDeposit()` return amount;

## 1.0.2 - 2023-07-28

### Changed

- Private properties on classes are now readonly;

### Fixed

- Constructor types in `RegularTransactionEssence`;
- `SenderFeature.getSender()` and `IssuerFeature.getIssuer()` now return the correct types;

## 1.0.1 - 2023-07-25

### Changed

- Deprecate `Account::{buildAliasOutput(), buildBasicOutput(), buildFoundryOutput(), buildNftOutput()}` and their `BuildAliasOutputData, BuildBasicOutputData, BuildFoundryOutputData, BuildNftOutputData` parameter types;

## 1.0.0 - 2023-07-24

### Added

- `fromOutputId` constructor for `UTXOInput`;
- `migrateStrongholdSnapshotV2ToV3` function;
- `ConsolidationParams`;

### Changed

- `Account::prepareConsolidateOutputs` takes a `ConsolidationParams`;

### Fixed

- `Utils.computeStorageDeposit()`;
- `Utils.computeTokenId()`;
- `Utils.computeFoundryId` 's parameter `tokenSchemeKind` renamed to `tokenSchemeType`;

## 1.0.0-rc.3 - 2023-07-21

### Added

- `TransactionProgressType` export;
- Optional `CreateAccountPayload::addresses` field;
- Optional `FilterOptions::{aliasIds, foundryIds, nftIds}` fields;

### Changed

- Rename `Client::listen` to `listenMqtt`, `Client::clearListeners` to `clearMqttListeners`;
- Moved `minimumRequiredStorageDeposit()` from `Account` to `Client`;
- `SecretManagerMethod::SignEd25519`, `SignSecp256k1Ecdsa`, and `SignatureUnlock` now accept newly added `Bip44` type chains;
- Use `BigInt` instead of strings for token amounts;
- Split `Account::send` into `send` and `sendWithParams`;
- Properties in event classes are now public;
- Remove getters from the event classes;
- `{Client, SecretManager}::signTransaction` return type from `Payload` to `TransactionPayload`;

### Removed

- `HD_WALLET_TYPE`, `HARDEN_MASK` constants;
- `CommonOutput::{setNativeTokens, setFeatures}`;
- `ImmutableFeaturesOutput::setImmutableFeatures`;
- `StateMetadataOutput::setStateMetadata`;
- `Client::findOutputs()` method;

### Fixed

- Super class of `NftOutput` from `StateMetadataOutput` to `ImmutableFeaturesOutput`;
- `Utils::parseBech32Address` now converts the string into a proper `Address`;
- Change type of `transactionInputs` from `[OutputResponse]` `to OutputResponse[]` in class `NewOutputWalletEvent`;
- Renamed `UTXOInput::transactionInputIndex` to `UTXOInput::transactionOutputIndex`;

## 1.0.0-rc.2 - 2023-07-05

### Added

- `callPluginRoute` to Client to fetch data from custom node plugins;
- `computeTokenId `, `computeOutputId`, `computeInputsCommitment` and `computeStorageDeposit` to Utils;
- Type alias for Ids which were previously just `HexEncodedString`;
- List of `ConflictReason` explanations matching the enum;
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
