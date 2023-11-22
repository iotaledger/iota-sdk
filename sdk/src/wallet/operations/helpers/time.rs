// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    types::block::{address::Address, output::Output, slot::SlotIndex},
    wallet::types::OutputData,
};

// Check if an output can be unlocked by the wallet address at the current time
pub(crate) fn can_output_be_unlocked_now(
    wallet_address: &Address,
    output_data: &OutputData,
    slot_index: SlotIndex,
    min_committable_age: SlotIndex,
    max_committable_age: SlotIndex,
) -> crate::wallet::Result<bool> {
    if let Some(unlock_conditions) = output_data.output.unlock_conditions() {
        if unlock_conditions.is_timelocked(slot_index, min_committable_age) {
            return Ok(false);
        }
    }

    let required_address = output_data
        .output
        .required_address(slot_index, min_committable_age, max_committable_age)?;

    // In case of `None` the output can currently not be unlocked because of expiration unlock condition
    required_address.map_or_else(|| Ok(false), |required_address| Ok(wallet_address == &required_address))
}

// Check if an output can be unlocked by one of the wallet address at the current time and at any
// point in the future
pub(crate) fn can_output_be_unlocked_forever_from_now_on(
    wallet_address: &Address,
    output: &Output,
    slot_index: SlotIndex,
    min_committable_age: SlotIndex,
    max_committable_age: SlotIndex,
) -> bool {
    if let Some(unlock_conditions) = output.unlock_conditions() {
        if unlock_conditions.is_timelocked(slot_index, min_committable_age) {
            return false;
        }

        // TODO HELP
        // If there is an expiration unlock condition, we can only unlock it forever from now on, if it's expired and
        // the return address belongs to the wallet
        if let Some(expiration) = unlock_conditions.expiration() {
            if let Some(address) = expiration.return_address_expired(
                // Safe to unwrap, if there is an expiration, then there also needs to be an address unlock condition
                unlock_conditions.address().unwrap().address(),
                slot_index,
                min_committable_age,
                max_committable_age,
            ) {
                if address == expiration.return_address() {
                    if wallet_address != address {
                        return false;
                    };
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    } else {
        false
    }
}
