// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

impl_id!(
    pub IssuerId,
    32,
    "Identifier of a block issuer."
);

#[cfg(feature = "serde")]
string_serde_impl!(IssuerId);
