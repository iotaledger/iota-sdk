// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Dtos with amount as String, to prevent overflow issues in other languages

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    types::block::{
        address::Bech32Address,
        output::{dto::FoundryOutputDto, FoundryId, OutputId},
        payload::transaction::TransactionId,
    },
    wallet::{
        account::{
            types::{AccountAddress, AddressWithUnspentOutputs, TransactionDto},
            AccountDetails, OutputDataDto,
        },
        AddressWithAmount,
    },
};

/// Dto for address with amount for `send_amount()`
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressWithAmountDto {
    /// Bech32 encoded address
    pub address: String,
    /// Amount
    pub amount: String,
    /// Bech32 encoded address return address, to which the storage deposit will be returned. Default will use the
    /// first address of the account
    pub return_address: Option<String>,
    /// Expiration in seconds, after which the output will be available for the sender again, if not spent by the
    /// receiver before. Default is 1 day
    pub expiration: Option<u32>,
}

impl TryFrom<&AddressWithAmountDto> for AddressWithAmount {
    type Error = crate::wallet::Error;

    fn try_from(value: &AddressWithAmountDto) -> crate::wallet::Result<Self> {
        Ok(Self::new(
            value.address.clone(),
            u64::from_str(&value.amount).map_err(|_| crate::client::Error::InvalidAmount(value.amount.clone()))?,
        )
        .with_return_address(value.return_address.clone())
        .with_expiration(value.expiration))
    }
}

/// Dto for an account address with output_ids of unspent outputs.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressWithUnspentOutputsDto {
    /// The address.
    pub address: Bech32Address,
    /// The address key index.
    pub key_index: u32,
    /// Determines if an address is a public or an internal (change) address.
    pub internal: bool,
    /// Output ids
    pub output_ids: Vec<OutputId>,
}

impl From<&AddressWithUnspentOutputs> for AddressWithUnspentOutputsDto {
    fn from(value: &AddressWithUnspentOutputs) -> Self {
        Self {
            address: value.address.clone(),
            key_index: value.key_index,
            internal: value.internal,
            output_ids: value.output_ids.clone(),
        }
    }
}

/// Dto for an Account.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDto {
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
    pub addresses_with_unspent_outputs: Vec<AddressWithUnspentOutputsDto>,
    /// Outputs
    pub outputs: HashMap<OutputId, OutputDataDto>,
    /// Unspent outputs that are currently used as input for transactions
    pub locked_outputs: HashSet<OutputId>,
    /// Unspent outputs
    pub unspent_outputs: HashMap<OutputId, OutputDataDto>,
    /// Sent transactions
    pub transactions: HashMap<TransactionId, TransactionDto>,
    /// Pending transactions
    pub pending_transactions: HashSet<TransactionId>,
    /// Incoming transactions
    pub incoming_transactions: HashMap<TransactionId, TransactionDto>,
    /// Foundries for native tokens in outputs
    #[serde(default)]
    pub native_token_foundries: HashMap<FoundryId, FoundryOutputDto>,
}

impl From<&AccountDetails> for AccountDto {
    fn from(value: &AccountDetails) -> Self {
        Self {
            index: *value.index(),
            coin_type: *value.coin_type(),
            alias: value.alias().clone(),
            public_addresses: value.public_addresses.clone(),
            internal_addresses: value.internal_addresses.clone(),
            addresses_with_unspent_outputs: value
                .addresses_with_unspent_outputs()
                .iter()
                .map(AddressWithUnspentOutputsDto::from)
                .collect(),
            outputs: value
                .outputs()
                .iter()
                .map(|(id, output)| (*id, OutputDataDto::from(output)))
                .collect(),
            locked_outputs: value.locked_outputs().clone(),
            unspent_outputs: value
                .unspent_outputs()
                .iter()
                .map(|(id, output)| (*id, OutputDataDto::from(output)))
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
                .map(|(id, foundry)| (*id, FoundryOutputDto::from(foundry)))
                .collect(),
        }
    }
}
