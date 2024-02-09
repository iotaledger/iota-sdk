// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, InputSelection, Requirement};
use crate::{client::secret::types::InputSigningData, types::block::address::Address};

impl InputSelection {
    /// Fulfills a sender requirement by selecting an available input that unlocks its address.
    pub(crate) fn fulfill_sender_requirement(&mut self, address: &Address) -> Result<Vec<InputSigningData>, Error> {
        match address {
            Address::Ed25519(_) => {
                log::debug!("Treating {address:?} sender requirement as an ed25519 requirement");

                match self.fulfill_ed25519_requirement(address) {
                    Ok(res) => Ok(res),
                    Err(Error::UnfulfillableRequirement(Requirement::Ed25519(_))) => {
                        Err(Error::UnfulfillableRequirement(Requirement::Sender(address.clone())))
                    }
                    Err(e) => Err(e),
                }
            }
            Address::Account(account_address) => {
                log::debug!("Treating {address:?} sender requirement as an account requirement");

                // A state transition is required to unlock the account address.
                match self.fulfill_account_requirement(account_address.into_account_id()) {
                    Ok(res) => Ok(res),
                    Err(Error::UnfulfillableRequirement(Requirement::Account(_))) => {
                        Err(Error::UnfulfillableRequirement(Requirement::Sender(address.clone())))
                    }
                    Err(e) => Err(e),
                }
            }
            Address::Nft(nft_address) => {
                log::debug!("Treating {address:?} sender requirement as an nft requirement");

                match self.fulfill_nft_requirement(nft_address.into_nft_id()) {
                    Ok(res) => Ok(res),
                    Err(Error::UnfulfillableRequirement(Requirement::Nft(_))) => {
                        Err(Error::UnfulfillableRequirement(Requirement::Sender(address.clone())))
                    }
                    Err(e) => Err(e),
                }
            }
            Address::Restricted(restricted_address) => {
                log::debug!("Forwarding {address:?} sender requirement to inner address");

                self.fulfill_sender_requirement(restricted_address.address())
            }
            _ => Err(Error::UnsupportedAddressType(address.kind())),
        }
    }
}
