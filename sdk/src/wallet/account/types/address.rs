// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::hash::Hash;

use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use crate::types::block::{address::Bech32Address, output::OutputId};

/// An account address.
#[derive(Debug, Getters, Setters, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct AccountAddress {
    /// The address.
    pub(crate) bech32_address: Bech32Address,
    /// The address key index.
    #[getset(set = "pub(crate)")]
    pub(crate) key_index: u32,
    /// Determines if an address is a public or an internal (change) address.
    #[getset(set = "pub(crate)")]
    pub(crate) internal: bool,
    // do we want this field? Could be useful if we don't store spent output ids and because of that wouldn't know if
    // an address was used or not just by looking at it
    pub(crate) used: bool,
}

impl AccountAddress {
    pub fn into_bech32(self) -> Bech32Address {
        self.bech32_address
    }
}

/// An account address with unspent output_ids for unspent outputs.
#[derive(Debug, Getters, Setters, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct AddressWithUnspentOutputs {
    /// The address.
    pub(crate) bech32_address: Bech32Address,
    /// The address key index.
    #[getset(set = "pub(crate)")]
    pub(crate) key_index: u32,
    /// Determines if an address is a public or an internal (change) address.
    #[getset(set = "pub(crate)")]
    pub(crate) internal: bool,
    /// Output ids
    pub(crate) output_ids: Vec<OutputId>,
}

impl AddressWithUnspentOutputs {
    pub fn into_bech32(self) -> Bech32Address {
        self.bech32_address
    }
}
