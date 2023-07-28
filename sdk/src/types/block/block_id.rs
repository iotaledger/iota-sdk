// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

impl_id!(
    pub BlockId,
    40,
    "A Block ID denotes an identifier of a block.
    It created using the concatenation of the following:
    - The BLAKE2b-256 hash of concatenating the Block header hash, Block Hash and Serialized Signature
    - Serialized Slot Index
    See <https://www.blake2.net/> for more information."
);

#[cfg(feature = "serde")]
string_serde_impl!(BlockId);
