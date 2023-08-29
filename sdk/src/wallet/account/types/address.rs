// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::hash::Hash;

use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use crate::types::{
    self,
    block::{address::Bech32Address, output::OutputId, ConvertTo},
};

/// A BIP44 address.
#[derive(Debug, Getters, Setters, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct Bip44Address {
    /// The address.
    pub(crate) address: Bech32Address,
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

impl Bip44Address {
    pub fn into_bech32(self) -> Bech32Address {
        self.address
    }
}

impl ConvertTo<Bech32Address> for Bip44Address {
    fn convert(self) -> Result<Bech32Address, types::block::Error> {
        Ok(self.address)
    }

    fn convert_unchecked(self) -> Bech32Address {
        self.address
    }
}

/// An account address with unspent output_ids for unspent outputs.
#[derive(Debug, Getters, Setters, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct AddressWithUnspentOutputs {
    /// The address.
    pub(crate) address: Bech32Address,
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
        self.address
    }
}
