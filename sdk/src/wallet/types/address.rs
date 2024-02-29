// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use crate::types::block::{address::Bech32Address, output::OutputId};

/// An account address with unspent output_ids for unspent outputs.
#[derive(Debug, Getters, Setters, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub(crate) struct AddressWithUnspentOutputs {
    /// The address.
    pub(crate) address: Bech32Address,
    /// Output ids
    pub(crate) output_ids: Vec<OutputId>,
}
