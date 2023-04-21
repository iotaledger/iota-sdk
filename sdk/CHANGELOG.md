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

## 0.3.0 - 2023-XX-XX

### Changed

- Renamed `AccountHandle` to `Account`, `Account` to `AccountDetails` and `AccountDto` to `AccountDetailsDto`;
- `AddressWrapper` renamed to `Bech32Address` and moved to `types`;
- `Address::try_from_bech32_with_hrp` address and HRP return have been reversed;
- `PostBlockPayload::payload_dto` renamed to `payload`;
- `SendNativeTokens::addresses_native_tokens` renamed to `addresses_and_native_tokens`;
- `SendNft::addresses_nft_ids` renamed to `addresses_and_nft_ids`;

### Removed

- Remove `AddressGenerationOptions` in favor of `GenerateAddressOptions`, which now contains the `internal` flag.

## 0.2.0 - 2023-04-17

### Added

- `tls` as default feature;
- `{Alias, Basic, Foundry, Nft}Output::clear_unlock_conditions` method;
- `{Alias, Basic, Foundry, Nft}Output::clear_features` method;
- `{Alias, Foundry, Nft}Output::clear_immutable_features` method;
- `{TransactionOptions, TransactionOptionsDto}::allow_micro_amount` field;
- `AddressWithAmount::{new, with_return_address, with_expiration}` methods;
- `{BaseCoinBalance, BaseCoinBalanceDto}::voting_power` field;
- `verify_mnemonic()`;
- `SecretManager::sign_transaction()`;

### Changed

- `AccountManager` and `AccountManagerBuilder` renamed to `Wallet` and `WalletBuilder`;
- `save_account_manager_data` renamed to `save_wallet_data`;
- `get_account_manager_data` renamed to `get_wallet_data`;
- Builder methods `add_unlock_condition`, `replace_unlock_condition`, `with_unlock_conditions` are now generic;
- Builder methods `add_feature`, `replace_feature`, `with_features` are now generic;
- Builder methods `add_immutable_feature`, `replace_immutable_feature`, `with_immutable_features` are now generic;
- Merge `send_amount` and `send_micro_transaction`;
- `AddressWithAmount::{address, amount}` fields are no longer public;
- Fields of `AccountBalance`, `BaseCoinBalance` and `NativeTokensBalance` have been made private and getters have been added;
- Exposed `AccountParticipationOverview, ParticipationEventWithNodes, AliasOutputOptions, AliasOutputOptionsDto, IncreaseNativeTokenSupplyOptions, IncreaseNativeTokenSupplyOptionsDto, NativeTokenOptions, NativeTokenOptionsDto, NftOptions, NftOptionsDto, OutputOptionsDto` from the `account` module;
- Made `Wallet::get_bech32_hrp()` public;

### Removed

- `AddressWithMicroAmount` and `AddressWithAmountDto`;

### Fixed

- Fallback to local PoW;
- Unlock unused inputs;
- Derive location in Stronghold for parallel usage;

## 0.1.0 - 2023-04-03

First release of the `iota-sdk` crate.
