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

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::utils::json::{FromJson, JsonExt, ToJson, Value};

    impl ToJson for Bip44Address {
        fn to_json(&self) -> Value {
            crate::json!({
                "address": self.address(),
                "keyIndex": self.key_index(),
                "internal": self.internal(),
            })
        }
    }

    impl FromJson for Bip44Address {
        type Error = crate::types::block::Error;

        fn from_non_null_json(mut value: Value) -> core::result::Result<Self, Self::Error>
        where
            Self: Sized,
        {
            Ok(Self {
                address: value["address"].take_value()?,
                key_index: value["keyIndex"].to_u32()?,
                internal: value["internal"].to_bool()?,
            })
        }
    }

    impl ToJson for AddressWithUnspentOutputs {
        fn to_json(&self) -> Value {
            crate::json!({
                "address": self.address(),
                "keyIndex": self.key_index(),
                "internal": self.internal(),
                "outputIds": self.output_ids()
            })
        }
    }

    impl FromJson for AddressWithUnspentOutputs {
        type Error = crate::types::block::Error;

        fn from_non_null_json(mut value: Value) -> core::result::Result<Self, Self::Error>
        where
            Self: Sized,
        {
            Ok(Self {
                address: value["address"].take_value()?,
                key_index: value["keyIndex"].to_u32()?,
                internal: value["internal"].to_bool()?,
                output_ids: value["outputIds"].take_vec()?,
            })
        }
    }
}
