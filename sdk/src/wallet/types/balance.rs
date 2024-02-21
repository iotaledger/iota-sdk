// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::{BTreeMap, BTreeSet};

use getset::{CopyGetters, Getters};
use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::{
    types::block::output::{
        feature::MetadataFeature, AccountId, DecayedMana, DelegationId, FoundryId, NftId, OutputId, TokenId,
    },
    utils::serde::string,
};

/// The balance of the wallet, returned from [`crate::wallet::core::Wallet::sync()`] and
/// [`crate::wallet::core::Wallet::balance()`].
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct Balance {
    /// Total and available amount of the base coin
    pub(crate) base_coin: BaseCoinBalance,
    /// Total and available mana
    pub(crate) mana: ManaBalance,
    /// Current required storage deposit amount
    pub(crate) required_storage_deposit: RequiredStorageDeposit,
    /// Native tokens
    pub(crate) native_tokens: BTreeMap<TokenId, NativeTokensBalance>,
    /// Accounts
    pub(crate) accounts: BTreeSet<AccountId>,
    /// Foundries
    pub(crate) foundries: BTreeSet<FoundryId>,
    /// Nfts
    pub(crate) nfts: BTreeSet<NftId>,
    /// Delegations
    pub(crate) delegations: BTreeSet<DelegationId>,
    /// Outputs with multiple unlock conditions and if they can currently be spent or not. If there is a
    /// [`TimelockUnlockCondition`](crate::types::block::output::unlock_condition::TimelockUnlockCondition) or
    /// [`ExpirationUnlockCondition`](crate::types::block::output::unlock_condition::ExpirationUnlockCondition) this
    /// can change at any time
    pub(crate) potentially_locked_outputs: BTreeMap<OutputId, bool>,
}

impl std::ops::AddAssign for Balance {
    fn add_assign(&mut self, rhs: Self) {
        self.base_coin += rhs.base_coin;
        self.required_storage_deposit += rhs.required_storage_deposit;

        for (token_id, rhs_native_token_balance) in rhs.native_tokens.into_iter() {
            *self.native_tokens.entry(token_id).or_default() += rhs_native_token_balance;
        }

        self.accounts.extend(rhs.accounts);
        self.foundries.extend(rhs.foundries);
        self.nfts.extend(rhs.nfts);
        self.delegations.extend(rhs.delegations);
    }
}

/// Base coin fields for [`Balance`]
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, CopyGetters, derive_more::AddAssign)]
#[serde(rename_all = "camelCase")]
#[getset(get_copy = "pub")]
pub struct BaseCoinBalance {
    /// Total amount
    #[serde(with = "string")]
    pub(crate) total: u64,
    /// Balance that can currently be spent
    #[serde(with = "string")]
    pub(crate) available: u64,
    /// Voting power
    #[cfg(feature = "participation")]
    #[serde(with = "string")]
    pub(crate) voting_power: u64,
}

/// Mana fields for [`Balance`]
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, Getters, derive_more::AddAssign)]
#[serde(rename_all = "camelCase")]
#[getset(get_copy = "pub")]
pub struct ManaBalance {
    /// Total mana.
    pub(crate) total: DecayedMana,
    /// Available mana.
    pub(crate) available: DecayedMana,
    /// Mana rewards.
    pub(crate) rewards: u64,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, CopyGetters, derive_more::AddAssign)]
#[getset(get_copy = "pub")]
pub struct RequiredStorageDeposit {
    #[serde(with = "crate::utils::serde::string")]
    pub(crate) basic: u64,
    #[serde(with = "crate::utils::serde::string")]
    pub(crate) account: u64,
    #[serde(with = "crate::utils::serde::string")]
    pub(crate) foundry: u64,
    #[serde(with = "crate::utils::serde::string")]
    pub(crate) nft: u64,
    #[serde(with = "crate::utils::serde::string")]
    pub(crate) delegation: u64,
}

/// Native tokens fields for [`Balance`]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Getters, CopyGetters)]
#[serde(rename_all = "camelCase")]
pub struct NativeTokensBalance {
    /// Total amount
    #[getset(get_copy = "pub")]
    pub(crate) total: U256,
    /// Balance that can currently be spent
    #[getset(get_copy = "pub")]
    pub(crate) available: U256,
    /// Token foundry immutable metadata
    #[getset(get = "pub")]
    pub(crate) metadata: Option<MetadataFeature>,
}

impl Default for NativeTokensBalance {
    fn default() -> Self {
        Self {
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

#[cfg(all(feature = "rand", feature = "protocol_parameters_samples"))]
impl Balance {
    pub fn rand() -> Self {
        use rand::Rng;

        use crate::types::block::rand::bytes::rand_bytes_array;

        let token_supply = crate::types::block::protocol::iota_mainnet_protocol_parameters().token_supply();
        let total = rand::thread_rng().gen_range(128..token_supply / 1000000);

        let mut generator = 0u8;
        // up to 10 fully random native token ids
        let native_tokens = std::iter::repeat_with(|| {
            let token_id = TokenId::from(rand_bytes_array());
            let total = rand::thread_rng().gen_range(1..10000u32);
            (
                token_id,
                NativeTokensBalance {
                    total: U256::from(total),
                    available: U256::from(rand::thread_rng().gen_range(1..total)),
                    ..Default::default()
                },
            )
        })
        .take(rand::thread_rng().gen_range(1..10))
        // up to 10 deterministic native token ids
        .chain(
            std::iter::repeat_with(|| {
                generator += 1;
                let token_id = TokenId::from([generator; TokenId::LENGTH]);
                let total = rand::thread_rng().gen_range(1..10000u32);
                (
                    token_id,
                    NativeTokensBalance {
                        total: U256::from(total),
                        available: U256::from(rand::thread_rng().gen_range(1..total)),
                        ..Default::default()
                    },
                )
            })
            .take(rand::thread_rng().gen_range(1..10)),
        )
        .collect();

        let accounts = std::iter::repeat_with(|| AccountId::from(rand_bytes_array()))
            .take(rand::thread_rng().gen_range(0..10))
            .collect();
        let nfts = std::iter::repeat_with(|| NftId::from(rand_bytes_array()))
            .take(rand::thread_rng().gen_range(0..10))
            .collect();
        let foundries = std::iter::repeat_with(|| FoundryId::from(rand_bytes_array()))
            .take(rand::thread_rng().gen_range(0..10))
            .collect();
        let delegations = std::iter::repeat_with(|| DelegationId::from(rand_bytes_array()))
            .take(rand::thread_rng().gen_range(0..10))
            .collect();

        Self {
            base_coin: BaseCoinBalance {
                total,
                available: total / 2,
                #[cfg(feature = "participation")]
                voting_power: total / 4,
            },
            required_storage_deposit: RequiredStorageDeposit {
                basic: total / 8,
                account: total / 16,
                foundry: total / 4,
                nft: total / 2,
                delegation: total / 16,
            },
            native_tokens,
            accounts,
            foundries,
            nfts,
            delegations,
            ..Default::default()
        }
    }
}
