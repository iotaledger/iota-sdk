// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{string::String, vec::Vec};

use crate::types::block::{
    output::{dto::OutputDto, OutputId, OutputMetadata, OutputWithMetadata},
    parent::{ShallowLikeParents, StrongParents, WeakParents},
    protocol::ProtocolParameters,
    semantic::TransactionFailureReason,
    slot::{SlotCommitment, SlotIndex},
    BlockId,
};

/// Response of GET /api/core/v3/info.
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
    pub metrics: MetricsResponse,
    pub supported_protocol_versions: Vec<u8>,
    pub protocol: ProtocolParameters,
    pub base_token: BaseTokenResponse,
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
    #[serde(with = "crate::utils::serde::string")]
    pub accepted_tangle_time: u64,
    #[serde(with = "crate::utils::serde::string")]
    pub relative_accepted_tangle_time: u64,
    #[serde(with = "crate::utils::serde::string")]
    pub confirmed_tangle_time: u64,
    #[serde(with = "crate::utils::serde::string")]
    pub relative_confirmed_tangle_time: u64,
    pub latest_committed_slot: SlotIndex,
    pub latest_finalized_slot: SlotIndex,
    pub pruning_slot: SlotIndex,
    pub latest_accepted_block_id: BlockId,
    pub latest_confirmed_block_id: BlockId,
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
    pub confirmed_blocks_per_second: f64,
    pub confirmed_rate: f64,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subunit: Option<String>,
    pub decimals: u32,
    pub use_metric_prefix: bool,
}

/// Response of
/// - GET /api/core/v3/blocks/issuance
/// Information that is ideal for attaching a block in the network.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct IssuanceBlockHeaderResponse {
    /// Blocks that are strongly directly approved.
    pub strong_parents: StrongParents,
    /// Blocks that are weakly directly approved.
    pub weak_parents: WeakParents,
    /// Blocks that are directly referenced to adjust opinion.
    pub shallow_like_parents: ShallowLikeParents,
    /// The slot index of the latest finalized slot.
    pub latest_finalized_slot: SlotIndex,
    /// The most recent slot commitment.
    pub commitment: SlotCommitment,
}

/// Response of POST /api/core/v3/blocks.
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

/// Describes the state of a block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum BlockState {
    // Stored but not confirmed.
    Pending,
    // Confirmed with the first level of knowledge.
    Confirmed,
    // Included and can no longer be reverted.
    Finalized,
    // Rejected by the node, and user should reissue payload if it contains one.
    Rejected,
    // Not successfully issued due to failure reason.
    Failed,
}

/// Describes the state of a transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum TransactionState {
    // Stored but not confirmed.
    Pending,
    // Confirmed with the first level of knowledge.
    Confirmed,
    // Included and can no longer be reverted.
    Finalized,
    // The block is not successfully issued due to failure reason.
    Failed,
}

/// Describes the reason of a block failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr),
    serde(rename_all = "camelCase")
)]
#[non_exhaustive]
#[repr(u8)]
pub enum BlockFailureReason {
    /// The block is too old to issue.
    TooOldToIssue = 1,
    /// The block's parents are too old.
    ParentsTooOld = 2,
    /// The block failed at the booker.
    FailedAtBooker = 3,
    /// The block is dropped due to congestion.
    DroppedDueToCongestion = 4,
    /// The block is invalid.
    Invalid = 5,
}

/// Response of GET /api/core/v3/blocks/{blockId}/metadata.
/// Returns the metadata of a block.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct BlockMetadataResponse {
    pub block_id: BlockId,
    // TODO: verify if really optional: https://github.com/iotaledger/tips-draft/pull/24/files#r1293426314
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_state: Option<BlockState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_state: Option<TransactionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_failure_reason: Option<BlockFailureReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_failure_reason: Option<TransactionFailureReason>,
}

/// Response of GET /api/core/v3/outputs/{output_id}.
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
/// - GET /api/core/v3/peer/{peer_id}
/// - POST /api/core/v3/peers
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub relation: Relation,
    pub connected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gossip: Option<Gossip>,
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

/// Response of
/// - GET /api/core/v3/commitments/{commitmentId}/utxo-changes
/// - GET /api/core/v3/commitments/by-index/{index}/utxo-changes
/// Returns all UTXO changes that happened at a specific slot.
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

/// Response of GET /api/core/v3/accounts/{accountId}/congestion.
/// Provides the cost and readiness to issue estimates.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct CongestionResponse {
    /// The slot index for which the congestion estimate is provided.
    pub slot_index: SlotIndex,
    /// Indicates if a node is ready to issue a block in a current congestion or should wait.
    pub ready: bool,
    /// The cost in mana for issuing a block in a current congestion estimated based on RMC and slot index.
    #[serde(with = "crate::utils::serde::string")]
    pub reference_mana_cost: u64,
    /// The Block Issuance Credits of the requested account.
    #[serde(with = "crate::utils::serde::string")]
    pub block_issuance_credits: u64,
}
