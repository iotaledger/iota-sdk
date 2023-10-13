// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Query parameters for output_id requests

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    client::node_api::query_tuples_to_query_string,
    types::block::{address::Bech32Address, output::TokenId, slot::SlotIndex},
};

// https://github.com/iotaledger/inx-indexer/tree/develop/pkg/indexer

pub trait QueryParameterHelper {
    /// Converts parameters to a single String.
    fn to_query_string(&self) -> Option<String>;
    fn replace_cursor(&mut self, cursor: String);
}

macro_rules! impl_query_parameters_methods {
    ($name:ty, $inner_type:ty) => {
        impl $name {
            /// Creates a hashset from a provided vec of query parameters.
            #[must_use]
            pub fn new(query_parameters: impl Into<Vec<$inner_type>>) -> Self {
                let mut query_parameters = query_parameters.into();
                query_parameters.sort_unstable_by_key(<$inner_type>::kind);
                query_parameters.dedup_by_key(|qp| qp.kind());

                Self(query_parameters)
            }
            /// Replaces or inserts an enum variant in the QueryParameters.
            pub fn replace(&mut self, query_parameter: $inner_type) {
                match self
                    .0
                    .binary_search_by_key(&query_parameter.kind(), <$inner_type>::kind)
                {
                    Ok(pos) => self.0[pos] = query_parameter,
                    Err(pos) => self.0.insert(pos, query_parameter),
                }
            }
            /// Creates new empty QueryParameters.
            pub fn empty() -> Self {
                Self(Vec::new())
            }
        }
        impl QueryParameterHelper for $name {
            /// Converts parameters to a single String.
            fn to_query_string(&self) -> Option<String> {
                query_tuples_to_query_string(self.0.iter().map(|q| Some(q.to_query_tuple())))
            }
            fn replace_cursor(&mut self, cursor: String) {
                self.replace(<$inner_type>::Cursor(cursor));
            }
        }

        impl fmt::Display for $inner_type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let query_tuple = self.to_query_tuple();
                write!(f, "{}={}", query_tuple.0, query_tuple.1)
            }
        }
    };
}

/// Query parameters for output_id requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputsQueryParameters(Vec<OutputsQueryParameter>);

impl_query_parameters_methods!(OutputsQueryParameters, OutputsQueryParameter);

/// Query parameter for output requests.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum OutputsQueryParameter {
    /// Returns outputs that were created after a certain slot index.
    CreatedAfter(SlotIndex),
    /// Returns outputs that were created before a certain slot index.
    CreatedBefore(SlotIndex),
    /// Starts the search from the cursor (confirmationMS+outputId.pageSize).
    Cursor(String),
    /// Filters outputs based on the presence of a native token.
    HasNativeToken(bool),
    /// Filters outputs based on the presence of a specific native token.
    NativeToken(TokenId),
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    PageSize(usize),
    /// Returns outputs that are unlockable by the bech32 address.
    UnlockableByAddress(Bech32Address),
}

impl OutputsQueryParameter {
    fn to_query_tuple(&self) -> (&'static str, String) {
        match self {
            Self::CreatedAfter(v) => ("createdAfter", v.to_string()),
            Self::CreatedBefore(v) => ("createdBefore", v.to_string()),
            Self::Cursor(v) => ("cursor", v.to_string()),
            Self::HasNativeToken(v) => ("hasNativeToken", v.to_string()),
            Self::NativeToken(v) => ("nativeToken", v.to_string()),
            Self::PageSize(v) => ("pageSize", v.to_string()),
            Self::UnlockableByAddress(v) => ("unlockableByAddress", v.to_string()),
        }
    }

    pub(crate) fn kind(&self) -> u8 {
        match self {
            Self::CreatedAfter(_) => 0,
            Self::CreatedBefore(_) => 1,
            Self::Cursor(_) => 2,
            Self::HasNativeToken(_) => 3,
            Self::NativeToken(_) => 4,
            Self::PageSize(_) => 5,
            Self::UnlockableByAddress(_) => 6,
        }
    }
}

/// Query parameters for output_id requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicOutputsQueryParameters(Vec<BasicOutputsQueryParameter>);

impl_query_parameters_methods!(BasicOutputsQueryParameters, BasicOutputsQueryParameter);

/// Query parameter for output requests.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum BasicOutputsQueryParameter {
    /// Bech32-encoded address that should be searched for.
    Address(Bech32Address),
    /// Returns outputs that were created after a certain slot index.
    CreatedAfter(SlotIndex),
    /// Returns outputs that were created before a certain slot index.
    CreatedBefore(SlotIndex),
    /// Starts the search from the cursor (confirmationMS+outputId.pageSize).
    Cursor(String),
    /// Filters outputs based on the presence of a specific Bech32-encoded return address in the expiration unlock
    /// condition.
    ExpirationReturnAddress(Bech32Address),
    /// Returns outputs that expire after a certain slot index.
    ExpiresAfter(SlotIndex),
    /// Returns outputs that expire before a certain slot index.
    ExpiresBefore(SlotIndex),
    /// Filters outputs based on the presence of expiration unlock condition.
    HasExpiration(bool),
    /// Filters outputs based on the presence of a native token.
    HasNativeToken(bool),
    /// Filters outputs based on the presence of storage deposit return unlock condition.
    HasStorageDepositReturn(bool),
    /// Filters outputs based on the presence of timelock unlock condition.
    HasTimelock(bool),
    /// Filters outputs based on the presence of a specific native token.
    NativeToken(TokenId),
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    PageSize(usize),
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    Sender(Bech32Address),
    /// Filters outputs based on the presence of a specific return address in the storage deposit return unlock
    /// condition.
    StorageDepositReturnAddress(Bech32Address),
    /// Filters outputs based on matching Tag Block.
    Tag(String),
    /// Returns outputs that are timelocked after a certain slot index.
    TimelockedAfter(SlotIndex),
    /// Returns outputs that are timelocked before a certain slot index.
    TimelockedBefore(SlotIndex),
    /// Returns outputs that are unlockable by the bech32 address.
    UnlockableByAddress(Bech32Address),
}

impl BasicOutputsQueryParameter {
    fn to_query_tuple(&self) -> (&'static str, String) {
        match self {
            Self::Address(v) => ("address", v.to_string()),
            Self::CreatedAfter(v) => ("createdAfter", v.to_string()),
            Self::CreatedBefore(v) => ("createdBefore", v.to_string()),
            Self::Cursor(v) => ("cursor", v.to_string()),
            Self::ExpirationReturnAddress(v) => ("expirationReturnAddress", v.to_string()),
            Self::ExpiresAfter(v) => ("expiresAfter", v.to_string()),
            Self::ExpiresBefore(v) => ("expiresBefore", v.to_string()),
            Self::HasExpiration(v) => ("hasExpiration", v.to_string()),
            Self::HasNativeToken(v) => ("hasNativeToken", v.to_string()),
            Self::HasStorageDepositReturn(v) => ("hasStorageDepositReturn", v.to_string()),
            Self::HasTimelock(v) => ("hasTimelock", v.to_string()),
            Self::NativeToken(v) => ("nativeToken", v.to_string()),
            Self::PageSize(v) => ("pageSize", v.to_string()),
            Self::Sender(v) => ("sender", v.to_string()),
            Self::StorageDepositReturnAddress(v) => ("storageDepositReturnAddress", v.to_string()),
            Self::Tag(v) => ("tag", v.to_string()),
            Self::TimelockedAfter(v) => ("timelockedAfter", v.to_string()),
            Self::TimelockedBefore(v) => ("timelockedBefore", v.to_string()),
            Self::UnlockableByAddress(v) => ("unlockableByAddress", v.to_string()),
        }
    }

    pub(crate) fn kind(&self) -> u8 {
        match self {
            Self::Address(_) => 0,
            Self::CreatedAfter(_) => 1,
            Self::CreatedBefore(_) => 2,
            Self::Cursor(_) => 3,
            Self::ExpirationReturnAddress(_) => 4,
            Self::ExpiresAfter(_) => 5,
            Self::ExpiresBefore(_) => 6,
            Self::HasExpiration(_) => 7,
            Self::HasNativeToken(_) => 8,
            Self::HasStorageDepositReturn(_) => 9,
            Self::HasTimelock(_) => 10,
            Self::NativeToken(_) => 11,
            Self::PageSize(_) => 12,
            Self::Sender(_) => 13,
            Self::StorageDepositReturnAddress(_) => 14,
            Self::Tag(_) => 15,
            Self::TimelockedAfter(_) => 16,
            Self::TimelockedBefore(_) => 17,
            Self::UnlockableByAddress(_) => 18,
        }
    }
}

/// Query parameters for output_id requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftOutputsQueryParameters(Vec<NftOutputsQueryParameter>);

impl_query_parameters_methods!(NftOutputsQueryParameters, NftOutputsQueryParameter);

/// Query parameter for output requests.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum NftOutputsQueryParameter {
    /// Bech32-encoded address that should be searched for.
    Address(Bech32Address),
    /// Returns outputs that were created after a certain slot index.
    CreatedAfter(SlotIndex),
    /// Returns outputs that were created before a certain slot index.
    CreatedBefore(SlotIndex),
    /// Starts the search from the cursor (confirmationMS+outputId.pageSize).
    Cursor(String),
    /// Filters outputs based on the presence of a specific Bech32-encoded return address in the expiration unlock
    /// condition.
    ExpirationReturnAddress(Bech32Address),
    /// Returns outputs that expire after a certain slot index.
    ExpiresAfter(SlotIndex),
    /// Returns outputs that expire before a certain slot index.
    ExpiresBefore(SlotIndex),
    /// Filters outputs based on the presence of expiration unlock condition.
    HasExpiration(bool),
    /// Filters outputs based on the presence of storage deposit return unlock condition.
    HasStorageDepositReturn(bool),
    /// Filters outputs based on the presence of timelock unlock condition.
    HasTimelock(bool),
    /// Filters outputs based on bech32-encoded issuer address.
    Issuer(Bech32Address),
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    PageSize(usize),
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    Sender(Bech32Address),
    /// Filters outputs based on the presence of a specific return address in the storage deposit return unlock
    /// condition.
    StorageDepositReturnAddress(Bech32Address),
    /// Filters outputs based on matching Tag Block.
    Tag(String),
    /// Returns outputs that are timelocked after a certain slot index.
    TimelockedAfter(SlotIndex),
    /// Returns outputs that are timelocked before a certain slot index.
    TimelockedBefore(SlotIndex),
    /// Returns outputs that are unlockable by the bech32 address.
    UnlockableByAddress(Bech32Address),
}

impl NftOutputsQueryParameter {
    fn to_query_tuple(&self) -> (&'static str, String) {
        match self {
            Self::Address(v) => ("address", v.to_string()),
            Self::CreatedAfter(v) => ("createdAfter", v.to_string()),
            Self::CreatedBefore(v) => ("createdBefore", v.to_string()),
            Self::Cursor(v) => ("cursor", v.to_string()),
            Self::ExpirationReturnAddress(v) => ("expirationReturnAddress", v.to_string()),
            Self::ExpiresAfter(v) => ("expiresAfter", v.to_string()),
            Self::ExpiresBefore(v) => ("expiresBefore", v.to_string()),
            Self::HasExpiration(v) => ("hasExpiration", v.to_string()),
            Self::HasStorageDepositReturn(v) => ("hasStorageDepositReturn", v.to_string()),
            Self::HasTimelock(v) => ("hasTimelock", v.to_string()),
            Self::Issuer(v) => ("issuer", v.to_string()),
            Self::PageSize(v) => ("pageSize", v.to_string()),
            Self::Sender(v) => ("sender", v.to_string()),
            Self::StorageDepositReturnAddress(v) => ("storageDepositReturnAddress", v.to_string()),
            Self::Tag(v) => ("tag", v.to_string()),
            Self::TimelockedAfter(v) => ("timelockedAfter", v.to_string()),
            Self::TimelockedBefore(v) => ("timelockedBefore", v.to_string()),
            Self::UnlockableByAddress(v) => ("unlockableByAddress", v.to_string()),
        }
    }

    pub(crate) fn kind(&self) -> u8 {
        match self {
            Self::Address(_) => 0,
            Self::CreatedAfter(_) => 1,
            Self::CreatedBefore(_) => 2,
            Self::Cursor(_) => 3,
            Self::ExpirationReturnAddress(_) => 4,
            Self::ExpiresAfter(_) => 5,
            Self::ExpiresBefore(_) => 6,
            Self::HasExpiration(_) => 7,
            Self::HasStorageDepositReturn(_) => 9,
            Self::HasTimelock(_) => 10,
            Self::Issuer(_) => 11,
            Self::PageSize(_) => 12,
            Self::Sender(_) => 13,
            Self::StorageDepositReturnAddress(_) => 14,
            Self::Tag(_) => 15,
            Self::TimelockedAfter(_) => 16,
            Self::TimelockedBefore(_) => 17,
            Self::UnlockableByAddress(_) => 18,
        }
    }
}

/// Query parameters for output_id requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOutputsQueryParameters(Vec<AccountOutputsQueryParameter>);

impl_query_parameters_methods!(AccountOutputsQueryParameters, AccountOutputsQueryParameter);

/// Query parameter for output requests.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum AccountOutputsQueryParameter {
    /// Returns outputs that were created after a certain slot index.
    CreatedAfter(SlotIndex),
    /// Returns outputs that were created before a certain slot index.
    CreatedBefore(SlotIndex),
    /// Starts the search from the cursor (confirmationMS+outputId.pageSize).
    Cursor(String),
    /// Filters outputs based on bech32-encoded governor (governance controller) address.
    Governor(Bech32Address),
    /// Filters outputs based on bech32-encoded issuer address.
    Issuer(Bech32Address),
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    PageSize(usize),
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    Sender(Bech32Address),
    /// Filters outputs based on bech32-encoded state controller address.
    StateController(Bech32Address),
    /// Returns outputs that are unlockable by the bech32 address.
    UnlockableByAddress(Bech32Address),
}

impl AccountOutputsQueryParameter {
    fn to_query_tuple(&self) -> (&'static str, String) {
        match self {
            Self::CreatedAfter(v) => ("createdAfter", v.to_string()),
            Self::CreatedBefore(v) => ("createdBefore", v.to_string()),
            Self::Cursor(v) => ("cursor", v.to_string()),
            Self::Governor(v) => ("governor", v.to_string()),
            Self::Issuer(v) => ("issuer", v.to_string()),
            Self::PageSize(v) => ("pageSize", v.to_string()),
            Self::Sender(v) => ("sender", v.to_string()),
            Self::StateController(v) => ("stateController", v.to_string()),
            Self::UnlockableByAddress(v) => ("unlockableByAddress", v.to_string()),
        }
    }

    pub(crate) fn kind(&self) -> u8 {
        match self {
            Self::CreatedAfter(_) => 0,
            Self::CreatedBefore(_) => 1,
            Self::Cursor(_) => 2,
            Self::Governor(_) => 3,
            Self::Issuer(_) => 4,
            Self::PageSize(_) => 5,
            Self::Sender(_) => 6,
            Self::StateController(_) => 7,
            Self::UnlockableByAddress(_) => 8,
        }
    }
}

/// Query parameters for output_id requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundryOutputsQueryParameters(Vec<FoundryOutputsQueryParameter>);

impl_query_parameters_methods!(FoundryOutputsQueryParameters, FoundryOutputsQueryParameter);

/// Query parameter for output requests.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum FoundryOutputsQueryParameter {
    /// Filter foundry outputs based on bech32-encoded address of the controlling account.
    AccountAddress(Bech32Address),
    /// Returns outputs that were created after a certain slot index.
    CreatedAfter(SlotIndex),
    /// Returns outputs that were created before a certain slot index.
    CreatedBefore(SlotIndex),
    /// Starts the search from the cursor (confirmationMS+outputId.pageSize).
    Cursor(String),
    /// Filters outputs based on the presence of native token.
    HasNativeToken(bool),
    /// Filters outputs based on the presence of a specific native token.
    NativeToken(TokenId),
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    PageSize(usize),
}

impl FoundryOutputsQueryParameter {
    fn to_query_tuple(&self) -> (&'static str, String) {
        match self {
            Self::AccountAddress(v) => ("accountAddress", v.to_string()),
            Self::CreatedAfter(v) => ("createdAfter", v.to_string()),
            Self::CreatedBefore(v) => ("createdBefore", v.to_string()),
            Self::Cursor(v) => ("cursor", v.to_string()),
            Self::HasNativeToken(v) => ("hasNativeToken", v.to_string()),
            Self::NativeToken(v) => ("nativeToken", v.to_string()),
            Self::PageSize(v) => ("pageSize", v.to_string()),
        }
    }

    pub(crate) fn kind(&self) -> u8 {
        match self {
            Self::AccountAddress(_) => 0,
            Self::CreatedAfter(_) => 1,
            Self::CreatedBefore(_) => 2,
            Self::Cursor(_) => 3,
            Self::HasNativeToken(_) => 4,
            Self::NativeToken(_) => 5,
            Self::PageSize(_) => 6,
        }
    }
}

/// Query parameters for output_id requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationOutputsQueryParameters(Vec<DelegationOutputsQueryParameter>);

impl_query_parameters_methods!(DelegationOutputsQueryParameters, DelegationOutputsQueryParameter);

/// Query parameter for output requests.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum DelegationOutputsQueryParameter {
    /// Filter foundry outputs based on bech32-encoded address of the controlling account.
    Address(Bech32Address),
    /// Filter foundry outputs based on bech32-encoded address of the validator.
    Validator(Bech32Address),
    /// Returns outputs that were created after a certain slot index.
    CreatedAfter(SlotIndex),
    /// Returns outputs that were created before a certain slot index.
    CreatedBefore(SlotIndex),
    /// Starts the search from the cursor (confirmationMS+outputId.pageSize).
    Cursor(String),
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    PageSize(usize),
}

impl DelegationOutputsQueryParameter {
    fn to_query_tuple(&self) -> (&'static str, String) {
        match self {
            Self::Address(v) => ("address", v.to_string()),
            Self::Validator(v) => ("validator", v.to_string()),
            Self::CreatedAfter(v) => ("createdAfter", v.to_string()),
            Self::CreatedBefore(v) => ("createdBefore", v.to_string()),
            Self::Cursor(v) => ("cursor", v.to_string()),
            Self::PageSize(v) => ("pageSize", v.to_string()),
        }
    }

    pub(crate) fn kind(&self) -> u8 {
        match self {
            Self::Address(_) => 0,
            Self::Validator(_) => 1,
            Self::CreatedAfter(_) => 2,
            Self::CreatedBefore(_) => 3,
            Self::Cursor(_) => 4,
            Self::PageSize(_) => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_parameter() {
        let address1 = BasicOutputsQueryParameter::Address(
            Bech32Address::try_from_str("atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r").unwrap(),
        );
        let address2 = BasicOutputsQueryParameter::Address(
            Bech32Address::try_from_str("atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r").unwrap(),
        );
        let address3 = BasicOutputsQueryParameter::Address(
            Bech32Address::try_from_str("atoi1qprxpfvaz2peggq6f8k9cj8zfsxuw69e4nszjyv5kuf8yt70t2847shpjak").unwrap(),
        );
        let state_controller = BasicOutputsQueryParameter::UnlockableByAddress(
            Bech32Address::try_from_str("atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r").unwrap(),
        );

        let mut query_parameters = BasicOutputsQueryParameters::new([address1, address2, state_controller]);
        // since address1 and address2 are of the same enum variant, we should only have one
        assert!(query_parameters.0.len() == 2);
        // since address2 and address3 are of the same enum variant, we should only have one
        query_parameters.replace(address3);
        assert!(query_parameters.0.len() == 2);
        // Contains address query parameter
        assert!(query_parameters
            .0
            .iter()
            .any(|param| matches!(param, BasicOutputsQueryParameter::Address(_))));
        // Contains no cursor query parameter
        assert!(!query_parameters
            .0
            .iter()
            .any(|param| matches!(param, BasicOutputsQueryParameter::Cursor(_))));
    }
}
