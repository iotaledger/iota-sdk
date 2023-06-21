// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

impl_id!(pub RootsId, 32, "A BLAKE2b-256 hash of concatenating multiple sparse merkle tree roots of a slot.");

#[cfg(feature = "serde")]
string_serde_impl!(RootsId);
