// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, InputSelection, Requirement};
use crate::{client::secret::types::InputSigningData, types::block::address::Address};

impl InputSelection {
    /// Fulfills an issuer requirement by fulfilling the equivalent sender requirement.
    /// Potentially converts the error for a more accurate one.
    pub(crate) fn fulfill_issuer_requirement(&mut self, address: &Address) -> Result<Vec<InputSigningData>, Error> {
        log::debug!("Treating {address:?} issuer requirement as a sender requirement");

        match self.fulfill_sender_requirement(address) {
            Ok(res) => Ok(res),
            Err(Error::UnfulfillableRequirement(Requirement::Sender(_))) => {
                Err(Error::UnfulfillableRequirement(Requirement::Issuer(address.clone())))
            }
            Err(e) => Err(e),
        }
    }
}
