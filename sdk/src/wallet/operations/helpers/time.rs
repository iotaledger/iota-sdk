// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    types::block::{address::Address, output::Output, protocol::CommittableAgeRange, slot::SlotIndex},
    wallet::{types::OutputWithExtendedMetadata, WalletError},
};

// Check if an output can be unlocked by the wallet address at the current time
pub(crate) fn can_output_be_unlocked_now(
    wallet_address: &Address,
    output_with_ext_metadata: &OutputWithExtendedMetadata,
    commitment_slot_index: impl Into<SlotIndex> + Copy,
    committable_age_range: CommittableAgeRange,
) -> Result<bool, WalletError> {
    if let Some(unlock_conditions) = output_with_ext_metadata.output.unlock_conditions() {
        if unlock_conditions.is_timelocked(commitment_slot_index, committable_age_range.min) {
            return Ok(false);
        }
    }

    let required_address = output_with_ext_metadata
        .output
        .required_address(commitment_slot_index.into(), committable_age_range)?;

    // In case of `None` the output can currently not be unlocked because of expiration unlock condition
    Ok(required_address.map_or_else(|| false, |required_address| wallet_address == &required_address))
}

// Check if an output can be unlocked by the wallet address at the current time and at any
// point in the future
pub(crate) fn can_output_be_unlocked_forever_from_now_on(
    wallet_address: &Address,
    output: &Output,
    slot_index: impl Into<SlotIndex> + Copy,
    committable_age_range: CommittableAgeRange,
) -> bool {
    if let Some(unlock_conditions) = output.unlock_conditions() {
        if unlock_conditions.is_timelocked(slot_index, committable_age_range.min) {
            return false;
        }

        // If there is an expiration unlock condition, we can only unlock it forever from now on, if it's expired and
        // the return address belongs to the wallet
        if let Some(expiration) = unlock_conditions.expiration() {
            if let Some(address) = expiration.return_address_expired(
                // Safe to unwrap, if there is an expiration, then there also needs to be an address unlock condition
                unlock_conditions.address().unwrap().address(),
                slot_index,
                committable_age_range,
            ) {
                if address != expiration.return_address() || address != wallet_address {
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
