// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{output::OutputId, Error};

crate::impl_id!(
    /// The hash of a [`Transaction`](crate::types::block::payload::signed_transaction::Transaction).
    pub TransactionHash {
        pub const LENGTH: usize = 32;
    }
    /// A [`Transaction`](crate::types::block::payload::signed_transaction::Transaction) identifier.
    pub TransactionId;
);

impl TransactionId {
    /// Creates an [`OutputId`] from this [`TransactionId`] and an output index.
    pub fn into_output_id(self, index: u16) -> Result<OutputId, Error> {
        OutputId::new(self, index)
    }
}

crate::impl_id!(
    /// The hash of a transaction commitment and output commitment which is used to create
    /// [`SignedTransactionPayload`](crate::types::block::payload::SignedTransactionPayload).
    pub TransactionSigningHash {
        pub const LENGTH: usize = 32;
    }
);
