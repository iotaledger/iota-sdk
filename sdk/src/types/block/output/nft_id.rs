// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    address::{Address, NftAddress},
    output::OutputId,
};

impl_id!(pub NftId, 32, "TODO.");

#[cfg(feature = "serde")]
string_serde_impl!(NftId);

impl From<&OutputId> for NftId {
    fn from(output_id: &OutputId) -> Self {
        Self::from(output_id.hash())
    }
}

impl NftId {
    ///
    pub fn or_from_output_id(self, output_id: &OutputId) -> Self {
        if self.is_null() { Self::from(output_id) } else { self }
    }
}

impl From<NftId> for Address {
    fn from(value: NftId) -> Self {
        Address::Nft(NftAddress::new(value))
    }
}
