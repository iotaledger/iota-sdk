// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    client::api::input_selection::Burn,
    types::block::{
        address::Address,
        context_input::ContextInput,
        mana::ManaAllotment,
        output::{AccountId, OutputId},
        payload::{signed_transaction::TransactionCapabilities, tagged_data::TaggedDataPayload},
        slot::SlotCommitmentId,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockOptions {
    #[serde(default)]
    pub issuer_id: Option<AccountId>,
    #[serde(default)]
    pub latest_slot_commitment_id: Option<SlotCommitmentId>,
    #[serde(default)]
    pub transaction_options: Option<TransactionOptions>,
}

impl core::ops::Deref for BlockOptions {
    type Target = Option<TransactionOptions>;

    fn deref(&self) -> &Self::Target {
        &self.transaction_options
    }
}

impl From<TransactionOptions> for BlockOptions {
    fn from(transaction_options: TransactionOptions) -> Self {
        Self {
            issuer_id: None,
            latest_slot_commitment_id: None,
            transaction_options: Some(transaction_options),
        }
    }
}

impl From<AccountId> for BlockOptions {
    fn from(account_id: AccountId) -> Self {
        Self {
            issuer_id: Some(account_id),
            latest_slot_commitment_id: None,
            transaction_options: None,
        }
    }
}

/// Options for transactions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionOptions {
    #[serde(default)]
    pub remainder_value_strategy: RemainderValueStrategy,
    #[serde(default)]
    pub tagged_data_payload: Option<TaggedDataPayload>,
    #[serde(default)]
    pub context_inputs: Option<Vec<ContextInput>>,
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
    #[serde(default)]
    pub capabilities: Option<TransactionCapabilities>,
    #[serde(default)]
    pub mana_allotments: Option<Vec<ManaAllotment>>,
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
