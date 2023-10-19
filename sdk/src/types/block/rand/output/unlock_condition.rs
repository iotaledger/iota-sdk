// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    address::Address,
    output::{
        unlock_condition::{
            AddressUnlockCondition, GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition,
        },
        AccountId, AnchorId, NftId,
    },
    rand::address::{rand_account_address, rand_address, rand_anchor_address, rand_nft_address},
};

/// Generates a random [`AddressUnlockCondition`].
pub fn rand_address_unlock_condition() -> AddressUnlockCondition {
    rand_address().into()
}

/// Generates a random [`StateControllerAddressUnlockCondition`] that is different from `anchor_id`.
pub fn rand_state_controller_address_unlock_condition_different_from(
    anchor_id: &AnchorId,
) -> StateControllerAddressUnlockCondition {
    let mut address = rand_address();

    if let Address::Anchor(mut account_address) = &mut address {
        while account_address.anchor_id() == anchor_id {
            account_address = rand_anchor_address();
        }
    }

    address.into()
}

/// Generates a random [`GovernorAddressUnlockCondition`] that is different from `anchor_id`.
pub fn rand_governor_address_unlock_condition_different_from(anchor_id: &AnchorId) -> GovernorAddressUnlockCondition {
    let mut address = rand_address();

    if let Address::Anchor(mut account_address) = &mut address {
        while account_address.anchor_id() == anchor_id {
            account_address = rand_anchor_address();
        }
    }

    address.into()
}

/// Generates a random [`AddressUnlockCondition`] that is different from `account_id`.
pub fn rand_address_unlock_condition_different_from_account_id(account_id: &AccountId) -> AddressUnlockCondition {
    let mut address = rand_address();

    if let Address::Account(mut account_address) = &mut address {
        while account_address.account_id() == account_id {
            account_address = rand_account_address();
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
