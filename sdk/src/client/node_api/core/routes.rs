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
        Client, ClientInner, Result,
    },
    types::{
        api::core::{
            BlockMetadataResponse, BlockWithMetadataResponse, CommitteeResponse, CongestionResponse, InfoResponse,
            IssuanceBlockHeaderResponse, ManaRewardsResponse, OutputResponse, RoutesResponse, SubmitBlockResponse,
            TransactionMetadataResponse, UtxoChangesResponse, ValidatorResponse, ValidatorsResponse,
        },
        block::{
            address::ToBech32Ext,
            output::{AccountId, OutputId, OutputMetadata, OutputWithMetadata},
            payload::signed_transaction::TransactionId,
            slot::{EpochIndex, SlotCommitment, SlotCommitmentId, SlotIndex},
            Block, BlockDto, BlockId,
        },
        TryFromDto,
    },
};

/// Info path is the exact path extension for node APIs to request their info.
pub(crate) static INFO_PATH: &str = "api/core/v3/info";

/// NodeInfo wrapper which contains the node info and the url from the node (useful when multiple nodes are used)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfoWrapper {
    /// The returned node info
    pub node_info: InfoResponse,
    /// The url from the node which returned the node info
    pub url: String,
}

impl ClientInner {
    // Node routes.

    /// Returns the health of the node.
    /// GET /health
    pub async fn get_health(&self, url: &str) -> Result<bool> {
        const PATH: &str = "health";

        let mut url = Url::parse(url)?;
        url.set_path(PATH);
        let status = crate::client::node_manager::http_client::HttpClient::new(DEFAULT_USER_AGENT.to_string())
            .get(
                Node {
                    url,
                    auth: None,
                    disabled: false,
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
    pub async fn get_routes(&self) -> Result<RoutesResponse> {
        const PATH: &str = "api/routes";

        self.get_request(PATH, None, false, false).await
    }

    /// Returns general information about the node.
    /// GET /api/core/v3/info
    pub async fn get_info(&self) -> Result<NodeInfoWrapper> {
        self.get_request(INFO_PATH, None, false, false).await
    }

    /// Checks if the account is ready to issue a block.
    /// GET /api/core/v3/accounts/{bech32Address}/congestion
    pub async fn get_account_congestion(&self, account_id: &AccountId) -> Result<CongestionResponse> {
        let bech32_address = account_id.to_bech32(self.get_bech32_hrp().await?);
        let path = &format!("api/core/v3/accounts/{bech32_address}/congestion");

        self.get_request(path, None, false, false).await
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
        slot_index: impl Into<Option<SlotIndex>> + Send,
    ) -> Result<ManaRewardsResponse> {
        let path = &format!("api/core/v3/rewards/{output_id}");

        let query = query_tuples_to_query_string([slot_index.into().map(|i| ("slotIndex", i.to_string()))]);

        self.get_request(path, query.as_deref(), false, false).await
    }

    // Committee routes.

    /// Returns the information of committee members at the given epoch index. If epoch index is not provided, the
    /// current committee members are returned.
    /// GET /api/core/v3/committee/?epochIndex
    pub async fn get_committee(&self, epoch_index: impl Into<Option<EpochIndex>> + Send) -> Result<CommitteeResponse> {
        const PATH: &str = "api/core/v3/committee";

        let query = query_tuples_to_query_string([epoch_index.into().map(|i| ("epochIndex", i.to_string()))]);

        self.get_request(PATH, query.as_deref(), false, false).await
    }

    // Validators routes.

    /// Returns information of all registered validators and if they are active.
    /// GET JSON to /api/core/v3/validators
    pub async fn get_validators(
        &self,
        page_size: impl Into<Option<u32>> + Send,
        cursor: impl Into<Option<String>> + Send,
    ) -> Result<ValidatorsResponse> {
        const PATH: &str = "api/core/v3/validators";

        let query = query_tuples_to_query_string([
            page_size.into().map(|i| ("pageSize", i.to_string())),
            cursor.into().map(|i| ("cursor", i)),
        ]);

        self.get_request(PATH, query.as_deref(), false, false).await
    }

    /// Return information about a validator.
    /// GET /api/core/v3/validators/{bech32Address}
    pub async fn get_validator(&self, account_id: &AccountId) -> Result<ValidatorResponse> {
        let bech32_address = account_id.to_bech32(self.get_bech32_hrp().await?);
        let path = &format!("api/core/v3/validators/{bech32_address}");

        self.get_request(path, None, false, false).await
    }

    // Blocks routes.

    /// Returns information that is ideal for attaching a block in the network.
    /// GET /api/core/v3/blocks/issuance
    pub async fn get_issuance(&self) -> Result<IssuanceBlockHeaderResponse> {
        const PATH: &str = "api/core/v3/blocks/issuance";

        self.get_request(PATH, None, false, false).await
    }

    /// Returns the BlockId of the submitted block.
    /// POST JSON to /api/core/v3/blocks
    pub async fn post_block(&self, block: &Block) -> Result<BlockId> {
        const PATH: &str = "api/core/v3/blocks";

        let block_dto = BlockDto::from(block);

        let response = self
            .post_request::<SubmitBlockResponse>(PATH, serde_json::to_value(block_dto)?)
            .await?;

        Ok(response.block_id)
    }

    /// Returns the BlockId of the submitted block.
    /// POST /api/core/v3/blocks
    pub async fn post_block_raw(&self, block: &Block) -> Result<BlockId> {
        const PATH: &str = "api/core/v3/blocks";

        let response = self
            .post_request_bytes::<SubmitBlockResponse>(PATH, &block.pack_to_vec())
            .await?;

        Ok(response.block_id)
    }

    /// Finds a block by its ID and returns it as object.
    /// GET /api/core/v3/blocks/{blockId}
    pub async fn get_block(&self, block_id: &BlockId) -> Result<Block> {
        let path = &format!("api/core/v3/blocks/{block_id}");

        let dto = self.get_request::<BlockDto>(path, None, false, true).await?;

        Ok(Block::try_from_dto_with_params(
            dto,
            &self.get_protocol_parameters().await?,
        )?)
    }

    /// Finds a block by its ID and returns it as raw bytes.
    /// GET /api/core/v3/blocks/{blockId}
    pub async fn get_block_raw(&self, block_id: &BlockId) -> Result<Vec<u8>> {
        let path = &format!("api/core/v3/blocks/{block_id}");

        self.get_request_bytes(path, None).await
    }

    /// Returns the metadata of a block.
    /// GET /api/core/v3/blocks/{blockId}/metadata
    pub async fn get_block_metadata(&self, block_id: &BlockId) -> Result<BlockMetadataResponse> {
        let path = &format!("api/core/v3/blocks/{block_id}/metadata");

        self.get_request(path, None, true, true).await
    }

    /// Returns a block with its metadata.
    /// GET /api/core/v3/blocks/{blockId}/full
    pub async fn get_block_with_metadata(&self, block_id: &BlockId) -> Result<BlockWithMetadataResponse> {
        let path = &format!("api/core/v3/blocks/{block_id}/full");

        self.get_request(path, None, true, true).await
    }

    // UTXO routes.

    /// Finds an output by its ID and returns it as object.
    /// GET /api/core/v3/outputs/{outputId}
    pub async fn get_output(&self, output_id: &OutputId) -> Result<OutputResponse> {
        let path = &format!("api/core/v3/outputs/{output_id}");

        self.get_request(path, None, false, true).await
    }

    /// Finds an output by its ID and returns it as raw bytes.
    /// GET /api/core/v3/outputs/{outputId}
    pub async fn get_output_raw(&self, output_id: &OutputId) -> Result<Vec<u8>> {
        let path = &format!("api/core/v3/outputs/{output_id}");

        self.get_request_bytes(path, None).await
    }

    /// Finds output metadata by output ID.
    /// GET /api/core/v3/outputs/{outputId}/metadata
    pub async fn get_output_metadata(&self, output_id: &OutputId) -> Result<OutputMetadata> {
        let path = &format!("api/core/v3/outputs/{output_id}/metadata");

        self.get_request(path, None, false, true).await
    }

    /// Finds an output with its metadata by output ID.
    /// GET /api/core/v3/outputs/{outputId}/full
    pub async fn get_output_with_metadata(&self, output_id: &OutputId) -> Result<OutputWithMetadata> {
        let path = &format!("api/core/v3/outputs/{output_id}/full");

        self.get_request(path, None, false, true).await
    }

    /// Returns the earliest confirmed block containing the transaction with the given ID.
    /// GET /api/core/v3/transactions/{transactionId}/included-block
    pub async fn get_included_block(&self, transaction_id: &TransactionId) -> Result<Block> {
        let path = &format!("api/core/v3/transactions/{transaction_id}/included-block");

        let dto = self.get_request::<BlockDto>(path, None, true, true).await?;

        Ok(Block::try_from_dto_with_params(
            dto,
            &self.get_protocol_parameters().await?,
        )?)
    }

    /// Returns the earliest confirmed block containing the transaction with the given ID, as raw bytes.
    /// GET /api/core/v3/transactions/{transactionId}/included-block
    pub async fn get_included_block_raw(&self, transaction_id: &TransactionId) -> Result<Vec<u8>> {
        let path = &format!("api/core/v3/transactions/{transaction_id}/included-block");

        self.get_request_bytes(path, None).await
    }

    /// Returns the metadata of the earliest block containing the tx that was confirmed.
    /// GET /api/core/v3/transactions/{transactionId}/included-block/metadata
    pub async fn get_included_block_metadata(&self, transaction_id: &TransactionId) -> Result<BlockMetadataResponse> {
        let path = &format!("api/core/v3/transactions/{transaction_id}/included-block/metadata");

        self.get_request(path, None, true, true).await
    }

    /// Finds the metadata of a transaction.
    /// GET /api/core/v3/transactions/{transactionId}/metadata
    pub async fn get_transaction_metadata(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<TransactionMetadataResponse> {
        let path = &format!("api/core/v3/transactions/{transaction_id}/metadata");

        self.get_request(path, None, true, true).await
    }

    // Commitments routes.

    /// Finds a slot commitment by its ID and returns it as object.
    /// GET /api/core/v3/commitments/{commitmentId}
    pub async fn get_slot_commitment_by_id(&self, slot_commitment_id: &SlotCommitmentId) -> Result<SlotCommitment> {
        let path = &format!("api/core/v3/commitments/{slot_commitment_id}");

        self.get_request(path, None, false, true).await
    }

    /// Finds a slot commitment by its ID and returns it as raw bytes.
    /// GET /api/core/v3/commitments/{commitmentId}
    pub async fn get_slot_commitment_by_id_raw(&self, slot_commitment_id: &SlotCommitmentId) -> Result<Vec<u8>> {
        let path = &format!("api/core/v3/commitments/{slot_commitment_id}");

        self.get_request_bytes(path, None).await
    }

    /// Get all UTXO changes of a given slot by slot commitment ID.
    /// GET /api/core/v3/commitments/{commitmentId}/utxo-changes
    pub async fn get_utxo_changes_by_slot_commitment_id(
        &self,
        slot_commitment_id: &SlotCommitmentId,
    ) -> Result<UtxoChangesResponse> {
        let path = &format!("api/core/v3/commitments/{slot_commitment_id}/utxo-changes");

        self.get_request(path, None, false, false).await
    }

    /// Finds a slot commitment by slot index and returns it as object.
    /// GET /api/core/v3/commitments/by-slot/{slot}
    pub async fn get_slot_commitment_by_slot(&self, slot_index: SlotIndex) -> Result<SlotCommitment> {
        let path = &format!("api/core/v3/commitments/by-slot/{slot_index}");

        self.get_request(path, None, false, true).await
    }

    /// Finds a slot commitment by slot index and returns it as raw bytes.
    /// GET /api/core/v3/commitments/by-slot/{slot}
    pub async fn get_slot_commitment_by_slot_raw(&self, slot_index: SlotIndex) -> Result<Vec<u8>> {
        let path = &format!("api/core/v3/commitments/by-slot/{slot_index}");

        self.get_request_bytes(path, None).await
    }

    /// Get all UTXO changes of a given slot by its index.
    /// GET /api/core/v3/commitments/by-slot/{slot}/utxo-changes
    pub async fn get_utxo_changes_by_slot(&self, slot_index: SlotIndex) -> Result<UtxoChangesResponse> {
        let path = &format!("api/core/v3/commitments/by-slot/{slot_index}/utxo-changes");

        self.get_request(path, None, false, false).await
    }
}

impl Client {
    /// GET /api/core/v3/info endpoint
    pub async fn get_node_info(url: &str, auth: Option<NodeAuth>) -> Result<InfoResponse> {
        let mut url = crate::client::node_manager::builder::validate_url(Url::parse(url)?)?;
        if let Some(auth) = &auth {
            if let Some((name, password)) = &auth.basic_auth_name_pwd {
                url.set_username(name)
                    .map_err(|_| crate::client::Error::UrlAuth("username"))?;
                url.set_password(Some(password))
                    .map_err(|_| crate::client::Error::UrlAuth("password"))?;
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
                    Node {
                        url,
                        auth,
                        disabled: false,
                    },
                    DEFAULT_API_TIMEOUT,
                )
                .await?
                .into_json()
                .await?;

        Ok(resp)
    }
}
