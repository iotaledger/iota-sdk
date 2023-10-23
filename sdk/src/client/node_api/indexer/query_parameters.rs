// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Query parameters for output_id requests

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::types::block::{address::Bech32Address, output::TokenId, slot::SlotIndex};

// https://github.com/iotaledger/inx-indexer/tree/develop/pkg/indexer

pub trait QueryParameter: Serialize + Send + std::marker::Sync {
    /// Converts parameters to a single String.
    fn to_query_string(&self) -> Option<String> {
        let value = serde_json::to_value(&self).unwrap();
        let mut final_str = String::new();
        for (field, v) in value.as_object().unwrap().iter() {
            if !v.is_null() {
                if let Some(v_str) = v.as_str() {
                    if !final_str.is_empty() {
                        final_str.push_str("&");
                    }
                    final_str.push_str(&format!("{}={}", field, v_str));
                }
                if let Some(v_u64) = v.as_u64() {
                    if !final_str.is_empty() {
                        final_str.push_str("&");
                    }
                    final_str.push_str(&format!("{}={}", field, v_u64));
                }
            }
        }
        if final_str.is_empty() { None } else { Some(final_str) }
    }
    fn replace_cursor(&mut self, cursor: String);
}

macro_rules! impl_query_parameters_methods {
    ($name:ty, $builder:ty) => {
        impl $builder {
            pub fn build(&mut self) -> $name {
                self.fallible_build()
                    .expect("builder can't fail, all values are optional")
            }
        }
        impl QueryParameter for $name {
            fn replace_cursor(&mut self, cursor: String) {
                self.cursor.replace(cursor);
            }
        }
    };
}

impl_query_parameters_methods!(OutputsQueryParameters, OutputsQueryParametersBuilder);

/// Query parameter for output requests.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Builder)]
#[builder(setter(prefix = "with", strip_option), build_fn(private, name = "fallible_build"))]
pub struct OutputsQueryParameters {
    /// Returns outputs that were created after a certain slot index.
    pub created_after: Option<SlotIndex>,
    /// Returns outputs that were created before a certain slot index.
    pub created_before: Option<SlotIndex>,
    /// Starts the search from the cursor (confirmationMS+outputId.pageSize).
    pub cursor: Option<String>,
    /// Filters outputs based on the presence of a native token.
    pub has_native_token: Option<bool>,
    /// Filters outputs based on the presence of a specific native token.
    pub native_token: Option<TokenId>,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    pub page_size: Option<usize>,
    /// Returns outputs that are unlockable by the bech32 address.
    pub unlockable_by_address: Option<Bech32Address>,
}

impl_query_parameters_methods!(BasicOutputsQueryParameters, BasicOutputsQueryParametersBuilder);

/// Query parameter for output requests.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Builder)]
#[builder(setter(prefix = "with", strip_option), build_fn(private, name = "fallible_build"))]
pub struct BasicOutputsQueryParameters {
    /// Returns outputs that were created after a certain slot index.
    pub created_after: Option<SlotIndex>,
    /// Returns outputs that were created before a certain slot index.
    pub created_before: Option<SlotIndex>,
    /// Starts the search from the cursor (confirmationMS+outputId.pageSize).
    pub cursor: Option<String>,
    /// Filters outputs based on the presence of a native token.
    pub has_native_token: Option<bool>,
    /// Filters outputs based on the presence of a specific native token.
    pub native_token: Option<TokenId>,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    pub page_size: Option<usize>,
    /// Returns outputs that are unlockable by the bech32 address.
    pub unlockable_by_address: Option<Bech32Address>,
    /// Bech32-encoded address that should be searched for.
    pub address: Option<Bech32Address>,
    /// Filters outputs based on the presence of a specific Bech32-encoded return address in the expiration unlock
    /// condition.
    pub expiration_return_address: Option<Bech32Address>,
    /// Returns outputs that expire after a certain slot index.
    pub expires_after: Option<SlotIndex>,
    /// Returns outputs that expire before a certain slot index.
    pub expires_before: Option<SlotIndex>,
    /// Filters outputs based on the presence of expiration unlock condition.
    pub has_expiration: Option<bool>,
    /// Filters outputs based on the presence of storage deposit return unlock condition.
    pub has_storage_deposit_return: Option<bool>,
    /// Filters outputs based on the presence of timelock unlock condition.
    pub has_timelock: Option<bool>,
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    pub sender: Option<Bech32Address>,
    /// Filters outputs based on the presence of a specific return address in the storage deposit return unlock
    /// condition.
    pub storage_deposit_return_address: Option<Bech32Address>,
    /// Filters outputs based on matching Tag Block.
    pub tag: Option<String>,
    /// Returns outputs that are timelocked after a certain slot index.
    pub timelocked_after: Option<SlotIndex>,
    /// Returns outputs that are timelocked before a certain slot index.
    pub timelocked_before: Option<SlotIndex>,
}

impl_query_parameters_methods!(AccountOutputsQueryParameters, AccountOutputsQueryParametersBuilder);

/// Query parameter for output requests.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Builder)]
#[builder(setter(prefix = "with", strip_option), build_fn(private, name = "fallible_build"))]
pub struct AccountOutputsQueryParameters {
    /// Returns outputs that were created after a certain slot index.
    pub created_after: Option<SlotIndex>,
    /// Returns outputs that were created before a certain slot index.
    pub created_before: Option<SlotIndex>,
    /// Starts the search from the cursor (confirmationMS+outputId.pageSize).
    pub cursor: Option<String>,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    pub page_size: Option<usize>,
    /// Returns outputs that are unlockable by the bech32 address.
    pub unlockable_by_address: Option<Bech32Address>,
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    pub sender: Option<Bech32Address>,
    /// Filters outputs based on bech32-encoded issuer address.
    pub issuer: Option<Bech32Address>,
    /// Filters outputs based on bech32-encoded governor (governance controller) address.
    pub governor: Option<Bech32Address>,
    /// Filters outputs based on bech32-encoded state controller address.
    pub state_controller: Option<Bech32Address>,
}

impl_query_parameters_methods!(NftOutputsQueryParameters, NftOutputsQueryParametersBuilder);

/// Query parameter for output requests.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Builder)]
#[builder(setter(prefix = "with", strip_option), build_fn(private, name = "fallible_build"))]
pub struct NftOutputsQueryParameters {
    /// Returns outputs that were created after a certain slot index.
    pub created_after: Option<SlotIndex>,
    /// Returns outputs that were created before a certain slot index.
    pub created_before: Option<SlotIndex>,
    /// Starts the search from the cursor (confirmationMS+outputId.pageSize).
    pub cursor: Option<String>,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    pub page_size: Option<usize>,
    /// Returns outputs that are unlockable by the bech32 address.
    pub unlockable_by_address: Option<Bech32Address>,
    /// Bech32-encoded address that should be searched for.
    pub address: Option<Bech32Address>,
    /// Filters outputs based on the presence of a specific Bech32-encoded return address in the expiration unlock
    /// condition.
    pub expiration_return_address: Option<Bech32Address>,
    /// Returns outputs that expire after a certain slot index.
    pub expires_after: Option<SlotIndex>,
    /// Returns outputs that expire before a certain slot index.
    pub expires_before: Option<SlotIndex>,
    /// Filters outputs based on the presence of expiration unlock condition.
    pub has_expiration: Option<bool>,
    /// Filters outputs based on the presence of storage deposit return unlock condition.
    pub has_storage_deposit_return: Option<bool>,
    /// Filters outputs based on the presence of timelock unlock condition.
    pub has_timelock: Option<bool>,
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    pub sender: Option<Bech32Address>,
    /// Filters outputs based on bech32-encoded issuer address.
    pub issuer: Option<Bech32Address>,
    /// Filters outputs based on the presence of a specific return address in the storage deposit return unlock
    /// condition.
    pub storage_deposit_return_address: Option<Bech32Address>,
    /// Filters outputs based on matching Tag Block.
    pub tag: Option<String>,
    /// Returns outputs that are timelocked after a certain slot index.
    pub timelocked_after: Option<SlotIndex>,
    /// Returns outputs that are timelocked before a certain slot index.
    pub timelocked_before: Option<SlotIndex>,
}

impl_query_parameters_methods!(FoundryOutputsQueryParameters, FoundryOutputsQueryParametersBuilder);

/// Query parameter for output requests.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Builder)]
#[builder(setter(prefix = "with", strip_option), build_fn(private, name = "fallible_build"))]
pub struct FoundryOutputsQueryParameters {
    /// Returns outputs that were created after a certain slot index.
    pub created_after: Option<SlotIndex>,
    /// Returns outputs that were created before a certain slot index.
    pub created_before: Option<SlotIndex>,
    /// Starts the search from the cursor (confirmationMS+outputId.pageSize).
    pub cursor: Option<String>,
    /// Filters outputs based on the presence of a native token.
    pub has_native_token: Option<bool>,
    /// Filters outputs based on the presence of a specific native token.
    pub native_token: Option<TokenId>,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    pub page_size: Option<usize>,
    /// Filter foundry outputs based on bech32-encoded address of the controlling account.
    pub account_address: Option<Bech32Address>,
}

/// Query parameter for output requests.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Builder)]
#[builder(setter(prefix = "with", strip_option), build_fn(private, name = "fallible_build"))]
pub struct DelegationOutputsQueryParameters {
    /// Returns outputs that were created after a certain slot index.
    pub created_after: Option<SlotIndex>,
    /// Returns outputs that were created before a certain slot index.
    pub created_before: Option<SlotIndex>,
    /// Starts the search from the cursor (confirmationMS+outputId.pageSize).
    pub cursor: Option<String>,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    pub page_size: Option<usize>,
    /// Bech32-encoded address that should be searched for.
    pub address: Option<Bech32Address>,
    /// Filter foundry outputs based on bech32-encoded address of the validator.
    pub validator: Option<Bech32Address>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_parameter() {
        let empty_basic_outputs_query_parameters = BasicOutputsQueryParameters::default();
        assert_eq!(empty_basic_outputs_query_parameters.to_query_string(), None);

        let mut basic_outputs_query_parameters = BasicOutputsQueryParameters {
            address: Some(
                Bech32Address::try_from_str("atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r")
                    .unwrap(),
            ),
            cursor: Some("".into()),
            ..Default::default()
        };
        assert_eq!(
            basic_outputs_query_parameters.to_query_string(),
            Some("address=atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r&cursor=".into())
        );

        basic_outputs_query_parameters.replace_cursor("newCursor".into());
        assert_eq!(
            basic_outputs_query_parameters.to_query_string(),
            Some("address=atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r&cursor=newCursor".into())
        );
    }
}
