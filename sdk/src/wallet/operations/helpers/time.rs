// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use crate::{
    types::block::{address::Address, output::Output, protocol::CommittableAgeRange, slot::SlotIndex},
    wallet::{types::OutputData, WalletError},
};

// Check if an output can be unlocked by one of the provided addresses at the current time
pub(crate) fn can_output_be_unlocked_now(
    controlled_addresses: &HashSet<Address>,
    output_data: &OutputData,
    commitment_slot_index: impl Into<SlotIndex> + Copy,
    committable_age_range: CommittableAgeRange,
) -> Result<bool, WalletError> {
    if output_data
        .output
        .unlock_conditions()
        .is_timelocked(commitment_slot_index, committable_age_range.min)
    {
        return Ok(false);
    }

    let required_address = output_data
        .output
        .required_address(commitment_slot_index.into(), committable_age_range)?;

    // In case of `None` the output can currently not be unlocked because of expiration unlock condition
    Ok(required_address.map_or_else(
        || false,
        |required_address| controlled_addresses.contains(&required_address),
    ))
}

// Check if an output can be unlocked by the wallet address at the current time and at any
// point in the future
pub(crate) fn can_output_be_unlocked_from_now_on(
    controlled_addresses: &HashSet<Address>,
    output: &Output,
    slot_index: impl Into<SlotIndex> + Copy,
    committable_age_range: CommittableAgeRange,
) -> bool {
    if output
        .unlock_conditions()
        .is_timelocked(slot_index, committable_age_range.min)
    {
        return false;
    }

    // If there is an expiration unlock condition, we can only unlock it forever from now on, if it's expired and
    // the return address belongs to the wallet
    if let Some(expiration) = output.unlock_conditions().expiration() {
        if let Some(address) = expiration.return_address_expired(
            // Safe to unwrap, if there is an expiration, then there also needs to be an address unlock condition
            output.unlock_conditions().address().unwrap().address(),
            slot_index,
            committable_age_range,
        ) {
            if address != expiration.return_address() || !controlled_addresses.contains(address) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}
