// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Node core API routes.

use packable::PackableExt;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    client::{
        constants::{DEFAULT_API_TIMEOUT, DEFAULT_USER_AGENT},
        node_api::query_tuples_to_query_string,
        node_manager::node::{Node, NodeAuth},
        Client, ClientError, ClientInner,
    },
    types::{
        api::core::{
            BlockMetadataResponse, BlockWithMetadataResponse, CommitteeResponse, CongestionResponse, InfoResponse,
            IssuanceBlockHeaderResponse, ManaRewardsResponse, NetworkMetricsResponse, OutputResponse,
            OutputWithMetadataResponse, PermanodeInfoResponse, RoutesResponse, SubmitBlockResponse,
            TransactionMetadataResponse, UtxoChangesFullResponse, UtxoChangesResponse, ValidatorResponse,
            ValidatorsResponse,
        },
        block::{
            address::ToBech32Ext,
            output::{AccountId, OutputId, OutputMetadata},
            payload::signed_transaction::TransactionId,
            slot::{EpochIndex, SlotCommitment, SlotCommitmentId, SlotIndex},
            Block, BlockDto, BlockId,
        },
        TryFromDto,
    },
};

/// Info path is the exact path extension for node APIs to request their info.
pub(crate) static INFO_PATH: &str = "api/core/v3/info";

/// Contains the info and the url from the node (useful when multiple nodes are used)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfoResponse {
    /// The returned info
    pub info: InfoResponse,
    /// The url from the node which returned the info
    pub url: String,
}

impl ClientInner {
    // Node routes.

    /// Returns the health of the node.
    /// GET /health
    pub async fn get_health(&self, url: &str) -> Result<bool, ClientError> {
        const PATH: &str = "health";

        let mut url = Url::parse(url)?;
        url.set_path(PATH);
        let status = crate::client::node_manager::http_client::HttpClient::new(DEFAULT_USER_AGENT.to_string())
            .get(
                &Node {
                    url,
                    auth: None,
                    disabled: false,
                    permanode: false,
                },
                DEFAULT_API_TIMEOUT,
            )
            .await?
            .status();

        match status {
            200 => Ok(true),
            _ => Ok(false),
        }
    }

    /// Returns the available API route groups of the node.
    /// GET /api/routes
    pub async fn get_routes(&self) -> Result<RoutesResponse, ClientError> {
        const PATH: &str = "api/routes";

        self.get_request(PATH, None, false).await
    }

    /// Returns general information about the node.
    /// GET /api/core/v3/info
    pub async fn get_node_info(&self) -> Result<NodeInfoResponse, ClientError> {
        self.get_request(INFO_PATH, None, false).await
    }

    /// Returns network metrics.
    /// GET /api/core/v3/network/metrics
    pub async fn get_network_metrics(&self) -> Result<NetworkMetricsResponse, ClientError> {
        const PATH: &str = "api/core/v3/network/metrics";

        self.get_request(PATH, None, false).await
    }

    /// Returns whether the network is healthy (finalization is not delayed).
    /// GET /api/core/v3/network/health
    pub async fn get_network_health(&self) -> Result<bool, ClientError> {
        const PATH: &str = "api/core/v3/network/health";

        let nodes = self.node_manager.read().await.get_nodes(PATH, None)?;
        let client = crate::client::node_manager::http_client::HttpClient::new(DEFAULT_USER_AGENT.to_string());

        for node in &nodes {
            if let Ok(res) = client.get(node, DEFAULT_API_TIMEOUT).await {
                if res.status() == 200 {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

impl Client {
    // Accounts routes.

    /// Checks if the account is ready to issue a block.
    /// GET /api/core/v3/accounts/{bech32Address}/congestion
    pub async fn get_account_congestion(
        &self,
        account_id: &AccountId,
        work_score: impl Into<Option<u32>> + Send,
    ) -> Result<CongestionResponse, ClientError> {
        let bech32_address = account_id.to_bech32(self.get_bech32_hrp().await?);
        let path = &format!("api/core/v3/accounts/{bech32_address}/congestion");
        let query = query_tuples_to_query_string([work_score.into().map(|s| ("workScore", s.to_string()))]);

        self.get_request(path, query.as_deref(), false).await
    }

    // Rewards routes.

    /// Returns the total available Mana rewards of an account or delegation output decayed up to `epochEnd` index
    /// provided in the response.
    /// Note that rewards for an epoch only become available at the beginning of the next epoch. If the end epoch of a
    /// staking feature is equal or greater than the current epoch, the rewards response will not include the potential
    /// future rewards for those epochs. `epochStart` and `epochEnd` indicates the actual range for which reward value
    /// is returned and decayed for.
    /// GET /api/core/v3/rewards/{outputId}
    pub async fn get_output_mana_rewards(
        &self,
        output_id: &OutputId,
        slot: impl Into<Option<SlotIndex>> + Send,
    ) -> Result<ManaRewardsResponse, ClientError> {
        let path = &format!("api/core/v3/rewards/{output_id}");
        let query = query_tuples_to_query_string([slot.into().map(|i| ("slot", i.to_string()))]);

        self.get_request(path, query.as_deref(), false).await
    }

    // Validators routes.

    /// Returns information of all stakers (registered validators) and if they are active, ordered by their holding
    /// stake.
    /// GET /api/core/v3/validators
    pub async fn get_validators(
        &self,
        page_size: impl Into<Option<u32>> + Send,
        cursor: impl Into<Option<String>> + Send,
    ) -> Result<ValidatorsResponse, ClientError> {
        const PATH: &str = "api/core/v3/validators";
        let query = query_tuples_to_query_string([
            page_size.into().map(|n| ("pageSize", n.to_string())),
            cursor.into().map(|c| ("cursor", c)),
        ]);

        self.get_request(PATH, query.as_deref(), false).await
    }

    /// Return information about a staker (registered validator).
    /// GET /api/core/v3/validators/{bech32Address}
    pub async fn get_validator(&self, account_id: &AccountId) -> Result<ValidatorResponse, ClientError> {
        let bech32_address = account_id.to_bech32(self.get_bech32_hrp().await?);
        let path = &format!("api/core/v3/validators/{bech32_address}");

        self.get_request(path, None, false).await
    }

    // Committee routes.

    /// Returns the information of committee members at the given epoch index. If epoch index is not provided, the
    /// current committee members are returned.
    /// GET /api/core/v3/committee/?epoch
    pub async fn get_committee(
        &self,
        epoch: impl Into<Option<EpochIndex>> + Send,
    ) -> Result<CommitteeResponse, ClientError> {
        const PATH: &str = "api/core/v3/committee";
        let query = query_tuples_to_query_string([epoch.into().map(|i| ("epoch", i.to_string()))]);

        self.get_request(PATH, query.as_deref(), false).await
    }

    // Blocks routes.

    /// Returns information that is ideal for attaching a block in the network.
    /// GET /api/core/v3/blocks/issuance
    pub async fn get_issuance(&self) -> Result<IssuanceBlockHeaderResponse, ClientError> {
        const PATH: &str = "api/core/v3/blocks/issuance";

        self.get_request(PATH, None, false).await
    }

    /// Returns the BlockId of the submitted block.
    /// POST /api/core/v3/blocks
    pub async fn post_block(&self, block: &Block) -> Result<BlockId, ClientError> {
        const PATH: &str = "api/core/v3/blocks";

        let block_dto = BlockDto::from(block);

        let response = self
            .post_request::<SubmitBlockResponse>(PATH, serde_json::to_value(block_dto)?)
            .await?;

        Ok(response.block_id)
    }

    /// Returns the BlockId of the submitted block.
    /// POST /api/core/v3/blocks
    pub async fn post_block_raw(&self, block: &Block) -> Result<BlockId, ClientError> {
        const PATH: &str = "api/core/v3/blocks";

        let response = self
            .post_request_bytes::<SubmitBlockResponse>(PATH, &block.pack_to_vec())
            .await?;

        Ok(response.block_id)
    }

    /// Finds a block by its ID and returns it as object.
    /// GET /api/core/v3/blocks/{blockId}
    pub async fn get_block(&self, block_id: &BlockId) -> Result<Block, ClientError> {
        let path = &format!("api/core/v3/blocks/{block_id}");

        let dto = self.get_request::<BlockDto>(path, None, false).await?;

        Ok(Block::try_from_dto_with_params(
            dto,
            &self.get_protocol_parameters().await?,
        )?)
    }

    /// Finds a block by its ID and returns it as raw bytes.
    /// GET /api/core/v3/blocks/{blockId}
    pub async fn get_block_raw(&self, block_id: &BlockId) -> Result<Vec<u8>, ClientError> {
        let path = &format!("api/core/v3/blocks/{block_id}");

        self.get_request_bytes(path, None).await
    }

    /// Returns the metadata of a block.
    /// GET /api/core/v3/blocks/{blockId}/metadata
    pub async fn get_block_metadata(&self, block_id: &BlockId) -> Result<BlockMetadataResponse, ClientError> {
        let path = &format!("api/core/v3/blocks/{block_id}/metadata");

        self.get_request(path, None, true).await
    }

    /// Returns a block with its metadata.
    /// GET /api/core/v3/blocks/{blockId}/full
    pub async fn get_block_with_metadata(&self, block_id: &BlockId) -> Result<BlockWithMetadataResponse, ClientError> {
        let path = &format!("api/core/v3/blocks/{block_id}/full");

        self.get_request(path, None, true).await
    }

    // UTXO routes.

    /// Finds an output by its ID and returns it as object.
    /// GET /api/core/v3/outputs/{outputId}
    pub async fn get_output(&self, output_id: &OutputId) -> Result<OutputResponse, ClientError> {
        let path = &format!("api/core/v3/outputs/{output_id}");

        self.get_request(path, None, false).await
    }

    /// Finds an output by its ID and returns it as raw bytes.
    /// GET /api/core/v3/outputs/{outputId}
    pub async fn get_output_raw(&self, output_id: &OutputId) -> Result<Vec<u8>, ClientError> {
        let path = &format!("api/core/v3/outputs/{output_id}");

        self.get_request_bytes(path, None).await
    }

    /// Finds output metadata by output ID.
    /// GET /api/core/v3/outputs/{outputId}/metadata
    pub async fn get_output_metadata(&self, output_id: &OutputId) -> Result<OutputMetadata, ClientError> {
        let path = &format!("api/core/v3/outputs/{output_id}/metadata");

        self.get_request(path, None, false).await
    }

    /// Finds an output with its metadata by output ID.
    /// GET /api/core/v3/outputs/{outputId}/full
    pub async fn get_output_with_metadata(
        &self,
        output_id: &OutputId,
    ) -> Result<OutputWithMetadataResponse, ClientError> {
        let path = &format!("api/core/v3/outputs/{output_id}/full");

        self.get_request(path, None, false).await
    }

    /// Returns the earliest confirmed block containing the transaction with the given ID.
    /// GET /api/core/v3/transactions/{transactionId}/included-block
    pub async fn get_included_block(&self, transaction_id: &TransactionId) -> Result<Block, ClientError> {
        let path = &format!("api/core/v3/transactions/{transaction_id}/included-block");

        let dto = self.get_request::<BlockDto>(path, None, true).await?;

        Ok(Block::try_from_dto_with_params(
            dto,
            &self.get_protocol_parameters().await?,
        )?)
    }

    /// Returns the earliest confirmed block containing the transaction with the given ID, as raw bytes.
    /// GET /api/core/v3/transactions/{transactionId}/included-block
    pub async fn get_included_block_raw(&self, transaction_id: &TransactionId) -> Result<Vec<u8>, ClientError> {
        let path = &format!("api/core/v3/transactions/{transaction_id}/included-block");

        self.get_request_bytes(path, None).await
    }

    /// Returns the metadata of the earliest block containing the tx that was confirmed.
    /// GET /api/core/v3/transactions/{transactionId}/included-block/metadata
    pub async fn get_included_block_metadata(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<BlockMetadataResponse, ClientError> {
        let path = &format!("api/core/v3/transactions/{transaction_id}/included-block/metadata");

        self.get_request(path, None, true).await
    }

    /// Finds the metadata of a transaction.
    /// GET /api/core/v3/transactions/{transactionId}/metadata
    pub async fn get_transaction_metadata(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<TransactionMetadataResponse, ClientError> {
        let path = &format!("api/core/v3/transactions/{transaction_id}/metadata");

        self.get_request(path, None, true).await
    }

    // Commitments routes.

    /// Finds a slot commitment by its ID and returns it as object.
    /// GET /api/core/v3/commitments/{commitmentId}
    pub async fn get_commitment(&self, commitment_id: &SlotCommitmentId) -> Result<SlotCommitment, ClientError> {
        let path = &format!("api/core/v3/commitments/{commitment_id}");

        self.get_request(path, None, false).await
    }

    /// Finds a slot commitment by its ID and returns it as raw bytes.
    /// GET /api/core/v3/commitments/{commitmentId}
    pub async fn get_commitment_raw(&self, commitment_id: &SlotCommitmentId) -> Result<Vec<u8>, ClientError> {
        let path = &format!("api/core/v3/commitments/{commitment_id}");

        self.get_request_bytes(path, None).await
    }

    /// Get all UTXO changes of a given slot by slot commitment ID.
    /// GET /api/core/v3/commitments/{commitmentId}/utxo-changes
    pub async fn get_utxo_changes(&self, commitment_id: &SlotCommitmentId) -> Result<UtxoChangesResponse, ClientError> {
        let path = &format!("api/core/v3/commitments/{commitment_id}/utxo-changes");

        self.get_request(path, None, false).await
    }

    /// Get all full UTXO changes of a given slot by slot commitment ID.
    /// GET /api/core/v3/commitments/{commitmentId}/utxo-changes/full
    pub async fn get_utxo_changes_full(
        &self,
        commitment_id: &SlotCommitmentId,
    ) -> Result<UtxoChangesFullResponse, ClientError> {
        let path = &format!("api/core/v3/commitments/{commitment_id}/utxo-changes/full");

        self.get_request(path, None, false).await
    }

    /// Finds a slot commitment by slot index and returns it as object.
    /// GET /api/core/v3/commitments/by-slot/{slot}
    pub async fn get_commitment_by_slot(&self, slot: SlotIndex) -> Result<SlotCommitment, ClientError> {
        let path = &format!("api/core/v3/commitments/by-slot/{slot}");

        self.get_request(path, None, false).await
    }

    /// Finds a slot commitment by slot index and returns it as raw bytes.
    /// GET /api/core/v3/commitments/by-slot/{slot}
    pub async fn get_commitment_by_slot_raw(&self, slot: SlotIndex) -> Result<Vec<u8>, ClientError> {
        let path = &format!("api/core/v3/commitments/by-slot/{slot}");

        self.get_request_bytes(path, None).await
    }

    /// Get all UTXO changes of a given slot by its index.
    /// GET /api/core/v3/commitments/by-slot/{slot}/utxo-changes
    pub async fn get_utxo_changes_by_slot(&self, slot: SlotIndex) -> Result<UtxoChangesResponse, ClientError> {
        let path = &format!("api/core/v3/commitments/by-slot/{slot}/utxo-changes");

        self.get_request(path, None, false).await
    }

    /// Get all full UTXO changes of a given slot by its index.
    /// GET /api/core/v3/commitments/by-slot/{slot}/utxo-changes/full
    pub async fn get_utxo_changes_full_by_slot(&self, slot: SlotIndex) -> Result<UtxoChangesFullResponse, ClientError> {
        let path = &format!("api/core/v3/commitments/by-slot/{slot}/utxo-changes/full");

        self.get_request(path, None, false).await
    }
}

impl Client {
    /// GET /api/core/v3/info endpoint
    pub async fn get_info(url: &str, auth: Option<NodeAuth>) -> Result<InfoResponse, ClientError> {
        let mut url = crate::client::node_manager::builder::validate_url(Url::parse(url)?)?;
        if let Some(auth) = &auth {
            if let Some((name, password)) = &auth.basic_auth_name_pwd {
                url.set_username(name).map_err(|_| ClientError::UrlAuth("username"))?;
                url.set_password(Some(password))
                    .map_err(|_| ClientError::UrlAuth("password"))?;
            }
        }

        if url.path().ends_with('/') {
            url.set_path(&format!("{}{}", url.path(), INFO_PATH));
        } else {
            url.set_path(&format!("{}/{}", url.path(), INFO_PATH));
        }

        let resp: InfoResponse =
            crate::client::node_manager::http_client::HttpClient::new(DEFAULT_USER_AGENT.to_string())
                .get(
                    &Node {
                        url,
                        auth,
                        disabled: false,
                        permanode: false,
                    },
                    DEFAULT_API_TIMEOUT,
                )
                .await?
                .into_json()
                .await?;

        Ok(resp)
    }

    /// GET /api/core/v3/info endpoint
    pub(crate) async fn get_permanode_info(mut node: Node) -> Result<PermanodeInfoResponse, ClientError> {
        log::debug!("get_permanode_info");
        if let Some(auth) = &node.auth {
            if let Some((name, password)) = &auth.basic_auth_name_pwd {
                node.url
                    .set_username(name)
                    .map_err(|_| ClientError::UrlAuth("username"))?;
                node.url
                    .set_password(Some(password))
                    .map_err(|_| ClientError::UrlAuth("password"))?;
            }
        }

        if node.url.path().ends_with('/') {
            node.url.set_path(&format!("{}{}", node.url.path(), INFO_PATH));
        } else {
            node.url.set_path(&format!("{}/{}", node.url.path(), INFO_PATH));
        }

        let resp: PermanodeInfoResponse =
            crate::client::node_manager::http_client::HttpClient::new(DEFAULT_USER_AGENT.to_string())
                .get(&node, DEFAULT_API_TIMEOUT)
                .await?
                .into_json()
                .await?;

        Ok(resp)
    }
}
