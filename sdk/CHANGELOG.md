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

## 0.4.0 - 2023-07-14

### Added

- `FilterOptions::{alias_ids, foundry_ids, nft_ids}` fields;
- `Account::{unspent_alias_output, unspent_foundry_output, unspent_nft_output}` methods;
- `StrongholdAdapter::inner` method;
- `OutputMetadata::set_spent` method;
- `ignore_if_bech32_mismatch` parameter to `Wallet::restore_backup()`;
- `OutputWithMetadata::{into_output, into_metadata}` methods;
- Storage and Backup migration;
- `types::block::Error::InvalidFoundryZeroSerialNumber` variant;
- `Hrp` type to represent a valid bech32 human-readable part;
- Multiple prepare methods returning `PreparedTransactionData`: `prepare_consolidate_outputs`, `prepare_vote`, `prepare_stop_participating`, `prepare_increase_voting_power`, `prepare_decrease_voting_power`, `prepare_decrease_native_token_supply` and `prepare_burn`;
- Multiple prepare methods returning `PreparedMintTokenTransaction`: `prepare_mint_native_token` and `prepare_increase_native_token_supply`;
- Stronghold snapshot migration from v2 to v3;
- `SecretManage::sign_evm`;
- `Account::addresses_balance` method accepting addresses to get balance for;
- `Wallet::get_secret_manager` method;
- `Password` type which is `Zeroize` and `ZeroizeOnDrop`;
- `TransactionOptions` parameter to `Account::{sign_and_submit_transaction, submit_and_store_transaction}`;
- Support for `LedgerSecretManager::sign_ed25519`;
- `UnlockCondition::{is_address, is_storage_deposit_return, is_timelock, is_expiration, is_state_controller_address, is_governor_address, is_immutable_alias_address}`;
- `UnlockCondition::{as_address, as_storage_deposit_return, as_timelock, as_expiration, as_state_controller_address, as_governor_address, as_immutable_alias_address}`;
- `ClientInner::call_plugin_route` to Client to fetch data from custom node plugins;
- `WalletBuilder::with_storage_options` method, allowing storage encryption;
- `StorageOptions::{new, with_encryption_key}` methods and getters;
- `MinimumStorageDepositBasicOutput`;
- `LedgerSecretManager::non_interactive` field;

### Changed

- `OutputData::metadata` changed from `OutputMetadataDto` to `OutputMetadata`;
- Rename messages `SendAmount::addresses_with_amount`, `SendNft::addresses_and_nft_ids`, `SendNativeTokens::addresses_and_native_tokens`, `CreateAliasOutput::alias_output_options`, `MintNfts::nftsOptions`, `MintNativeToken::native_token_options`, `PrepareOutput::options` to `params`.
- Rename `AddressesWithAmount` to `SendAmountParams`.
- Rename `AddressesAndNftIds` to `SendNftParams`.
- Rename `AddressesAndNativeTokens` to `SendNativeTokensParams`.
- Rename `AliasOutputOptions` to `CreateAliasParams`.
- Rename `NftOptions` to `MintNftParams`.
- Rename `NativeTokenOptions` to `MintNativeTokenParams`.
- Rename `OutputOptions` to `OutputParams`.
- `Client::get_outputs()` and derived methods return `OutputWithMetadata` instead of `OutputWithMetadataDto`;
- rename `Client::try_get_outputs()` into `Client::get_outputs_ignore_errors()`;
- rename `Client::try_get_outputs_metadata()` into `Client::get_outputs_metadata_ignore_errors()`;
- All `Node` related errors have been moved from the client error to a newly created `client::node_api::Error`;
- MQTT connections to a node using https will now use wss/tls with native certificates;
- `ClientBuilder::finish` is now async;
- Features and UnlockConditions that take an `Address` now take `impl Into<Address>`;
- Functions that accepted a string bech32 address now accept a `Bech32Address`;
- Functions that accepted a string bech32 HRP now accept an `Hrp`;
- `Account::read` and `write` now accessible via `details` and `details_mut`;
- `Wallet::emit_test_event` no longer returns a `Result`;
- `Client`, `Wallet`, and `Account` fns updated to reflect internal reorganization;
- `AccountBuilder::new` now takes a wallet;
- `InputSigningDataDto::chain` is now `Vec<u32>` instead of `Chain`;
- Most `StrongholdAdapter` fns no longer require a mutable reference;
- `StorageProvider` fns no longer require a mutable reference;
- `Account::burn_native_tokens()`, `Account::burn_nft()`, `Account::destroy_foundry()`, `Account::destroy_alias()` merged into `Account::burn()`;
- `Account::burn_native_tokens()`, `Account::burn_nft()`, `Account::destroy_foundry()`, `Account::destroy_alias()` merged into `Account::burn()`
- `ValidationContext::{input_native_tokens, output_native_tokens}` from HashMap to BTreeMap;
- Rename `AccountInner::get_incoming_transaction_data` to `get_incoming_transaction`;
- `AccountInner::{incoming_transactions, transactions, pending_transactions}` don't return a `Result` anymore;
- `AccountInner::incoming_transactions` returns a `Vec` instead of a `HashMap`;
- `Address::try_from_bech32_with_hrp` refactored to `try_from_bech32`;
- `{MetadataFeature, TagFeature}::new` take an `impl Into<Vec<u8>>` param;
- Merged `StorageProvider` into `StorageAdapter`;
- `GetAddressesBuilderOptions` renamed to `GetAddressesOptions` and fields no longer nullable;
- Methods on `GetAddressesBuilder` moved to `SecretManager`;
- Rename `GenerateAddresses` method to `GenerateEd25519Addresses` for Account and SecretManager, and their respective responses;
- Rename `SecretManager` and `SecretManage` ed25519 address generation methods;
- `SecretManage::generate_ed25519_addresses` returns `Ed25519Address` type;
- Made certain `prepare_` methods public: `prepare_mint_nfts`, `prepare_send_native_tokens`, `prepare_send_nft` and `prepare_create_alias_output`;
- `Wallet`, `WalletBuilder`, `Account`, `AccountBuilder` now specify generic secret manager type;
- `Address`-like types now implement `ToBech32Ext` for `to_bech32` and similar fns;
- Add constructors for `SendNftParams`, `SendAmountParams`, `SendNativeTokensParams`, `MintNftParams`;
- Rename `AccountBalance` to `Balance` and `AccountBalanceDto` to `BalanceDto`:
- `Bech32AddressLike`, `HrpLike` and other `TryInto` parameters unified with `ConvertTo` trait;
- Custom `Serialize` and `Deserialize` impls for `WalletEvent` to have an integer `type` as tag;
- `WalletEventType` now converts to/from u8 instead of string;
- `Client` methods `get_outputs`, `get_outputs_ignore_errors`, `get_outputs_metadata_ignore_errors` now accept a slice of output ids;
- More functions accept generic types for ergonomics: `Wallet::listen`, `clear_listeners`, `EventEmitter` fns, `RegularTransactionEssenceBuilder` fns, `AliasOutputBuilder` fns, `Account::claim_outputs`, `prepare_transaction`, `send`, `finish_transaction`, `send_nft`, `prepare_send_nft`, `send_native_tokens`, `prepare_send_native_tokens`, `send_amount`, `prepare_send_amount`, `mint_nfts`, `prepare_mint_nfts`, `vote`, `prepare_vote`, `Unlocks::new`, `TaggedDataPayload::new`, `MilestonePayload::new`, `ReceiptMilestoneOption::new`, `Client::subscribe`, `unsubscribe`, `basic_output_ids`, `alias_output_ids`, `foundry_output_ids`, `nft_output_ids`, `MqttManager::with_topics`, `MqttTopicManager::new`, `with_topics`, `QueryParameters::new`;
- `Topic::try_new` renamed to `new`, `topic` renamed to `as_str`;
- `LedgerNanoStatus::locked` is now optional since it's an IOTA/Shimmer specific API;
- `StorageManager` and wallet dynamic `StorageAdapter` are now private;
- All public password-related methods now claim ownership over provided passwords and take care of zeroing the memory on drop;
- Dto type conversion to represented type now always takes owned data;
- Rename `WalletOptions::build_manager` to `build`;
- `PeerDto` renamed to `PeerResponse`, `ReceiptDto` to `ReceiptResponse`, `LedgerInclusionStateDto` to `LedgerInclusionState`, `HeartbeatDto` to `Heartbeat`, `MetricsDto` tp `Metrics`, `GossipDto` to `Gossip`, `RelationDto` to `Relation`;
- Default number of workers for nonce `Miner` changed from `1` to `num_cpu::get()`;
- Made `Account::get_basic_outputs_for_additional_inputs` private;
- `Account::get_unlockable_outputs_with_additional_unlock_conditions` renamed to `claimable_outputs`;
- Use concrete ID types instead of String in HTTP responses;
- `Client::get_outputs_metadata_ignore_errors` returns `OutputMetadata` instead of DTO;
- `ClientInner::get_output_metadata` returns `OutputMetadata` instead of DTO;
- Rename `Account::mint_native_token` to `create_native_token`, `Account::increase_native_token_supply` to `mint_native_token`, `Account::decrease_native_token_supply` to `melt_native_token`;
- Rename `Account::prepare_mint_native_token` to `prepare_create_native_token`, `Account::prepare_increase_native_token_supply` to `prepare_mint_native_token`, `Account::prepare_decrease_native_token_supply` to `prepare_melt_native_token`;
- Rename `MintNativeTokenParams` to `CreateNativeTokenParams`;
- Rename `MintNativeTokenTransaction` to `CreateNativeTokenTransaction` and `PreparedMintNativeTokenTransaction` to `PreparedCreateNativeTokenTransaction` (including their corresponding DTOs);
- `Signature::Ed25519` now holds a boxed type;
- `Ed25519Signature::new` renamed to `try_from_bytes` and returns a Result;
- `Ed25519Signature::new`, `public_key`, `signature` now use concrete types;
- `Ed25519Signature::verify` is no longer fallable;
- `Mnemonic` type used over Strings where possible;
- `SecretManage::sign_ed25519`, `sign_secp256k1_ecdsa`, and `signature_unlock` now accept Bip44 type chains;
- Rename `SendAmountParams` to `SendParams`;
- Rename `Account::send` to `send_outputs`, `Account::send_amount` to `send`, `Account::prepare_send_amount` to `prepare_send`;
- Made `ManagerStorage` public and renamed it to `StorageKind`;
- Made `StorageOptions` public;
- Renamed `Client::block` to `build_block`;
- Renamed "inception" modules to `core` (ex. `wallet::wallet` -> `wallet::core`);

### Removed

- `FilterOptions`'s `Hash` derivation;
- `client_without_tls` feature in favor of separate `client` and `tls` features;
- `IncreaseNativeTokenSupplyOptions`;
- `HARDENED` const;
- `AliasIdDto`, `NftIdDto` and `TokenIdDto`;
- `U256Dto`, `SendAmountParamsDto`, `AddressWithUnspentOutputsDto`, `RequiredStorageDepositDto` and `BaseCoinBalanceDto`;
- `GetAddressesBuilder`;
- Excess `SecretManager` address generation methods;
- `Bech32Addresses` and `RawAddresses`;
- `Client::get_addresses`;
- `StorageAdapterId`;
- `Topic` `TryFrom<String>` impl;
- `Client::generate_ed25519_addresses`
- `Wallet::get_node_info`
- `NativeTokenDto`, which required a migration;
- `RentStructureDto`, `CreateAliasParamsDto`, `AssetsDto`, `OutputParamsDto`, `MintNativeTokenParamsDto` and `MintNftParamsDto`;
- `NativeTokensBalanceDto` and `BalanceDto`;
- `RentStructureBuilder`;
- `PlaceholderSecretManager`;
- `block::Error::{InvalidControllerKind, MigratedFundsNotSorted, MissingPayload, MissingRequiredSenderBlock}` variants;
- `client::Error::InvalidBIP32ChainData`;
- `BlockResponse`, `OutputResponse` and `MilestoneResponse`;
- `ClientError::UnexpectedApiResponse`;
- `HD_WALLET_TYPE` constant;

### Fixed

- Storage records decryption;
- CoinType check, by moving it from AccountBuilder to WalletBuilder;
- Validation for transitions in the input selection;
- Automatically increase foundry counter of alias outputs;
- Validate that foundry outputs can't have serial number `0`;
- Allow QueryParameter::Issuer for NFTs;

## 0.3.0 - 2023-05-02

### Added

- `NativeTokensBuilder::finish_set`;
- `Features`, `UnlockConditions`, `NativeTokens`, `MilestoneOptions`, and `Parents` added `from_set`;
- `types::block::Error::InvalidField` variant;
- `StorageProvider` and `SecretManage` have an `Error` associated type;
- `SecretManageExt` is a super trait of `SecretManage`;
- `OutputsToClaim::Amount` to allow claiming only outputs that will add an amount to the account;
- `Account::{set_default_sync_options, default_sync_options}` methods;
- `Wallet::get_client` method;
- `Wallet::get_account_aliases` method;

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
- `OutputsToClaim::None` variant;

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
