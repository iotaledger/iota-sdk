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
- `SendParams, SendNativeTokensParams, SendNftParams, CreateNativeTokenParams, MintNftParams, CreateAliasOutputParams, OutputParams, Assets, Features, Unlocks, ReturnStrategy, StorageDeposit`;
- `AccountAddress, AddressWithUnspentOutputs`;
- `FilterOptions`;
- `NetworkInfo`;
- `ClientOptions, MqttBrokerOptions, Duration`;

### Changed

- Moved `minimum_required_storage_deposit()` from `Account` to `Client`;
- `Wallet::create_account()` returns `Account` now;
- `SecretManager::{sign_ed25519, sign_secp256k1_ecdsa, signature_unlock}` now accept `Bip44` type chains;
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
- `Account::{generate_ed25519_addresses(), addresses()}` now return `AccountAddress`;
- `Account::addresses_with_unspent_outputs()` now returns `AddressWithUnspentOutputs`;
- `Account::prepare_output()` now returns `Output`;
- `Wallet::get_accounts()` now returns `List[Account]`;
- `OutputData.chain` now is `Optional[Bip44]`;
- `Wallet()` constructor and `Wallet::set_client_options()` now accept `ClientOptions`;
- Split `Account::send()` into `send` and `send_with_params`;

### Removed

- `Wallet::{generate_mnemonic(), verify_mnemonic()}` since they're available from `Utils`;
- `HD_WALLET_TYPE`, `HARDEN_MASK` constants;

### Fixed

- Serialization for wallet methods;

## 1.0.0-rc.0 - 2023-07-11

Initial release of the Python SDK bindings.
