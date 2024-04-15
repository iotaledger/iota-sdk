// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use crate::{
    types::block::{address::Bech32Address, output::OutputId},
    wallet::OutputData,
};

#[derive(Debug, Getters, Setters, Clone, Serialize, Deserialize, Eq, PartialEq, derive_more::Deref)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub(crate) struct AddressWithUnspentOutputIds {
    #[deref]
    pub(crate) address: Bech32Address,
    pub(crate) unspent_output_ids: Vec<OutputId>,
}

#[derive(Debug, Getters, Setters, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub(crate) struct AddressWithUnspentOutputs {
    #[serde(flatten)]
    pub(crate) address_with_unspent_output_ids: AddressWithUnspentOutputIds,
    pub(crate) unspent_outputs: Vec<OutputData>,
}

pub(crate) type SpentOutputId = OutputId;
