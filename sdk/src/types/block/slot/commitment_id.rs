// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

impl_id!(pub SlotCommitmentId, 40, "Identifier of a slot commitment.");

#[cfg(feature = "serde")]
string_serde_impl!(SlotCommitmentId);
