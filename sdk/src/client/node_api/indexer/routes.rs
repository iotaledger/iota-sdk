// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node indexer routes

use super::query_parameters::{BasicOutputsQueryParameters, OutputsQueryParameters};
use crate::{
    client::{
        node_api::indexer::query_parameters::{
            AccountOutputsQueryParameter, AccountOutputsQueryParameters, BasicOutputsQueryParameter,
            FoundryOutputsQueryParameter, FoundryOutputsQueryParameters, NftOutputsQueryParameter,
            NftOutputsQueryParameters, OutputsQueryParameter,
        },
        ClientInner, Error, Result,
    },
    types::{
        api::plugins::indexer::OutputIdsResponse,
        block::output::{AccountId, FoundryId, NftId, OutputId},
    },
};

// hornet: https://github.com/gohornet/hornet/blob/develop/plugins/indexer/routes.go

impl ClientInner {
    /// Get basic, alias, nft and foundry outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Query parameters: "hasNativeTokens", "minNativeTokenCount", "maxNativeTokenCount", "unlockableByAddress",
    /// "createdBefore", "createdAfter", "cursor", "pageSize".
    /// Returns Err(Node(NotFound) if no results are found.
    /// api/indexer/v1/outputs
    pub async fn output_ids(
        &self,
        query_parameters: impl Into<Vec<OutputsQueryParameter>>,
    ) -> Result<OutputIdsResponse> {
        let route = "api/indexer/v1/outputs";

        self.get_output_ids(route, OutputsQueryParameters::new(query_parameters), true, false)
            .await
    }

    /// Get basic outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Query parameters: "address", "hasStorageDepositReturn", "storageDepositReturnAddress",
    /// "hasExpiration", "expiresBefore", "expiresAfter", "hasTimelock", "timelockedBefore",
    /// "timelockedAfter", "sender", "tag", "createdBefore" and "createdAfter".
    /// Returns Err(Node(NotFound) if no results are found.
    /// api/indexer/v2/outputs/basic
    pub async fn basic_output_ids(
        &self,
        query_parameters: impl Into<Vec<BasicOutputsQueryParameter>>,
    ) -> Result<OutputIdsResponse> {
        let route = "api/indexer/v2/outputs/basic";

        self.get_output_ids(route, BasicOutputsQueryParameters::new(query_parameters), true, false)
            .await
    }

    /// Get account outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Query parameters: "stateController", "governor", "issuer", "sender", "createdBefore", "createdAfter"
    /// Returns Err(Node(NotFound) if no results are found.
    /// api/indexer/v2/outputs/account
    pub async fn account_output_ids(
        &self,
        query_parameters: impl Into<Vec<AccountOutputsQueryParameter>>,
    ) -> Result<OutputIdsResponse> {
        let route = "api/indexer/v2/outputs/account";

        self.get_output_ids(route, AccountOutputsQueryParameters::new(query_parameters), true, false)
            .await
    }

    /// Get account output by its accountID.
    /// api/indexer/v2/outputs/account/:{AccountId}
    pub async fn account_output_id(&self, account_id: AccountId) -> Result<OutputId> {
        let route = format!("api/indexer/v2/outputs/account/{account_id}");

        Ok(*(self
            .get_output_ids(&route, AccountOutputsQueryParameters::empty(), true, false)
            .await?
            .first()
            .ok_or_else(|| Error::NoOutput(format!("{account_id:?}")))?))
    }

    /// Get foundry outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Query parameters: "address", "createdBefore", "createdAfter"
    /// Returns Err(Node(NotFound) if no results are found.
    /// api/indexer/v2/outputs/foundry
    pub async fn foundry_output_ids(
        &self,
        query_parameters: impl Into<Vec<FoundryOutputsQueryParameter>>,
    ) -> Result<OutputIdsResponse> {
        let route = "api/indexer/v2/outputs/foundry";

        self.get_output_ids(route, FoundryOutputsQueryParameters::new(query_parameters), true, false)
            .await
    }

    /// Get foundry output by its foundryID.
    /// api/indexer/v2/outputs/foundry/:{FoundryID}
    pub async fn foundry_output_id(&self, foundry_id: FoundryId) -> Result<OutputId> {
        let route = format!("api/indexer/v2/outputs/foundry/{foundry_id}");

        Ok(*(self
            .get_output_ids(&route, FoundryOutputsQueryParameters::empty(), true, false)
            .await?
            .first()
            .ok_or_else(|| Error::NoOutput(format!("{foundry_id:?}")))?))
    }

    /// Get NFT outputs filtered by the given parameters.
    /// Query parameters: "address", "hasStorageDepositReturn", "storageDepositReturnAddress",
    /// "hasExpiration", "expiresBefore", "expiresAfter", "hasTimelock", "timelockedBefore",
    /// "timelockedAfter", "issuer", "sender", "tag", "createdBefore", "createdAfter"
    /// Returns Err(Node(NotFound) if no results are found.
    /// api/indexer/v2/outputs/nft
    pub async fn nft_output_ids(
        &self,
        query_parameters: impl Into<Vec<NftOutputsQueryParameter>>,
    ) -> Result<OutputIdsResponse> {
        let route = "api/indexer/v2/outputs/nft";

        self.get_output_ids(route, NftOutputsQueryParameters::new(query_parameters), true, false)
            .await
    }

    /// Get NFT output by its nftID.
    /// api/indexer/v2/outputs/nft/:{NftId}
    pub async fn nft_output_id(&self, nft_id: NftId) -> Result<OutputId> {
        let route = format!("api/indexer/v2/outputs/nft/{nft_id}");

        Ok(*(self
            .get_output_ids(&route, NftOutputsQueryParameters::empty(), true, false)
            .await?
            .first()
            .ok_or_else(|| Error::NoOutput(format!("{nft_id:?}")))?))
    }
}
