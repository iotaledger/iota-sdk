// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Requirement, TransactionBuilder, TransactionBuilderError};
use crate::types::block::address::Address;

impl TransactionBuilder {
    /// Fulfills an issuer requirement by fulfilling the equivalent sender requirement.
    /// Potentially converts the error for a more accurate one.
    pub(crate) fn fulfill_issuer_requirement(&mut self, address: &Address) -> Result<(), TransactionBuilderError> {
        log::debug!("Treating {address:?} issuer requirement as a sender requirement");

        self.fulfill_sender_requirement(address).map_err(|e| match e {
            TransactionBuilderError::UnfulfillableRequirement(Requirement::Sender(_)) => {
                TransactionBuilderError::UnfulfillableRequirement(Requirement::Issuer(address.clone()))
            }
            e => e,
        })
    }
}
