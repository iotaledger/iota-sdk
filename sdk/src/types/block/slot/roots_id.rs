// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

impl_id!(pub RootsId, 32, "The digest of multiple sparse merkle tree roots of a slot.");

#[cfg(feature = "serde")]
string_serde_impl!(RootsId);
