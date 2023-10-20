// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{output::OutputId, Error};

crate::impl_id!(
    /// The hash of a [`TransactionPayload`](crate::types::block::TransactionPayload).
    pub TransactionHash {
        pub const LENGTH: usize = 32;
    }
    /// A [`TransactionPayload`](crate::types::block::TransactionPayload) identifier.
    pub TransactionId;
);

impl TransactionId {
    /// Creates an [`OutputId`] from this [`TransactionId`] and an output index.
    pub fn with_output_index(self, index: u16) -> Result<OutputId, Error> {
        OutputId::new(self, index)
    }
}
