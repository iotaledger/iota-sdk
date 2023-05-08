// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::types::block::{
    dto::U256Dto,
    output::{dto::TokenIdDto, feature::MetadataFeature, AliasId, FoundryId, NftId, OutputId, TokenId},
};

/// The balance of an account, returned from [`crate::wallet::account::Account::sync()`] and
/// [`crate::wallet::account::Account::balance()`].
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct AccountBalance {
    /// Total and available amount of the base coin
    pub(crate) base_coin: BaseCoinBalance,
    /// Current required storage deposit amount
    pub(crate) required_storage_deposit: RequiredStorageDeposit,
    /// Native tokens
    pub(crate) native_tokens: Vec<NativeTokensBalance>,
    /// Nfts
    pub(crate) nfts: Vec<NftId>,
    /// Aliases
    pub(crate) aliases: Vec<AliasId>,
    /// Foundries
    pub(crate) foundries: Vec<FoundryId>,
    /// Outputs with multiple unlock conditions and if they can currently be spent or not. If there is a
    /// [`TimelockUnlockCondition`](crate::types::block::output::unlock_condition::TimelockUnlockCondition) or
    /// [`ExpirationUnlockCondition`](crate::types::block::output::unlock_condition::ExpirationUnlockCondition) this
    /// can change at any time
    pub(crate) potentially_locked_outputs: HashMap<OutputId, bool>,
}

impl std::ops::AddAssign for AccountBalance {
    fn add_assign(&mut self, rhs: Self) {
        self.base_coin += rhs.base_coin;
        self.required_storage_deposit += rhs.required_storage_deposit;

        for native_token_balance in rhs.native_tokens.into_iter() {
            if let Some(total_native_token_balance) = self
                .native_tokens
                .iter_mut()
                .find(|n| n.token_id == native_token_balance.token_id)
            {
                *total_native_token_balance += native_token_balance;
            } else {
                self.native_tokens.push(native_token_balance);
            }
        }

        self.nfts.extend(rhs.nfts.into_iter());
        self.aliases.extend(rhs.aliases.into_iter());
        self.foundries.extend(rhs.foundries.into_iter());
    }
}

/// Dto for the balance of an account, returned from [`crate::wallet::account::Account::sync()`] and
/// [`crate::wallet::account::Account::balance()`].
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalanceDto {
    /// Total and available amount of the base coin
    pub base_coin: BaseCoinBalanceDto,
    /// Current required storage deposit amount
    pub required_storage_deposit: RequiredStorageDepositDto,
    /// Native tokens
    pub native_tokens: Vec<NativeTokensBalanceDto>,
    /// Nfts
    pub nfts: Vec<NftId>,
    /// Aliases
    pub aliases: Vec<AliasId>,
    /// Foundries
    pub foundries: Vec<FoundryId>,
    /// Outputs with multiple unlock conditions and if they can currently be spent or not. If there is a
    /// [`TimelockUnlockCondition`](crate::types::block::output::unlock_condition::TimelockUnlockCondition) or
    /// [`ExpirationUnlockCondition`](crate::types::block::output::unlock_condition::ExpirationUnlockCondition) this
    /// can change at any time
    pub potentially_locked_outputs: HashMap<OutputId, bool>,
}

impl From<&AccountBalance> for AccountBalanceDto {
    fn from(value: &AccountBalance) -> Self {
        Self {
            base_coin: BaseCoinBalanceDto::from(&value.base_coin),
            required_storage_deposit: RequiredStorageDepositDto::from(&value.required_storage_deposit),
            native_tokens: value
                .native_tokens
                .iter()
                .map(NativeTokensBalanceDto::from)
                .collect::<_>(),
            nfts: value.nfts.clone(),
            aliases: value.aliases.clone(),
            foundries: value.foundries.clone(),
            potentially_locked_outputs: value.potentially_locked_outputs.clone(),
        }
    }
}

/// Base coin fields for [`AccountBalance`]
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, CopyGetters)]
#[serde(rename_all = "camelCase")]
#[getset(get_copy = "pub")]
pub struct BaseCoinBalance {
    /// Total amount
    pub(crate) total: u64,
    /// Balance that can currently be spent
    pub(crate) available: u64,
    /// Voting power
    #[cfg(feature = "participation")]
    pub(crate) voting_power: u64,
}

impl std::ops::AddAssign for BaseCoinBalance {
    fn add_assign(&mut self, rhs: Self) {
        self.total += rhs.total;
        self.available += rhs.available;
        #[cfg(feature = "participation")]
        {
            self.voting_power += rhs.voting_power;
        }
    }
}

/// Base coin fields for [`AccountBalance`]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseCoinBalanceDto {
    /// Total amount
    pub total: String,
    /// Balance that can currently be spent
    pub available: String,
    /// Voting power
    #[cfg(feature = "participation")]
    pub voting_power: String,
}

impl From<&BaseCoinBalance> for BaseCoinBalanceDto {
    fn from(value: &BaseCoinBalance) -> Self {
        Self {
            total: value.total.to_string(),
            available: value.available.to_string(),
            #[cfg(feature = "participation")]
            voting_power: value.voting_power.to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct RequiredStorageDeposit {
    pub(crate) alias: u64,
    pub(crate) basic: u64,
    pub(crate) foundry: u64,
    pub(crate) nft: u64,
}

impl std::ops::AddAssign for RequiredStorageDeposit {
    fn add_assign(&mut self, rhs: Self) {
        self.alias += rhs.alias;
        self.basic += rhs.basic;
        self.foundry += rhs.foundry;
        self.nft += rhs.nft;
    }
}

/// DTO for [`RequiredStorageDeposit`].
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequiredStorageDepositDto {
    pub alias: String,
    pub basic: String,
    pub foundry: String,
    pub nft: String,
}

impl From<&RequiredStorageDeposit> for RequiredStorageDepositDto {
    fn from(value: &RequiredStorageDeposit) -> Self {
        Self {
            alias: value.alias.to_string(),
            basic: value.basic.to_string(),
            foundry: value.foundry.to_string(),
            nft: value.nft.to_string(),
        }
    }
}

/// Native tokens fields for [`AccountBalance`]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Getters, CopyGetters)]
#[serde(rename_all = "camelCase")]
pub struct NativeTokensBalance {
    /// Token id
    #[getset(get = "pub")]
    pub(crate) token_id: TokenId,
    /// Token foundry immutable metadata
    #[getset(get = "pub")]
    pub(crate) metadata: Option<MetadataFeature>,
    /// Total amount
    #[getset(get_copy = "pub")]
    pub(crate) total: U256,
    /// Balance that can currently be spent
    #[getset(get_copy = "pub")]
    pub(crate) available: U256,
}

impl Default for NativeTokensBalance {
    fn default() -> Self {
        Self {
            token_id: TokenId::null(),
            metadata: None,
            total: U256::from(0u8),
            available: U256::from(0u8),
        }
    }
}

impl std::ops::AddAssign for NativeTokensBalance {
    fn add_assign(&mut self, rhs: Self) {
        if self.metadata.is_none() {
            self.metadata = rhs.metadata;
        }
        self.total += rhs.total;
        self.available += rhs.available;
    }
}

/// Base coin fields for [`AccountBalanceDto`]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeTokensBalanceDto {
    /// Token id
    pub token_id: TokenIdDto,
    /// Token foundry immutable metadata
    pub metadata: Option<String>,
    /// Total amount
    pub total: U256Dto,
    /// Balance that can currently be spent
    pub available: U256Dto,
}

impl From<&NativeTokensBalance> for NativeTokensBalanceDto {
    fn from(value: &NativeTokensBalance) -> Self {
        Self {
            token_id: TokenIdDto::from(&value.token_id),
            metadata: value.metadata.as_ref().map(|m| prefix_hex::encode(m.data())),
            total: U256Dto::from(&value.total),
            available: U256Dto::from(&value.available),
        }
    }
}

impl AccountBalance {
    #[cfg(feature = "rand")]
    pub fn rand_mock() -> Self {
        use rand::Rng;

        use crate::types::block::rand::bytes::rand_bytes_array;

        let token_supply = crate::types::block::protocol::protocol_parameters().token_supply();
        let total = rand::thread_rng().gen_range(128..token_supply / 1000000);

        let native_tokens = std::iter::repeat_with(|| {
            let token_id = TokenId::from(rand_bytes_array());
            let total = rand::thread_rng().gen_range(1..10000u32);
            NativeTokensBalance {
                token_id,
                total: U256::from(total),
                available: U256::from(rand::thread_rng().gen_range(1..total)),
                ..Default::default()
            }
        })
        .take(rand::thread_rng().gen_range(0..10))
        .collect::<Vec<_>>();

        let aliases = std::iter::repeat_with(|| AliasId::from(rand_bytes_array()))
            .take(rand::thread_rng().gen_range(0..10))
            .collect::<Vec<_>>();
        let nfts = std::iter::repeat_with(|| NftId::from(rand_bytes_array()))
            .take(rand::thread_rng().gen_range(0..10))
            .collect::<Vec<_>>();
        let foundries = std::iter::repeat_with(|| FoundryId::from(rand_bytes_array()))
            .take(rand::thread_rng().gen_range(0..10))
            .collect::<Vec<_>>();

        Self {
            base_coin: BaseCoinBalance {
                total,
                available: total / 2,
                #[cfg(feature = "participation")]
                voting_power: total / 4,
            },
            required_storage_deposit: RequiredStorageDeposit {
                alias: total / 16,
                basic: total / 8,
                foundry: total / 4,
                nft: total / 2,
            },
            native_tokens,
            aliases,
            foundries,
            nfts,
            ..Default::default()
        }
    }
}
