// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{output::OutputId, Error};

crate::impl_id_with_slot!(
    @explicit_docs
    pub Transaction, 32,
    "A [`TransactionPayload`] identifier.",
    "The hash of a [`TransactionPayload`]."
);

impl TransactionId {
    /// Creates an [`OutputId`] from this [`TransactionId`] and an output index.
    pub fn with_output_index(self, index: u16) -> Result<OutputId, Error> {
        OutputId::new(self, index)
    }
}
