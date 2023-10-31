// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

crate::impl_id!(
    /// The hash of a [`Block`](crate::types::block::Block).
    pub BlockHash {
        pub const LENGTH: usize = 32;
    }
    /// A [`Block`](crate::types::block::Block) identifier.
    pub BlockId;
);
