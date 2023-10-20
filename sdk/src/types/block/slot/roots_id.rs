// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

crate::impl_id!(
    /// The digest of multiple sparse merkle tree roots of a slot.
    pub RootsId {
        pub const LENGTH: usize = 32;
    }
);
