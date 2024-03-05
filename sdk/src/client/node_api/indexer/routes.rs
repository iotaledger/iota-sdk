// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node indexer routes.

use crate::{
    client::{
        node_api::indexer::query_parameters::{
            AccountOutputQueryParameters, AnchorOutputQueryParameters, BasicOutputQueryParameters,
            DelegationOutputQueryParameters, FoundryOutputQueryParameters, NftOutputQueryParameters,
            OutputQueryParameters,
        },
        Client, ClientError,
    },
    types::{
        api::plugins::indexer::OutputIdsResponse,
        block::{
            address::ToBech32Ext,
            output::{AccountId, AnchorId, DelegationId, FoundryId, NftId, OutputId},
        },
    },
};

impl Client {
    /// Get account, anchor, basic, delegation, nft and foundry outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Returns Err(Node(NotFound) if no results are found.
    /// api/indexer/v2/outputs
    pub async fn output_ids(&self, query_parameters: OutputQueryParameters) -> Result<OutputIdsResponse, ClientError> {
        let route = "api/indexer/v2/outputs";

        self.get_output_ids(route, query_parameters, true).await
    }

    /// Get basic outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Returns Err(Node(NotFound) if no results are found.
    /// api/indexer/v2/outputs/basic
    pub async fn basic_output_ids(
        &self,
        query_parameters: BasicOutputQueryParameters,
    ) -> Result<OutputIdsResponse, ClientError> {
        let route = "api/indexer/v2/outputs/basic";

        self.get_output_ids(route, query_parameters, true).await
    }

    /// Get account outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Returns Err(Node(NotFound) if no results are found.
    /// api/indexer/v2/outputs/account
    pub async fn account_output_ids(
        &self,
        query_parameters: AccountOutputQueryParameters,
    ) -> Result<OutputIdsResponse, ClientError> {
        let route = "api/indexer/v2/outputs/account";

        self.get_output_ids(route, query_parameters, true).await
    }

    /// Get account output by its accountID.
    /// api/indexer/v2/outputs/account/{bech32Address}
    pub async fn account_output_id(&self, account_id: AccountId) -> Result<OutputId, ClientError> {
        let bech32_address = account_id.to_bech32(self.get_bech32_hrp().await?);
        let route = format!("api/indexer/v2/outputs/account/{bech32_address}");

        Ok(*(self
            .get_output_ids(&route, AccountOutputQueryParameters::new(), true)
            .await?
            .first()
            .ok_or_else(|| ClientError::NoOutput(format!("{account_id:?}")))?))
    }

    /// Get anchor outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Returns Err(Node(NotFound) if no results are found.
    /// api/indexer/v2/outputs/anchor
    pub async fn anchor_output_ids(
        &self,
        query_parameters: AnchorOutputQueryParameters,
    ) -> Result<OutputIdsResponse, ClientError> {
        let route = "api/indexer/v2/outputs/anchor";

        self.get_output_ids(route, query_parameters, true).await
    }

    /// Get anchor output by its anchorID.
    /// api/indexer/v2/outputs/anchor/{bech32Address}
    pub async fn anchor_output_id(&self, anchor_id: AnchorId) -> Result<OutputId, ClientError> {
        let bech32_address = anchor_id.to_bech32(self.get_bech32_hrp().await?);
        let route = format!("api/indexer/v2/outputs/anchor/{bech32_address}");

        Ok(*(self
            .get_output_ids(&route, AnchorOutputQueryParameters::new(), true)
            .await?
            .first()
            .ok_or_else(|| ClientError::NoOutput(format!("{anchor_id:?}")))?))
    }

    /// Get delegation outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Returns Err(Node(NotFound) if no results are found.
    /// api/indexer/v2/outputs/delegation
    pub async fn delegation_output_ids(
        &self,
        query_parameters: DelegationOutputQueryParameters,
    ) -> Result<OutputIdsResponse, ClientError> {
        let route = "api/indexer/v2/outputs/delegation";

        self.get_output_ids(route, query_parameters, true).await
    }

    /// Get delegation output by its delegationID.
    /// api/indexer/v2/outputs/delegation/:{DelegationId}
    pub async fn delegation_output_id(&self, delegation_id: DelegationId) -> Result<OutputId, ClientError> {
        let route = format!("api/indexer/v2/outputs/delegation/{delegation_id}");

        Ok(*(self
            .get_output_ids(&route, DelegationOutputQueryParameters::new(), true)
            .await?
            .first()
            .ok_or_else(|| ClientError::NoOutput(format!("{delegation_id:?}")))?))
    }

    /// Get foundry outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Returns Err(Node(NotFound) if no results are found.
    /// api/indexer/v2/outputs/foundry
    pub async fn foundry_output_ids(
        &self,
        query_parameters: FoundryOutputQueryParameters,
    ) -> Result<OutputIdsResponse, ClientError> {
        let route = "api/indexer/v2/outputs/foundry";

        self.get_output_ids(route, query_parameters, true).await
    }

    /// Get foundry output by its foundryID.
    /// api/indexer/v2/outputs/foundry/:{FoundryID}
    pub async fn foundry_output_id(&self, foundry_id: FoundryId) -> Result<OutputId, ClientError> {
        let route = format!("api/indexer/v2/outputs/foundry/{foundry_id}");

        Ok(*(self
            .get_output_ids(&route, FoundryOutputQueryParameters::new(), true)
            .await?
            .first()
            .ok_or_else(|| ClientError::NoOutput(format!("{foundry_id:?}")))?))
    }

    /// Get NFT outputs filtered by the given parameters.
    /// Returns Err(Node(NotFound) if no results are found.
    /// api/indexer/v2/outputs/nft
    pub async fn nft_output_ids(
        &self,
        query_parameters: NftOutputQueryParameters,
    ) -> Result<OutputIdsResponse, ClientError> {
        let route = "api/indexer/v2/outputs/nft";

        self.get_output_ids(route, query_parameters, true).await
    }

    /// Get NFT output by its nftID.
    /// api/indexer/v2/outputs/nft/{bech32Address}
    pub async fn nft_output_id(&self, nft_id: NftId) -> Result<OutputId, ClientError> {
        let bech32_address = nft_id.to_bech32(self.get_bech32_hrp().await?);
        let route = format!("api/indexer/v2/outputs/nft/{bech32_address}");

        Ok(*(self
            .get_output_ids(&route, NftOutputQueryParameters::new(), true)
            .await?
            .first()
            .ok_or_else(|| ClientError::NoOutput(format!("{nft_id:?}")))?))
    }
}
