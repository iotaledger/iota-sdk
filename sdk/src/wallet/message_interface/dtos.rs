// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Dtos with amount as String, to prevent overflow issues in other languages

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    types::block::{
        output::{FoundryId, FoundryOutput, OutputId},
        payload::transaction::TransactionId,
    },
    wallet::account::{
        types::{AccountAddress, AddressWithUnspentOutputs, OutputData, TransactionDto},
        AccountDetails,
    },
};

/// Dto for an Account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDetailsDto {
    /// The account index
    pub index: u32,
    /// The coin type
    pub coin_type: u32,
    /// The account alias.
    pub alias: String,
    /// Public addresses
    pub public_addresses: Vec<AccountAddress>,
    /// Internal addresses
    pub internal_addresses: Vec<AccountAddress>,
    /// Addresses with unspent outputs
    pub addresses_with_unspent_outputs: Vec<AddressWithUnspentOutputs>,
    /// Outputs
    pub outputs: HashMap<OutputId, OutputData>,
    /// Unspent outputs that are currently used as input for transactions
    pub locked_outputs: HashSet<OutputId>,
    /// Unspent outputs
    pub unspent_outputs: HashMap<OutputId, OutputData>,
    /// Sent transactions
    pub transactions: HashMap<TransactionId, TransactionDto>,
    /// Pending transactions
    pub pending_transactions: HashSet<TransactionId>,
    /// Incoming transactions
    pub incoming_transactions: HashMap<TransactionId, TransactionDto>,
    /// Foundries for native tokens in outputs
    #[serde(default)]
    pub native_token_foundries: HashMap<FoundryId, FoundryOutput>,
}

impl From<&AccountDetails> for AccountDetailsDto {
    fn from(value: &AccountDetails) -> Self {
        Self {
            index: *value.index(),
            coin_type: *value.coin_type(),
            alias: value.alias().clone(),
            public_addresses: value.public_addresses.clone(),
            internal_addresses: value.internal_addresses.clone(),
            addresses_with_unspent_outputs: value.addresses_with_unspent_outputs().clone(),
            outputs: value
                .outputs()
                .iter()
                .map(|(id, output)| (*id, output.clone()))
                .collect(),
            locked_outputs: value.locked_outputs().clone(),
            unspent_outputs: value
                .unspent_outputs()
                .iter()
                .map(|(id, output)| (*id, output.clone()))
                .collect(),
            transactions: value
                .transactions()
                .iter()
                .map(|(id, transaction)| (*id, TransactionDto::from(transaction)))
                .collect(),
            pending_transactions: value.pending_transactions().clone(),
            incoming_transactions: value
                .incoming_transactions()
                .iter()
                .map(|(id, transaction)| (*id, TransactionDto::from(transaction)))
                .collect(),
            native_token_foundries: value
                .native_token_foundries()
                .iter()
                .map(|(id, foundry)| (*id, foundry.clone()))
                .collect(),
        }
    }
}
