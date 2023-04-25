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

### Added

- `NativeTokensBuilder::finish_set`;
- `Features`, `UnlockConditions`, `NativeTokens`, `MilestoneOptions`, and `Parents` added `from_set`;
- `types::block::Error::InvalidField` variant;
- `StorageProvider` and `SecretManage` have an `Error` associated type;
- `SecretManageExt` is a super trait of `SecretManage`;
- `OutputsToClaim::Amount` to allow claiming only outputs that will add an amount to the account;

### Changed

- Renamed `AccountHandle` to `Account`, `Account` to `AccountDetails` and `AccountDto` to `AccountDetailsDto`;
- `AddressWrapper` renamed to `Bech32Address` and moved to `types`;
- `Address::try_from_bech32_with_hrp` address and HRP return have been reversed;
- `PostBlockPayload::payload_dto` renamed to `payload`;
- `SendNativeTokens::addresses_native_tokens` renamed to `addresses_and_native_tokens`;
- `SendNft::addresses_nft_ids` renamed to `addresses_and_nft_ids`;
- `Output` builder types, `NativeTokensBuilder`, and `Burn` now use unique, ordered sets for unlock conditions, features, and native tokens. `add_x` and `replace_x` methods thus function appropriately;
- `Features`, `UnlockConditions`, `NativeTokens`, `MilestoneOptions`, and `Parents` constructor `new` renamed to `from_vec`;
- Modified `Ord` and `PartialOrd` implementations for `Feature`, `UnlockCondition`, `NativeToken`, and `MilestoneOption` to support unique, ordered sets;
- `{AliasOutputBuilder, BasicOutputBuilder, FoundryOutputBuilder, NftOutputBuilder}::{new_with_amount, new_with_minimum_storage_deposit, new, with_amount}` don't return a `Result` anymore;
- `{AliasOutput, BasicOutput, FoundryOutput, NftOutput}::{build_with_amount, build_with_minimum_storage_deposit}` don't return a `Result` anymore;
- Lots of builder setters are now taking an `impl Into<Option<T>>` instead of a `T` parameter;
- All `ledger_nano` related errors have been moved from the client error to a newly created `client::secret::ledger_nano::Error`;
- All `stronghold` related errors have been moved from the client error to a newly created `client::stronghold::Error`;

### Removed

- `AddressGenerationOptions` in favor of `GenerateAddressOptions`, which now contains the `internal` flag;
- `types::block::DtoError`, `client::Error::BlockDto` and `wallet::Error::BlockDto`;
- `BasicOutput`, `AliasOutput`, `FoundryOutput`, `NftOutput` - `new_with_amount` and `new_with_minimum_storage_deposit` functions;

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

First release of the `iota-sdk` crate which is a combination and successor of [iota.rs](https://github.com/iotaledger/iota.rs) and [wallet.rs](https://github.com/iotaledger/wallet.rs).

This is a strict implementation of the `stardust` related [TIPs](https://github.com/iotaledger/tips) which are not compatible with the `chrysalis` features set.

All the changes compared to the previous version are mostly derived from the following TIPs:

- [Multi-Asset Ledger and ISC Support](https://github.com/iotaledger/tips/blob/main/tips/TIP-0018/tip-0018.md)
- [Dust Protection Based on Byte Costs (Storage Deposit)](https://github.com/iotaledger/tips/blob/main/tips/TIP-0019/tip-0019.md)
- [Transaction Payload with TIP-18 Output Types](https://github.com/iotaledger/tips/blob/main/tips/TIP-0020/tip-0020.md)
- [Tangle Block](https://github.com/iotaledger/tips/blob/main/tips/TIP-0024/tip-0024.md)
- [Core REST API](https://github.com/iotaledger/tips/blob/main/tips/TIP-0025/tip-0025.md)
- [UTXO Indexer API](https://github.com/iotaledger/tips/blob/main/tips/TIP-0026/tip-0026.md)
- [Event API](https://github.com/iotaledger/tips/blob/main/tips/TIP-0028/tip-0028.md)
- [Milestone Payload](https://github.com/iotaledger/tips/blob/main/tips/TIP-0029/tip-0029.md)
- [Bech32 Address Format](https://github.com/iotaledger/tips/blob/main/tips/TIP-0031/tip-0031.md)
- [Shimmer Protocol Parameters](https://github.com/iotaledger/tips/blob/main/tips/TIP-0032/tip-0032.md)

Past changelogs: [types](https://github.com/iotaledger/iota.rs/blob/develop/types/CHANGELOG.md), [pow](https://github.com/iotaledger/iota.rs/blob/develop/pow/CHANGELOG.md), [client](https://github.com/iotaledger/iota.rs/blob/develop/client/CHANGELOG.md) and [wallet](https://github.com/iotaledger/wallet.rs/blob/develop/wallet/CHANGELOG.md).
