// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    types::block::{
        address::Address,
        output::{AccountTransition, Output},
        slot::SlotIndex,
    },
    wallet::account::types::{AddressWithUnspentOutputs, OutputData},
};

// Check if an output can be unlocked by one of the account addresses at the current time
pub(crate) fn can_output_be_unlocked_now(
    // We use the addresses with unspent outputs, because other addresses of the account without unspent outputs can't
    // be related to this output
    // TODO disambiguate these two parameters when we are done with Account changes https://github.com/iotaledger/iota-sdk/issues/647
    account_addresses: &[AddressWithUnspentOutputs],
    account_and_nft_addresses: &[Address],
    output_data: &OutputData,
    slot_index: SlotIndex,
    account_transition: Option<AccountTransition>,
) -> crate::wallet::Result<bool> {
    if let Some(unlock_conditions) = output_data.output.unlock_conditions() {
        if unlock_conditions.is_time_locked(slot_index) {
            return Ok(false);
        }
    }

    let (required_unlock_address, _unlocked_account_or_nft_address) = output_data
        .output
        .required_and_unlocked_address(slot_index, &output_data.output_id, account_transition)?;

    Ok(account_addresses
        .iter()
        .any(|a| a.address.inner == required_unlock_address)
        || account_and_nft_addresses.iter().any(|a| *a == required_unlock_address))
}

// Check if an output can be unlocked by one of the account addresses at the current time and at any
// point in the future
pub(crate) fn can_output_be_unlocked_forever_from_now_on(
    // We use the addresses with unspent outputs, because other addresses of the account without unspent outputs can't
    // be related to this output
    account_addresses: &[AddressWithUnspentOutputs],
    output: &Output,
    slot_index: SlotIndex,
) -> bool {
    if let Some(unlock_conditions) = output.unlock_conditions() {
        if unlock_conditions.is_time_locked(slot_index) {
            return false;
        }

        // If there is an expiration unlock condition, we can only unlock it forever from now on, if it's expired and
        // the return address belongs to the account
        if let Some(expiration) = unlock_conditions.expiration() {
            if let Some(return_address) = expiration.return_address_expired(slot_index) {
                if !account_addresses.iter().any(|a| a.address.inner == *return_address) {
                    return false;
                };
            } else {
                return false;
            }
        }

        true
    } else {
        false
    }
}
