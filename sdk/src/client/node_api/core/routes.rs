// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Node core API routes.

use packable::PackableExt;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    client::{
        constants::{DEFAULT_API_TIMEOUT, DEFAULT_USER_AGENT},
        node_manager::node::{Node, NodeAuth},
        Client, ClientInner, Result,
    },
    types::{
        api::core::response::{
            BlockMetadataResponse, CongestionResponse, InfoResponse, IssuanceBlockHeaderResponse, PeerResponse,
            RoutesResponse, SubmitBlockResponse, UtxoChangesResponse,
        },
        block::{
            output::{dto::OutputDto, AccountId, Output, OutputId, OutputMetadata},
            payload::transaction::TransactionId,
            slot::{SlotCommitment, SlotCommitmentId, SlotIndex},
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

        self.node_manager
            .read()
            .await
            .get_request(PATH, None, self.get_timeout().await, false, false)
            .await
    }

    /// Returns general information about the node.
    /// GET /api/core/v3/info
    pub async fn get_info(&self) -> Result<NodeInfoWrapper> {
        self.node_manager
            .read()
            .await
            .get_request(INFO_PATH, None, self.get_timeout().await, false, false)
            .await
    }

    /// Checks if the account is ready to issue a block.
    /// GET /api/core/v3/accounts/{accountId}/congestion
    pub async fn get_account_congestion(&self, account_id: &AccountId) -> Result<CongestionResponse> {
        let path = &format!("api/core/v3/accounts/{account_id}/congestion");

        self.node_manager
            .read()
            .await
            .get_request(path, None, self.get_timeout().await, false, false)
            .await
    }

    // Blocks routes.

    /// Returns information that is ideal for attaching a block in the network.
    /// GET /api/core/v3/blocks/issuance
    pub async fn get_issuance(&self) -> Result<IssuanceBlockHeaderResponse> {
        const PATH: &str = "api/core/v3/blocks/issuance";

        self.node_manager
            .read()
            .await
            .get_request(PATH, None, self.get_timeout().await, false, false)
            .await
    }

    /// Returns the BlockId of the submitted block.
    /// POST JSON to /api/core/v3/blocks
    pub async fn post_block(&self, block: &Block) -> Result<BlockId> {
        const PATH: &str = "api/core/v3/blocks";
        let timeout = self.get_timeout().await;
        let block_dto = BlockDto::from(block);

        let response = self
            .node_manager
            .read()
            .await
            .post_request_json::<SubmitBlockResponse>(PATH, timeout, serde_json::to_value(block_dto)?)
            .await?;

        Ok(response.block_id)
    }

    /// Returns the BlockId of the submitted block.
    /// POST /api/core/v3/blocks
    pub async fn post_block_raw(&self, block: &Block) -> Result<BlockId> {
        const PATH: &str = "api/core/v3/blocks";
        let timeout = self.get_timeout().await;

        let response = self
            .node_manager
            .read()
            .await
            .post_request_bytes::<SubmitBlockResponse>(PATH, timeout, &block.pack_to_vec())
            .await?;

        Ok(response.block_id)
    }

    /// Finds a block by its ID and returns it as object.
    /// GET /api/core/v3/blocks/{blockId}
    pub async fn get_block(&self, block_id: &BlockId) -> Result<Block> {
        let path = &format!("api/core/v3/blocks/{block_id}");

        let dto = self
            .node_manager
            .read()
            .await
            .get_request::<BlockDto>(path, None, self.get_timeout().await, false, true)
            .await?;

        Ok(Block::try_from_dto(dto, self.get_protocol_parameters().await?)?)
    }

    /// Finds a block by its ID and returns it as raw bytes.
    /// GET /api/core/v3/blocks/{blockId}
    pub async fn get_block_raw(&self, block_id: &BlockId) -> Result<Vec<u8>> {
        let path = &format!("api/core/v3/blocks/{block_id}");

        self.node_manager
            .read()
            .await
            .get_request_bytes(path, None, self.get_timeout().await)
            .await
    }

    /// Returns the metadata of a block.
    /// GET /api/core/v3/blocks/{blockId}/metadata
    pub async fn get_block_metadata(&self, block_id: &BlockId) -> Result<BlockMetadataResponse> {
        let path = &format!("api/core/v3/blocks/{block_id}/metadata");

        self.node_manager
            .read()
            .await
            .get_request(path, None, self.get_timeout().await, true, true)
            .await
    }

    // UTXO routes.

    /// Finds an output by its ID and returns it as object.
    /// GET /api/core/v3/outputs/{outputId}
    pub async fn get_output(&self, output_id: &OutputId) -> Result<Output> {
        let path = &format!("api/core/v3/outputs/{output_id}");

        let output = self
            .node_manager
            .read()
            .await
            .get_request::<OutputDto>(path, None, self.get_timeout().await, false, true)
            .await?;
        let token_supply = self.get_token_supply().await?;

        Ok(Output::try_from_dto_with_params(output, token_supply)?)
    }

    /// Finds an output by its ID and returns it as raw bytes.
    /// GET /api/core/v3/outputs/{outputId}
    pub async fn get_output_raw(&self, output_id: &OutputId) -> Result<Vec<u8>> {
        let path = &format!("api/core/v3/outputs/{output_id}");

        self.node_manager
            .read()
            .await
            .get_request_bytes(path, None, self.get_timeout().await)
            .await
    }

    /// Finds output metadata by output ID.
    /// GET /api/core/v3/outputs/{outputId}/metadata
    pub async fn get_output_metadata(&self, output_id: &OutputId) -> Result<OutputMetadata> {
        let path = &format!("api/core/v3/outputs/{output_id}/metadata");

        self.node_manager
            .read()
            .await
            .get_request::<OutputMetadata>(path, None, self.get_timeout().await, false, true)
            .await
    }

    /// Returns the earliest confirmed block containing the transaction with the given ID.
    /// GET /api/core/v3/transactions/{transactionId}/included-block
    pub async fn get_included_block(&self, transaction_id: &TransactionId) -> Result<Block> {
        let path = &format!("api/core/v3/transactions/{transaction_id}/included-block");

        let dto = self
            .node_manager
            .read()
            .await
            .get_request::<BlockDto>(path, None, self.get_timeout().await, true, true)
            .await?;

        Ok(Block::try_from_dto(dto, self.get_protocol_parameters().await?)?)
    }

    /// Returns the earliest confirmed block containing the transaction with the given ID, as raw bytes.
    /// GET /api/core/v3/transactions/{transactionId}/included-block
    pub async fn get_included_block_raw(&self, transaction_id: &TransactionId) -> Result<Vec<u8>> {
        let path = &format!("api/core/v3/transactions/{transaction_id}/included-block");

        self.node_manager
            .read()
            .await
            .get_request_bytes(path, None, self.get_timeout().await)
            .await
    }

    /// Returns the metadata of the earliest block containing the tx that was confirmed.
    /// GET /api/core/v3/transactions/{transactionId}/included-block/metadata
    pub async fn get_included_block_metadata(&self, transaction_id: &TransactionId) -> Result<BlockMetadataResponse> {
        let path = &format!("api/core/v3/transactions/{transaction_id}/included-block/metadata");

        self.node_manager
            .read()
            .await
            .get_request(path, None, self.get_timeout().await, true, true)
            .await
    }

    // Commitments routes.

    /// Finds a slot commitment by its ID and returns it as object.
    /// GET /api/core/v3/commitments/{commitmentId}
    pub async fn get_slot_commitment_by_id(&self, slot_commitment_id: &SlotCommitmentId) -> Result<SlotCommitment> {
        let path = &format!("api/core/v3/commitments/{slot_commitment_id}");

        self.node_manager
            .read()
            .await
            .get_request::<SlotCommitment>(path, None, self.get_timeout().await, false, true)
            .await
    }

    /// Finds a slot commitment by its ID and returns it as raw bytes.
    /// GET /api/core/v3/commitments/{commitmentId}
    pub async fn get_slot_commitment_by_id_raw(&self, slot_commitment_id: &SlotCommitmentId) -> Result<Vec<u8>> {
        let path = &format!("api/core/v3/commitments/{slot_commitment_id}");

        self.node_manager
            .read()
            .await
            .get_request_bytes(path, None, self.get_timeout().await)
            .await
    }

    /// Get all UTXO changes of a given slot by slot commitment ID.
    /// GET /api/core/v3/commitments/{commitmentId}/utxo-changes
    pub async fn get_utxo_changes_by_slot_commitment_id(
        &self,
        slot_commitment_id: &SlotCommitmentId,
    ) -> Result<UtxoChangesResponse> {
        let path = &format!("api/core/v3/commitments/{slot_commitment_id}/utxo-changes");

        self.node_manager
            .read()
            .await
            .get_request(path, None, self.get_timeout().await, false, false)
            .await
    }

    /// Finds a slot commitment by slot index and returns it as object.
    /// GET /api/core/v3/commitments/by-index/{index}
    pub async fn get_slot_commitment_by_index(&self, slot_index: SlotIndex) -> Result<SlotCommitment> {
        let path = &format!("api/core/v3/commitments/by-index/{slot_index}");

        self.node_manager
            .read()
            .await
            .get_request::<SlotCommitment>(path, None, self.get_timeout().await, false, true)
            .await
    }

    /// Finds a slot commitment by slot index and returns it as raw bytes.
    /// GET /api/core/v3/commitments/by-index/{index}
    pub async fn get_slot_commitment_by_index_raw(&self, slot_index: SlotIndex) -> Result<Vec<u8>> {
        let path = &format!("api/core/v3/commitments/by-index/{slot_index}");

        self.node_manager
            .read()
            .await
            .get_request_bytes(path, None, self.get_timeout().await)
            .await
    }

    /// Get all UTXO changes of a given slot by its index.
    /// GET /api/core/v3/commitments/by-index/{index}/utxo-changes
    pub async fn get_utxo_changes_by_slot_index(&self, slot_index: SlotIndex) -> Result<UtxoChangesResponse> {
        let path = &format!("api/core/v3/commitments/by-index/{slot_index}/utxo-changes");

        self.node_manager
            .read()
            .await
            .get_request(path, None, self.get_timeout().await, false, false)
            .await
    }

    // Peers routes.

    /// GET /api/core/v3/peers
    pub async fn get_peers(&self) -> Result<Vec<PeerResponse>> {
        const PATH: &str = "api/core/v3/peers";

        let resp = self
            .node_manager
            .read()
            .await
            .get_request::<Vec<PeerResponse>>(PATH, None, self.get_timeout().await, false, false)
            .await?;

        Ok(resp)
    }

    // // RoutePeer is the route for getting peers by their peerID.
    // // GET returns the peer
    // // DELETE deletes the peer.
    // RoutePeer = "/peers/:" + restapipkg.ParameterPeerID

    // // RoutePeers is the route for getting all peers of the node.
    // // GET returns a list of all peers.
    // // POST adds a new peer.
    // RoutePeers = "/peers"

    // Control routes.

    // // RouteControlDatabasePrune is the control route to manually prune the database.
    // // POST prunes the database.
    // RouteControlDatabasePrune = "/control/database/prune"

    // // RouteControlSnapshotsCreate is the control route to manually create a snapshot files.
    // // POST creates a snapshot (full, delta or both).
    // RouteControlSnapshotsCreate = "/control/snapshots/create"
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
        url.set_path(INFO_PATH);

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
