// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod output_ids;
mod outputs;

use std::collections::HashSet;

use crate::wallet::account::{operations::syncing::SyncOptions, types::address::AddressWithUnspentOutputs, Account};

impl Account {
    /// Get the addresses that should be synced with the current known unspent output ids
    /// Also adds alias and nft addresses from unspent alias or nft outputs that have no Timelock, Expiration or
    /// StorageDepositReturn [`UnlockCondition`]
    pub(crate) async fn get_addresses_to_sync(
        &self,
        options: &SyncOptions,
    ) -> crate::wallet::Result<Vec<AddressWithUnspentOutputs>> {
        log::debug!("[SYNC] get_addresses_to_sync");

        let mut addresses_before_syncing = self.addresses().await?;

        // If custom addresses are provided check if they are in the account and only use them
        if !options.addresses.is_empty() {
            let mut specific_addresses_to_sync = HashSet::new();
            for bech32_address in &options.addresses {
                match addresses_before_syncing.iter().find(|a| &a.address == bech32_address) {
                    Some(address) => {
                        specific_addresses_to_sync.insert(address.clone());
                    }
                    None => {
                        return Err(crate::wallet::Error::AddressNotFoundInAccount(bech32_address.clone()));
                    }
                }
            }
            addresses_before_syncing = specific_addresses_to_sync.into_iter().collect();
        } else if options.address_start_index != 0 || options.address_start_index_internal != 0 {
            // Filter addresses when address_start_index(_internal) is not 0, so we skip these addresses
            addresses_before_syncing.retain(|a| {
                if a.internal {
                    a.key_index >= options.address_start_index_internal
                } else {
                    a.key_index >= options.address_start_index
                }
            });
        }

        // Check if selected addresses contains addresses with balance so we can correctly update them
        let addresses_with_unspent_outputs = self.addresses_with_unspent_outputs().await?;
        let mut addresses_with_old_output_ids = Vec::new();
        for address in addresses_before_syncing {
            let mut output_ids = Vec::new();
            // Add currently known unspent output ids, so we can later compare them with the new output ids and see if
            // one got spent (is missing in the new returned output ids)
            if let Some(address_with_unspent_outputs) = addresses_with_unspent_outputs
                .iter()
                .find(|a| a.address == address.address)
            {
                output_ids = address_with_unspent_outputs.output_ids.to_vec();
            }
            addresses_with_old_output_ids.push(AddressWithUnspentOutputs {
                address: address.address,
                key_index: address.key_index,
                internal: address.internal,
                output_ids,
            })
        }

        Ok(addresses_with_old_output_ids)
    }
}
