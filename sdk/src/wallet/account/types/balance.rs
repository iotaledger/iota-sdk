// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::types::block::output::{feature::MetadataFeature, AliasId, FoundryId, NftId, OutputId, TokenId};

/// The balance of an account, returned from [`crate::wallet::account::Account::sync()`] and
/// [`crate::wallet::account::Account::balance()`].
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct Balance {
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

impl std::ops::AddAssign for Balance {
    fn add_assign(&mut self, rhs: Self) {
        self.base_coin += rhs.base_coin;
        self.required_storage_deposit += rhs.required_storage_deposit;

        for rhs_native_token_balance in rhs.native_tokens.into_iter() {
            if let Some(total_native_token_balance) = self
                .native_tokens
                .iter_mut()
                .find(|lhs_native_token_balance| lhs_native_token_balance.token_id == rhs_native_token_balance.token_id)
            {
                *total_native_token_balance += rhs_native_token_balance;
            } else {
                self.native_tokens.push(rhs_native_token_balance);
            }
        }

        self.nfts.extend(rhs.nfts);
        self.aliases.extend(rhs.aliases);
        self.foundries.extend(rhs.foundries);
    }
}

/// Base coin fields for [`Balance`]
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, CopyGetters)]
#[serde(rename_all = "camelCase")]
#[getset(get_copy = "pub")]
pub struct BaseCoinBalance {
    /// Total amount
    #[serde(with = "crate::utils::serde::string")]
    pub(crate) total: u64,
    /// Balance that can currently be spent
    #[serde(with = "crate::utils::serde::string")]
    pub(crate) available: u64,
    /// Voting power
    #[cfg(feature = "participation")]
    #[serde(with = "crate::utils::serde::string")]
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

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct RequiredStorageDeposit {
    #[serde(with = "crate::utils::serde::string")]
    pub(crate) alias: u64,
    #[serde(with = "crate::utils::serde::string")]
    pub(crate) basic: u64,
    #[serde(with = "crate::utils::serde::string")]
    pub(crate) foundry: u64,
    #[serde(with = "crate::utils::serde::string")]
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

/// Native tokens fields for [`Balance`]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Getters, CopyGetters)]
#[serde(rename_all = "camelCase")]
pub struct NativeTokensBalance {
    /// Token id
    #[getset(get = "pub")]
    pub(crate) token_id: TokenId,
    /// Total amount
    #[getset(get_copy = "pub")]
    pub(crate) total: U256,
    /// Balance that can currently be spent
    #[getset(get_copy = "pub")]
    pub(crate) available: U256,
    /// Token foundry immutable metadata
    #[getset(get = "pub")]
    #[serde(with = "crate::utils::serde::option_string")]
    pub(crate) metadata: Option<MetadataFeature>,
}

impl Default for NativeTokensBalance {
    fn default() -> Self {
        Self {
            token_id: TokenId::null(),
            total: U256::from(0u8),
            available: U256::from(0u8),
            metadata: None,
        }
    }
}

impl std::ops::AddAssign for NativeTokensBalance {
    fn add_assign(&mut self, rhs: Self) {
        self.total += rhs.total;
        self.available += rhs.available;
        if self.metadata.is_none() {
            self.metadata = rhs.metadata;
        }
    }
}

#[cfg(feature = "rand")]
impl Balance {
    pub fn rand_mock() -> Self {
        use rand::Rng;

        use crate::types::block::rand::bytes::rand_bytes_array;

        let token_supply = crate::types::block::protocol::protocol_parameters().token_supply();
        let total = rand::thread_rng().gen_range(128..token_supply / 1000000);

        let mut generator = 0u8;
        // up to 10 fully random native token ids
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
        .take(rand::thread_rng().gen_range(1..10))
        // up to 10 deterministic native token ids
        .chain(
            std::iter::repeat_with(|| {
                generator += 1;
                let token_id = TokenId::from([generator; TokenId::LENGTH]);
                let total = rand::thread_rng().gen_range(1..10000u32);
                NativeTokensBalance {
                    token_id,
                    total: U256::from(total),
                    available: U256::from(rand::thread_rng().gen_range(1..total)),
                    ..Default::default()
                }
            })
            .take(rand::thread_rng().gen_range(1..10)),
        )
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
