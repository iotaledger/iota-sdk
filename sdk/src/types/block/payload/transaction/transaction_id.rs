// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{output::OutputId, Error};

impl_id_with_slot!(
    pub TransactionHash, 32, "The hash of a [`TransactionPayload`].",
    pub TransactionId, "A [`TransactionPayload`] identifier."
);

impl TransactionId {
    /// Creates an [`OutputId`] from this [`TransactionId`] and an output index.
    pub fn with_output_index(self, index: u16) -> Result<OutputId, Error> {
        OutputId::new(self, index)
    }
}
