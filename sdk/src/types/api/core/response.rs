// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

use serde::{Deserialize, Serialize};

use crate::{
    types::block::{
        output::{dto::OutputDto, AccountId, OutputId, OutputMetadata, OutputWithMetadata},
        parent::{ShallowLikeParents, StrongParents, WeakParents},
        protocol::ProtocolParameters,
        semantic::TransactionFailureReason,
        slot::{EpochIndex, SlotCommitment, SlotCommitmentId, SlotIndex},
        BlockId,
    },
    utils::serde::{option_string, string},
};

/// Response of GET /api/core/v3/info.
/// Returns general information about the node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    pub name: String,
    pub version: String,
    pub status: StatusResponse,
    pub metrics: MetricsResponse,
    pub protocol_parameters: ProtocolParametersMap,
    pub base_token: BaseTokenResponse,
    pub features: Box<[String]>,
}

impl InfoResponse {
    pub fn latest_protocol_parameters(&self) -> &ProtocolParametersResponse {
        self.protocol_parameters.latest()
    }

    pub fn protocol_parameters_by_version(&self, protocol_version: u8) -> Option<&ProtocolParametersResponse> {
        self.protocol_parameters.by_version(protocol_version)
    }

    pub fn protocol_parameters_by_epoch(&self, epoch_index: EpochIndex) -> Option<&ProtocolParametersResponse> {
        self.protocol_parameters.by_epoch(epoch_index)
    }
}

impl core::fmt::Display for InfoResponse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

/// Returned in [`InfoResponse`].
/// Status information about the node.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    pub is_healthy: bool,
    #[serde(with = "option_string")]
    pub accepted_tangle_time: Option<u64>,
    #[serde(with = "option_string")]
    pub relative_accepted_tangle_time: Option<u64>,
    #[serde(with = "option_string")]
    pub confirmed_tangle_time: Option<u64>,
    #[serde(with = "option_string")]
    pub relative_confirmed_tangle_time: Option<u64>,
    pub latest_commitment_id: SlotCommitmentId,
    pub latest_finalized_slot: SlotIndex,
    pub latest_accepted_block_slot: Option<SlotIndex>,
    pub latest_confirmed_block_slot: Option<SlotIndex>,
    pub pruning_epoch: EpochIndex,
}

/// Returned in [`InfoResponse`].
/// Metric information about the node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricsResponse {
    #[serde(with = "string")]
    pub blocks_per_second: f64,
    #[serde(with = "string")]
    pub confirmed_blocks_per_second: f64,
    #[serde(with = "string")]
    pub confirmation_rate: f64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolParametersResponse {
    pub parameters: ProtocolParameters,
    pub start_epoch: EpochIndex,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolParametersMap {
    parameters: Box<[ProtocolParametersResponse]>,
    version_map: BTreeMap<u8, usize>,
    epoch_map: BTreeMap<EpochIndex, usize>,
}

impl ProtocolParametersMap {
    pub fn iter(&self) -> impl Iterator<Item = &ProtocolParametersResponse> {
        self.parameters.iter()
    }

    pub fn latest(&self) -> &ProtocolParametersResponse {
        &self.parameters[*self.version_map.last_key_value().unwrap().1]
    }

    pub fn by_version(&self, protocol_version: u8) -> Option<&ProtocolParametersResponse> {
        self.version_map.get(&protocol_version).map(|&i| &self.parameters[i])
    }

    pub fn by_epoch(&self, epoch_index: EpochIndex) -> Option<&ProtocolParametersResponse> {
        self.epoch_map
            .range(..=epoch_index)
            .map(|(_, &i)| &self.parameters[i])
            .last()
    }
}

mod serde_protocol_params_response {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::*;

    impl Serialize for ProtocolParametersMap {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(self.iter())
        }
    }

    impl<'de> Deserialize<'de> for ProtocolParametersMap {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let parameters = Box::<[ProtocolParametersResponse]>::deserialize(deserializer)?;
            let (mut version_map, mut epoch_map) = (BTreeMap::default(), BTreeMap::default());
            for (i, res) in parameters.iter().enumerate() {
                version_map.insert(res.parameters.version(), i);
                epoch_map.insert(res.start_epoch, i);
            }
            Ok(Self {
                parameters,
                version_map,
                epoch_map,
            })
        }
    }
}

/// Returned in [`InfoResponse`].
/// Information about the base token.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseTokenResponse {
    pub name: String,
    pub ticker_symbol: String,
    pub unit: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subunit: Option<String>,
    pub decimals: u32,
    pub use_metric_prefix: bool,
}

/// Response of GET /api/core/v3/blocks/validators.
/// A paginated list of all registered validators ready for the next epoch and indicates if they were active recently
/// (are eligible for committee selection).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorsResponse {
    /// List of registered validators ready for the next epoch.
    validators: Vec<ValidatorResponse>,
    ///  The number of validators returned per one API request with pagination.
    page_size: u32,
    /// The cursor that needs to be provided as cursor query parameter to request the next page. If empty, this was the
    /// last page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
}

/// Response of GET /api/core/v3/rewards/{outputId}.
/// Returns the mana rewards of an account or delegation output.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManaRewardsResponse {
    /// The starting epoch index for which the mana rewards are returned.
    pub epoch_start: EpochIndex,
    /// The ending epoch index for which the mana rewards are returned, the decay is applied up to this point
    /// included.
    pub epoch_end: EpochIndex,
    /// The amount of totally available rewards the requested output may claim.
    #[serde(with = "string")]
    pub rewards: u64,
}

/// Response of GET /api/core/v3/committee
/// The validator information of the committee.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitteeResponse {
    /// The epoch index of the committee.
    pub epoch_index: EpochIndex,
    /// The total amount of delegated and staked IOTA tokens in the selected committee.
    #[serde(with = "string")]
    pub total_stake: u64,
    /// The total amount of staked IOTA tokens in the selected committee.
    #[serde(with = "string")]
    pub total_validator_stake: u64,
    /// The validators of the committee.
    pub committee: Box<[CommitteeMember]>,
}

/// Returns information of a committee member.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitteeMember {
    /// The account identifier of the validator
    pub account_id: AccountId,
    /// The total stake of the pool, including delegators.
    #[serde(with = "string")]
    pub pool_stake: u64,
    /// The stake of a validator.
    #[serde(with = "string")]
    pub validator_stake: u64,
    /// The fixed cost of the validator, which it receives as part of its Mana rewards.
    #[serde(with = "string")]
    pub fixed_cost: u64,
}

/// Information of a validator.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorResponse {
    /// The account identifier of the validator
    account_id: AccountId,
    /// The epoch index until which the validator registered to stake.
    staking_end_epoch: EpochIndex,
    /// The total stake of the pool, including delegators.
    #[serde(with = "string")]
    pool_stake: u64,
    /// The stake of a validator.
    #[serde(with = "string")]
    validator_stake: u64,
    /// The fixed cost of the validator, which it receives as part of its Mana rewards.
    #[serde(with = "string")]
    fixed_cost: u64,
    /// Shows whether validator was active recently.
    active: bool,
    /// The latest protocol version the validator supported.
    latest_supported_protocol_version: u8,
}

/// Response of GET /api/core/v3/blocks/issuance
/// Information that is ideal for attaching a block in the network.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// Response of GET /api/core/v3/accounts/{accountId}/congestion.
/// Provides the cost and readiness to issue estimates.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CongestionResponse {
    /// The slot index for which the congestion estimate is provided.
    pub slot_index: SlotIndex,
    /// Indicates if a node is ready to issue a block in a current congestion or should wait.
    pub ready: bool,
    /// The cost in mana for issuing a block in a current congestion estimated based on RMC and slot index.
    #[serde(with = "string")]
    pub reference_mana_cost: u64,
    /// The Block Issuance Credits of the requested account.
    #[serde(with = "string")]
    pub block_issuance_credits: u64,
}

/// Response of POST /api/core/v3/blocks.
/// Returns the block identifier of the submitted block.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitBlockResponse {
    pub block_id: BlockId,
}

/// Describes the state of a block.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[repr(u8)]
pub enum BlockFailureReason {
    /// The block is too old to issue.
    TooOldToIssue = 1,
    /// One of the block's parents is too old.
    ParentTooOld = 2,
    /// One of the block's parents does not exist.
    ParentDoesNotExist = 3,
    /// One of the block's parents is invalid.
    ParentInvalid = 4,
    /// The block is dropped due to congestion.
    DroppedDueToCongestion = 5,
    /// The block is invalid.
    Invalid = 6,
}

/// Response of GET /api/core/v3/blocks/{blockId}/metadata.
/// Returns the metadata of a block.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockMetadataResponse {
    pub block_id: BlockId,
    // TODO: verify if really optional: https://github.com/iotaledger/tips-draft/pull/24/files#r1293426314
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_state: Option<BlockState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_state: Option<TransactionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_failure_reason: Option<BlockFailureReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_failure_reason: Option<TransactionFailureReason>,
}

/// Response of GET /api/core/v3/outputs/{output_id}.
/// Returns an output and its metadata.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {
    pub solid_milestone_index: u32,
    pub pruned_milestone_index: u32,
    pub latest_milestone_index: u32,
    pub connected_peers: u8,
    pub synced_peers: u8,
}

/// Describes metrics of a gossip stream.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct Gossip {
    pub heartbeat: Heartbeat,
    pub metrics: Metrics,
}

/// Describes the relation with the peer.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Relation {
    Known,
    Unknown,
    Autopeered,
}

/// Response of
/// - GET /api/core/v3/peer/{peer_id}
/// - POST /api/core/v3/peers
/// Returns information about a peer.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerResponse {
    pub id: String,
    pub multi_addresses: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub relation: Relation,
    pub connected: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gossip: Option<Gossip>,
}

/// Response of GET /api/routes.
/// Returns the available API route groups of the node.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutesResponse {
    pub routes: Vec<String>,
}

/// Response of
/// - GET /api/core/v3/commitments/{commitmentId}/utxo-changes
/// - GET /api/core/v3/commitments/by-index/{index}/utxo-changes
/// Returns all UTXO changes that happened at a specific slot.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoChangesResponse {
    pub index: u32,
    pub created_outputs: Vec<OutputId>,
    pub consumed_outputs: Vec<OutputId>,
}
