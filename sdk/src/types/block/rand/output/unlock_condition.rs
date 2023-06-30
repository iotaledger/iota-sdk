// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    address::Address,
    output::{
        unlock_condition::{
            AddressUnlockCondition, GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition,
        },
        AccountId, NftId,
    },
    rand::address::{rand_account_address, rand_address, rand_nft_address},
};

/// Generates a random [`AddressUnlockCondition`].
pub fn rand_address_unlock_condition() -> AddressUnlockCondition {
    rand_address().into()
}

/// Generates a random [`StateControllerAddressUnlockCondition`].
pub fn rand_state_controller_address_unlock_condition_different_from(
    alias_id: &AccountId,
) -> StateControllerAddressUnlockCondition {
    let mut address = rand_address();

    if let Address::Account(mut alias_address) = &mut address {
        while alias_address.alias_id() == alias_id {
            alias_address = rand_account_address();
        }
    }

    address.into()
}

/// Generates a random [`GovernorAddressUnlockCondition`] that is different from `alias_id`.
pub fn rand_governor_address_unlock_condition_different_from(alias_id: &AccountId) -> GovernorAddressUnlockCondition {
    let mut address = rand_address();

    if let Address::Account(mut alias_address) = &mut address {
        while alias_address.alias_id() == alias_id {
            alias_address = rand_account_address();
        }
    }

    address.into()
}

/// Generates a random [`AddressUnlockCondition`] that is different from `nft_id`.
pub fn rand_address_unlock_condition_different_from(nft_id: &NftId) -> AddressUnlockCondition {
    let mut address = rand_address();

    if let Address::Nft(mut nft_address) = &mut address {
        while nft_address.nft_id() == nft_id {
            nft_address = rand_nft_address();
        }
    }

    address.into()
}
