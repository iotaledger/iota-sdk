// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Requirement, TransactionBuilder, TransactionBuilderError};
use crate::types::block::address::Address;

impl TransactionBuilder {
    /// Fulfills a sender requirement by selecting an available input that unlocks its address.
    pub(crate) fn fulfill_sender_requirement(&mut self, address: &Address) -> Result<(), TransactionBuilderError> {
        match address {
            Address::Ed25519(_) => {
                log::debug!("Treating {address:?} sender requirement as an ed25519 requirement");

                self.fulfill_ed25519_requirement(address).map_err(|e| match e {
                    TransactionBuilderError::UnfulfillableRequirement(Requirement::Ed25519(_)) => {
                        TransactionBuilderError::UnfulfillableRequirement(Requirement::Sender(address.clone()))
                    }
                    e => e,
                })
            }
            Address::Account(account_address) => {
                log::debug!("Treating {address:?} sender requirement as an account requirement");

                // A state transition is required to unlock the account address.
                self.fulfill_account_requirement(account_address.into_account_id())
                    .map_err(|e| match e {
                        TransactionBuilderError::UnfulfillableRequirement(Requirement::Account(_)) => {
                            TransactionBuilderError::UnfulfillableRequirement(Requirement::Sender(address.clone()))
                        }
                        e => e,
                    })
            }
            Address::Nft(nft_address) => {
                log::debug!("Treating {address:?} sender requirement as an nft requirement");

                self.fulfill_nft_requirement(nft_address.into_nft_id())
                    .map_err(|e| match e {
                        TransactionBuilderError::UnfulfillableRequirement(Requirement::Nft(_)) => {
                            TransactionBuilderError::UnfulfillableRequirement(Requirement::Sender(address.clone()))
                        }
                        e => e,
                    })
            }
            // TODO https://github.com/iotaledger/iota-sdk/issues/1721
            Address::Multi(multi_address) => {
                let mut cumulative_weight = 0;

                for weight_address in multi_address.addresses() {
                    for input in self.selected_inputs.iter() {
                        let required_address = input
                            .output
                            .required_address(
                                self.latest_slot_commitment_id.slot_index(),
                                self.protocol_parameters.committable_age_range(),
                            )?
                            .expect("expiration unlockable outputs already filtered out");

                        if &required_address == weight_address.address() {
                            cumulative_weight += weight_address.weight() as u16;
                            break;
                        }
                    }

                    if cumulative_weight >= multi_address.threshold() {
                        break;
                    }
                }

                if cumulative_weight < multi_address.threshold() {
                    Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Sender(
                        address.clone(),
                    )))
                } else {
                    Ok(())
                }
            }
            Address::Restricted(restricted_address) => {
                log::debug!("Forwarding {address:?} sender requirement to inner address");

                self.fulfill_sender_requirement(restricted_address.address())
            }
            _ => Err(TransactionBuilderError::UnsupportedAddressType(
                address.kind_str().to_owned(),
            )),
        }
    }
}