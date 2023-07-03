// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    address::{AccountAddress, Address},
    output::OutputId,
};

impl_id!(pub AccountId, 32, "TODO.");

#[cfg(feature = "serde")]
string_serde_impl!(AccountId);

impl From<&OutputId> for AccountId {
    fn from(output_id: &OutputId) -> Self {
        Self::from(output_id.hash())
    }
}

impl AccountId {
    ///
    pub fn or_from_output_id(self, output_id: &OutputId) -> Self {
        if self.is_null() { Self::from(output_id) } else { self }
    }
}

impl From<AccountId> for Address {
    fn from(value: AccountId) -> Self {
        Self::Account(AccountAddress::new(value))
    }
}
