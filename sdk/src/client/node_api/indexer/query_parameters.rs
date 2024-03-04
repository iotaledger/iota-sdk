// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Query parameters for output_id requests

use derive_setters::Setters;
use serde::{Deserialize, Serialize};

use crate::types::block::{address::Bech32Address, output::TokenId, slot::SlotIndex};

pub trait QueryParameter: Serialize + Send + Sync {
    /// Converts parameters to a single String.
    fn to_query_string(&self) -> Option<String> {
        let value = serde_json::to_value(self).unwrap();
        let mut query_string = String::new();

        for (field, v) in value.as_object().unwrap().iter() {
            if !v.is_null() {
                if let Some(v_bool) = v.as_bool() {
                    if !query_string.is_empty() {
                        query_string.push('&');
                    }
                    query_string.push_str(&format!("{field}={v_bool}"));
                } else if let Some(v_str) = v.as_str() {
                    if !query_string.is_empty() {
                        query_string.push('&');
                    }
                    query_string.push_str(&format!("{field}={v_str}"));
                } else if let Some(v_u64) = v.as_u64() {
                    if !query_string.is_empty() {
                        query_string.push('&');
                    }
                    query_string.push_str(&format!("{field}={v_u64}"));
                }
            }
        }

        if query_string.is_empty() {
            None
        } else {
            Some(query_string)
        }
    }

    fn replace_cursor(&mut self, cursor: String);
}

macro_rules! impl_query_parameters_methods {
    ($name:ty) => {
        impl $name {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl QueryParameter for $name {
            fn replace_cursor(&mut self, cursor: String) {
                self.cursor.replace(cursor);
            }
        }
    };
}

/// Query parameters for output requests.
#[derive(Setters, Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[setters(strip_option)]
#[serde(rename_all = "camelCase")]
pub struct OutputQueryParameters {
    /// Filters outputs based on the presence of a native token.
    has_native_token: Option<bool>,
    /// Filters outputs based on the presence of a specific native token.
    native_token: Option<TokenId>,
    /// Returns outputs that are unlockable by the bech32 address.
    unlockable_by_address: Option<Bech32Address>,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    page_size: Option<usize>,
    /// Starts the search from the cursor (createdSlotIndex+outputId.pageSize). If an empty String is provided, only
    /// the first page is returned.
    cursor: Option<String>,
    /// Returns outputs that were created before a certain slot index.
    created_before: Option<SlotIndex>,
    /// Returns outputs that were created after a certain slot index.
    created_after: Option<SlotIndex>,
}

impl_query_parameters_methods!(OutputQueryParameters);

/// Query parameters for basic output requests.
#[derive(Setters, Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[setters(strip_option)]
#[serde(rename_all = "camelCase")]
pub struct BasicOutputQueryParameters {
    /// Filters outputs based on the presence of a native token.
    has_native_token: Option<bool>,
    /// Filters outputs based on the presence of a specific native token.
    native_token: Option<TokenId>,
    /// Returns outputs that are unlockable by the bech32 address.
    unlockable_by_address: Option<Bech32Address>,
    /// Bech32-encoded address that should be searched for.
    address: Option<Bech32Address>,
    /// Filters outputs based on the presence of storage deposit return unlock condition.
    has_storage_deposit_return: Option<bool>,
    /// Filters outputs based on the presence of a specific return address in the storage deposit return unlock
    /// condition.
    storage_deposit_return_address: Option<Bech32Address>,
    /// Filters outputs based on the presence of expiration unlock condition.
    has_expiration: Option<bool>,
    /// Filters outputs based on the presence of a specific Bech32-encoded return address in the expiration unlock
    /// condition.
    expiration_return_address: Option<Bech32Address>,
    /// Returns outputs that expire before a certain slot index.
    expires_before: Option<SlotIndex>,
    /// Returns outputs that expire after a certain slot index.
    expires_after: Option<SlotIndex>,
    /// Filters outputs based on the presence of timelock unlock condition.
    has_timelock: Option<bool>,
    /// Returns outputs that are timelocked before a certain slot index.
    timelocked_before: Option<SlotIndex>,
    /// Returns outputs that are timelocked after a certain slot index.
    timelocked_after: Option<SlotIndex>,
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    sender: Option<Bech32Address>,
    /// Filters outputs based on matching Tag Block.
    tag: Option<String>,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    page_size: Option<usize>,
    /// Starts the search from the cursor (createdSlotIndex+outputId.pageSize). If an empty String is provided, only
    /// the first page is returned.
    cursor: Option<String>,
    /// Returns outputs that were created before a certain slot index.
    created_before: Option<SlotIndex>,
    /// Returns outputs that were created after a certain slot index.
    created_after: Option<SlotIndex>,
}

impl_query_parameters_methods!(BasicOutputQueryParameters);

impl BasicOutputQueryParameters {
    /// Sets `.address(address).has_expiration(false).has_storage_deposit_return(false).has_timelock(false)` to only
    /// get outputs that can be unlocked by the address without potential further restrictions.
    pub fn only_address_unlock_condition(address: impl Into<Bech32Address>) -> Self {
        Self::default()
            .address(address.into())
            .has_expiration(false)
            .has_storage_deposit_return(false)
            .has_timelock(false)
    }
}

/// Query parameters for account output requests.
#[derive(Setters, Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[setters(strip_option)]
#[serde(rename_all = "camelCase")]
pub struct AccountOutputQueryParameters {
    /// Bech32-encoded address that should be searched for.
    address: Option<Bech32Address>,
    /// Filters outputs based on bech32-encoded issuer address.
    issuer: Option<Bech32Address>,
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    sender: Option<Bech32Address>,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    page_size: Option<usize>,
    /// Starts the search from the cursor (createdSlotIndex+outputId.pageSize). If an empty String is provided, only
    /// the first page is returned.
    cursor: Option<String>,
    /// Returns outputs that were created before a certain slot index.
    created_before: Option<SlotIndex>,
    /// Returns outputs that were created after a certain slot index.
    created_after: Option<SlotIndex>,
}

impl_query_parameters_methods!(AccountOutputQueryParameters);

/// Query parameters for anchor output requests.
#[derive(Setters, Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[setters(strip_option)]
#[serde(rename_all = "camelCase")]
pub struct AnchorOutputQueryParameters {
    /// Returns outputs that are unlockable by the bech32 address.
    unlockable_by_address: Option<Bech32Address>,
    /// Filters outputs based on bech32-encoded state controller address.
    state_controller: Option<Bech32Address>,
    /// Filters outputs based on bech32-encoded governor (governance controller) address.
    governor: Option<Bech32Address>,
    /// Filters outputs based on bech32-encoded issuer address.
    issuer: Option<Bech32Address>,
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    sender: Option<Bech32Address>,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    page_size: Option<usize>,
    /// Starts the search from the cursor (createdSlotIndex+outputId.pageSize). If an empty String is provided, only
    /// the first page is returned.
    cursor: Option<String>,
    /// Returns outputs that were created before a certain slot index.
    created_before: Option<SlotIndex>,
    /// Returns outputs that were created after a certain slot index.
    created_after: Option<SlotIndex>,
}

impl_query_parameters_methods!(AnchorOutputQueryParameters);

/// Query parameters for delegation output requests.
#[derive(Setters, Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[setters(strip_option)]
#[serde(rename_all = "camelCase")]
pub struct DelegationOutputQueryParameters {
    /// Bech32-encoded address that should be searched for.
    address: Option<Bech32Address>,
    /// Filter foundry outputs based on bech32-encoded address of the validator.
    validator: Option<Bech32Address>,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    page_size: Option<usize>,
    /// Starts the search from the cursor (createdSlotIndex+outputId.pageSize). If an empty String is provided, only
    /// the first page is returned.
    cursor: Option<String>,
    /// Returns outputs that were created before a certain slot index.
    created_before: Option<SlotIndex>,
    /// Returns outputs that were created after a certain slot index.
    created_after: Option<SlotIndex>,
}

impl_query_parameters_methods!(DelegationOutputQueryParameters);

/// Query parameters for foundry output requests.
#[derive(Setters, Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[setters(strip_option)]
#[serde(rename_all = "camelCase")]
pub struct FoundryOutputQueryParameters {
    /// Filters outputs based on the presence of a native token.
    has_native_token: Option<bool>,
    /// Filters outputs based on the presence of a specific native token.
    native_token: Option<TokenId>,
    /// Filter foundry outputs based on bech32-encoded address of the controlling account.
    account: Option<Bech32Address>,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    page_size: Option<usize>,
    /// Starts the search from the cursor (createdSlotIndex+outputId.pageSize). If an empty String is provided, only
    /// the first page is returned.
    cursor: Option<String>,
    /// Returns outputs that were created before a certain slot index.
    created_before: Option<SlotIndex>,
    /// Returns outputs that were created after a certain slot index.
    created_after: Option<SlotIndex>,
}

impl_query_parameters_methods!(FoundryOutputQueryParameters);

/// Query parameters for nft output requests.
#[derive(Setters, Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[setters(strip_option)]
#[serde(rename_all = "camelCase")]
pub struct NftOutputQueryParameters {
    /// Returns outputs that are unlockable by the bech32 address.
    unlockable_by_address: Option<Bech32Address>,
    /// Bech32-encoded address that should be searched for.
    address: Option<Bech32Address>,
    /// Filters outputs based on the presence of storage deposit return unlock condition.
    has_storage_deposit_return: Option<bool>,
    /// Filters outputs based on the presence of a specific return address in the storage deposit return unlock
    /// condition.
    storage_deposit_return_address: Option<Bech32Address>,
    /// Filters outputs based on the presence of expiration unlock condition.
    has_expiration: Option<bool>,
    /// Filters outputs based on the presence of a specific Bech32-encoded return address in the expiration unlock
    /// condition.
    expiration_return_address: Option<Bech32Address>,
    /// Returns outputs that expire before a certain slot index.
    expires_before: Option<SlotIndex>,
    /// Returns outputs that expire after a certain slot index.
    expires_after: Option<SlotIndex>,
    /// Filters outputs based on the presence of timelock unlock condition.
    has_timelock: Option<bool>,
    /// Returns outputs that are timelocked before a certain slot index.
    timelocked_before: Option<SlotIndex>,
    /// Returns outputs that are timelocked after a certain slot index.
    timelocked_after: Option<SlotIndex>,
    /// Filters outputs based on bech32-encoded issuer address.
    issuer: Option<Bech32Address>,
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    sender: Option<Bech32Address>,
    /// Filters outputs based on matching Tag Block.
    tag: Option<String>,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    page_size: Option<usize>,
    /// Starts the search from the cursor (createdSlotIndex+outputId.pageSize). If an empty String is provided, only
    /// the first page is returned.
    cursor: Option<String>,
    /// Returns outputs that were created before a certain slot index.
    created_before: Option<SlotIndex>,
    /// Returns outputs that were created after a certain slot index.
    created_after: Option<SlotIndex>,
}

impl_query_parameters_methods!(NftOutputQueryParameters);

impl NftOutputQueryParameters {
    /// Sets `.address(address).has_expiration(false).has_storage_deposit_return(false).has_timelock(false)` to only
    /// get outputs that can be unlocked by the address without potential further restrictions.
    pub fn only_address_unlock_condition(address: impl Into<Bech32Address>) -> Self {
        Self::default()
            .address(address.into())
            .has_expiration(false)
            .has_storage_deposit_return(false)
            .has_timelock(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_parameter() {
        let empty_basic_outputs_query_parameters = BasicOutputQueryParameters::new();
        assert_eq!(empty_basic_outputs_query_parameters.to_query_string(), None);

        let mut basic_outputs_query_parameters = BasicOutputQueryParameters::new()
            .address(
                Bech32Address::try_from_str("atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r")
                    .unwrap(),
            )
            .created_after(5.into())
            .has_timelock(true)
            .cursor("".into());
        assert_eq!(
            basic_outputs_query_parameters.to_query_string(),
            Some(
                "address=atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r&createdAfter=5&cursor=&hasTimelock=true"
                    .into()
            )
        );

        basic_outputs_query_parameters.replace_cursor("newCursor".into());
        assert_eq!(
            basic_outputs_query_parameters.to_query_string(),
            Some("address=atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r&createdAfter=5&cursor=newCursor&hasTimelock=true".into())
        );
    }
}
