// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{string::String, vec::Vec};

use crate::types::block::{
    output::{dto::OutputDto, OutputId, OutputMetadata, OutputWithMetadata},
    payload::milestone::{option::dto::ReceiptMilestoneOptionDto, MilestoneId},
    protocol::ProtocolParameters,
    BlockId,
};

/// Response of GET /api/core/v2/info.
/// Returns general information about the node.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct InfoResponse {
    pub name: String,
    pub version: String,
    pub status: StatusResponse,
    pub supported_protocol_versions: Vec<u8>,
    pub protocol: ProtocolParameters,
    pub pending_protocol_parameters: Vec<PendingProtocolParameter>,
    pub base_token: BaseTokenResponse,
    pub metrics: MetricsResponse,
    pub features: Vec<String>,
}

#[cfg(feature = "serde")]
impl core::fmt::Display for InfoResponse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

/// Returned in [`InfoResponse`].
/// Status information about the node.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct StatusResponse {
    pub is_healthy: bool,
    pub latest_milestone: LatestMilestoneResponse,
    pub confirmed_milestone: ConfirmedMilestoneResponse,
    pub pruning_index: u32,
}

/// Returned in [`StatusResponse`].
/// Information about the latest milestone.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct LatestMilestoneResponse {
    pub index: u32,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub timestamp: Option<u32>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub milestone_id: Option<MilestoneId>,
}

/// Returned in [`StatusResponse`].
/// Information about the confirmed milestone.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ConfirmedMilestoneResponse {
    pub index: u32,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub timestamp: Option<u32>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub milestone_id: Option<MilestoneId>,
}

/// Returned in [`InfoResponse`].
/// Pending protocol parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct PendingProtocolParameter {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub kind: u8,
    pub target_milestone_index: u32,
    pub protocol_version: u8,
    pub params: String,
}

/// Returned in [`InfoResponse`].
/// Information about the base token.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct BaseTokenResponse {
    pub name: String,
    pub ticker_symbol: String,
    pub unit: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub subunit: Option<String>,
    pub decimals: u8,
    pub use_metric_prefix: bool,
}

/// Returned in [`InfoResponse`].
/// Metric information about the node.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct MetricsResponse {
    pub blocks_per_second: f64,
    pub referenced_blocks_per_second: f64,
    pub referenced_rate: f64,
}

/// Response of GET /api/core/v2/tips.
/// Returns non-lazy tips.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct TipsResponse {
    pub tips: Vec<BlockId>,
}

/// Response of POST /api/core/v2/blocks.
/// Returns the block identifier of the submitted block.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct SubmitBlockResponse {
    pub block_id: BlockId,
}

/// Describes the ledger inclusion state of a transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum LedgerInclusionState {
    Conflicting,
    Included,
    NoTransaction,
}

/// Response of GET /api/core/v2/blocks/{block_id}/metadata.
/// Returns the metadata of a block.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct BlockMetadataResponse {
    pub block_id: BlockId,
    pub parents: Vec<BlockId>,
    pub is_solid: bool,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub referenced_by_milestone_index: Option<u32>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub milestone_index: Option<u32>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ledger_inclusion_state: Option<LedgerInclusionState>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub conflict_reason: Option<u8>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub white_flag_index: Option<u32>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub should_promote: Option<bool>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub should_reattach: Option<bool>,
}

/// Response of GET /api/core/v2/outputs/{output_id}.
/// Returns an output and its metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct OutputWithMetadataResponse {
    pub metadata: OutputMetadata,
    pub output: OutputDto,
}

impl From<&OutputWithMetadata> for OutputWithMetadataResponse {
    fn from(value: &OutputWithMetadata) -> Self {
        Self {
            metadata: value.metadata,
            output: OutputDto::from(value.output()),
        }
    }
}

impl From<OutputWithMetadata> for OutputWithMetadataResponse {
    fn from(value: OutputWithMetadata) -> Self {
        Self::from(&value)
    }
}

/// Describes a receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ReceiptResponse {
    pub receipt: ReceiptMilestoneOptionDto,
    pub milestone_index: u32,
}

/// Response of:
/// * GET /api/core/v2/receipts/{milestone_index}, returns all stored receipts for the given milestone index.
/// * GET /api/core/v2/receipts, returns all stored receipts, independent of a milestone index.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReceiptsResponse {
    pub receipts: Vec<ReceiptResponse>,
}

/// Response of GET /api/core/v2/treasury.
/// Returns all information about the treasury.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct TreasuryResponse {
    pub milestone_id: MilestoneId,
    pub amount: String,
}

/// Response of GET /api/core/v2/milestone/{milestone_index}/utxo-changes.
/// Returns all UTXO changes that happened at a specific milestone.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct UtxoChangesResponse {
    pub index: u32,
    pub created_outputs: Vec<OutputId>,
    pub consumed_outputs: Vec<OutputId>,
}

/// Describes the heartbeat of a node.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Heartbeat {
    pub solid_milestone_index: u32,
    pub pruned_milestone_index: u32,
    pub latest_milestone_index: u32,
    pub connected_peers: u8,
    pub synced_peers: u8,
}

/// Describes metrics of a gossip stream.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Metrics {
    pub new_blocks: u64,
    pub received_blocks: u64,
    pub known_blocks: u64,
    pub received_block_requests: u64,
    pub received_milestone_requests: u64,
    pub received_heartbeats: u64,
    pub sent_blocks: u64,
    pub sent_block_requests: u64,
    pub sent_milestone_requests: u64,
    pub sent_heartbeats: u64,
    pub dropped_packets: u64,
}

/// Returns all information about the gossip stream with the peer.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gossip {
    pub heartbeat: Heartbeat,
    pub metrics: Metrics,
}

/// Describes the relation with the peer.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum Relation {
    Known,
    Unknown,
    Autopeered,
}

/// Response of
/// - GET /api/core/v2/peer/{peer_id}
/// - POST /api/core/v2/peers
/// Returns information about a peer.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct PeerResponse {
    pub id: String,
    pub multi_addresses: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub alias: Option<String>,
    pub relation: Relation,
    pub connected: bool,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub gossip: Option<Gossip>,
}

/// Response of GET /api/plugins/debug/whiteflag.
/// Returns the computed merkle tree hash for the given white flag traversal.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct WhiteFlagResponse {
    pub merkle_tree_hash: String,
}

/// Response of GET /api/routes.
/// Returns the available API route groups of the node.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct RoutesResponse {
    pub routes: Vec<String>,
}
