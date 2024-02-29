// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    client::api::transaction_builder::Burn,
    types::block::{
        address::Address,
        context_input::ContextInput,
        output::{AccountId, OutputId},
        payload::{signed_transaction::TransactionCapabilities, tagged_data::TaggedDataPayload},
    },
};

/// Options for transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct TransactionOptions {
    /// The strategy applied for base coin remainders.
    pub remainder_value_strategy: RemainderValueStrategy,
    /// An optional tagged data payload.
    pub tagged_data_payload: Option<TaggedDataPayload>,
    /// Transaction context inputs to include.
    pub context_inputs: Vec<ContextInput>,
    /// Inputs that must be used for the transaction.
    pub required_inputs: BTreeSet<OutputId>,
    /// Specifies what needs to be burned during iransaction builder.
    pub burn: Option<Burn>,
    /// A string attached to the transaction.
    pub note: Option<String>,
    /// Whether to allow sending a micro amount.
    pub allow_micro_amount: bool,
    /// Whether to allow the selection of additional inputs for this transaction.
    pub allow_additional_input_selection: bool,
    /// Transaction capabilities.
    pub capabilities: Option<TransactionCapabilities>,
    /// Mana allotments for the transaction.
    pub mana_allotments: BTreeMap<AccountId, u64>,
    /// Optional block issuer to which the transaction will have required mana allotted.
    pub issuer_id: Option<AccountId>,
}

impl Default for TransactionOptions {
    fn default() -> Self {
        Self {
            remainder_value_strategy: Default::default(),
            tagged_data_payload: Default::default(),
            context_inputs: Default::default(),
            required_inputs: Default::default(),
            burn: Default::default(),
            note: Default::default(),
            allow_micro_amount: false,
            allow_additional_input_selection: true,
            capabilities: Default::default(),
            mana_allotments: Default::default(),
            issuer_id: Default::default(),
        }
    }
}

#[allow(clippy::enum_variant_names)]
/// The strategy to use for the remainder value management when sending funds.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "strategy", content = "value")]
pub enum RemainderValueStrategy {
    /// Keep the remainder value on the source address.
    ReuseAddress,
    /// Move the remainder value to any specified address.
    CustomAddress(Address),
}

impl Default for RemainderValueStrategy {
    fn default() -> Self {
        Self::ReuseAddress
    }
}
