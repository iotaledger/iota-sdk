// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops;

use derive_more::{AsRef, From};
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use crate::{
    types::block::{address::Bech32Address, output::OutputId},
    wallet::OutputWithExtendedMetadata,
};

#[derive(Debug, Getters, Setters, Clone, Serialize, Deserialize, Eq, PartialEq, derive_more::Deref)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub(crate) struct AddressWithUnspentOutputIds {
    #[deref]
    pub(crate) address: Bech32Address,
    pub(crate) unspent_output_ids: Vec<OutputId>,
}

#[derive(Debug, Getters, Setters, Clone, Serialize, Deserialize, Eq, PartialEq, derive_more::Deref)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub(crate) struct AddressWithUnspentOutputs {
    #[deref]
    pub(crate) address: Bech32Address,
    pub(crate) unspent_output_ids: Vec<OutputId>,
    pub(crate) unspent_outputs: Vec<OutputWithExtendedMetadata>,
}

pub(crate) type SpentOutput = OutputId;
