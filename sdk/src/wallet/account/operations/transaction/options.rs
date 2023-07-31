// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    client::api::input_selection::Burn,
    types::block::{output::OutputId, payload::tagged_data::TaggedDataPayload},
    wallet::account::types::address::AccountAddress,
};

/// Options for transactions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionOptions {
    #[serde(default)]
    pub remainder_value_strategy: RemainderValueStrategy,
    #[serde(default)]
    pub tagged_data_payload: Option<TaggedDataPayload>,
    // If custom inputs are provided only they are used. If also other additional inputs should be used,
    // `mandatory_inputs` should be used instead.
    #[serde(default)]
    pub custom_inputs: Option<Vec<OutputId>>,
    #[serde(default)]
    pub mandatory_inputs: Option<Vec<OutputId>>,
    pub burn: Option<Burn>,
    pub note: Option<String>,
    #[serde(default)]
    pub allow_micro_amount: bool,
}

#[allow(clippy::enum_variant_names)]
/// The strategy to use for the remainder value management when sending funds.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "strategy", content = "value")]
pub enum RemainderValueStrategy {
    /// Keep the remainder value on the source address.
    ReuseAddress,
    /// Move the remainder value to a change address.
    ChangeAddress,
    /// Move the remainder value to any specified address.
    CustomAddress(AccountAddress),
}

impl Default for RemainderValueStrategy {
    fn default() -> Self {
        Self::ReuseAddress
    }
}
