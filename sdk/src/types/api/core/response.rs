// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

use crate::types::block::{
    output::{dto::OutputDto, AccountId, OutputId, OutputMetadata, OutputWithMetadata},
    parent::{ShallowLikeParents, StrongParents, WeakParents},
    protocol::ProtocolParameters,
    slot::{EpochIndex, SlotCommitment, SlotCommitmentId, SlotIndex},
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
    pub protocol_parameters: ProtocolParametersResponse,
    pub base_token: BaseTokenResponse,
    pub features: Box<[String]>,
}

impl InfoResponse {
    pub fn latest_protocol_parameters(&self) -> &ProtocolParameters {
        self.protocol_parameters.latest()
    }

    pub fn parameters_by_version(&self, protocol_version: u8) -> Option<&ProtocolParameters> {
        self.protocol_parameters.by_version(protocol_version)
    }

    pub fn parameters_by_epoch(&self, epoch_index: EpochIndex) -> Option<&ProtocolParameters> {
        self.protocol_parameters.by_epoch(epoch_index)
    }
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
    pub latest_commitment_id: SlotCommitmentId,
    pub latest_finalized_slot: SlotIndex,
    pub latest_accepted_block_slot: BlockId,
    pub latest_confirmed_block_slot: BlockId,
    pub pruning_slot: SlotIndex,
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
    #[serde(with = "crate::utils::serde::string")]
    pub blocks_per_second: f64,
    #[serde(with = "crate::utils::serde::string")]
    pub confirmed_blocks_per_second: f64,
    #[serde(with = "crate::utils::serde::string")]
    pub confirmed_rate: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolParametersResponse {
    parameters: Box<[ProtocolParameters]>,
    version_map: BTreeMap<u8, usize>,
    epoch_map: BTreeMap<EpochIndex, usize>,
}

impl ProtocolParametersResponse {
    pub fn iter(&self) -> impl Iterator<Item = (EpochIndex, &ProtocolParameters)> {
        self.epoch_map.iter().map(|(&epoch, &i)| (epoch, &self.parameters[i]))
    }

    pub fn latest(&self) -> &ProtocolParameters {
        &self.parameters[*self.version_map.last_key_value().unwrap().1]
    }

    pub fn by_version(&self, protocol_version: u8) -> Option<&ProtocolParameters> {
        self.version_map.get(&protocol_version).map(|&i| &self.parameters[i])
    }

    pub fn by_epoch(&self, epoch_index: EpochIndex) -> Option<&ProtocolParameters> {
        self.epoch_map.get(&epoch_index).map(|&i| &self.parameters[i])
    }
}

#[cfg(feature = "serde")]
mod serde_protocol_params_response {
    use alloc::borrow::Cow;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::*;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ProtocolParametersResponseDto<'a> {
        parameters: Cow<'a, ProtocolParameters>,
        start_epoch: EpochIndex,
    }

    impl Serialize for ProtocolParametersResponse {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(
                self.iter()
                    .map(|(start_epoch, parameters)| ProtocolParametersResponseDto {
                        parameters: Cow::Borrowed(parameters),
                        start_epoch,
                    }),
            )
        }
    }

    impl<'de> Deserialize<'de> for ProtocolParametersResponse {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let parameters = Vec::<ProtocolParametersResponseDto<'_>>::deserialize(deserializer)?;
            let (mut version_map, mut epoch_map) = (BTreeMap::default(), BTreeMap::default());
            let parameters = parameters
                .into_iter()
                .enumerate()
                .map(|(i, res)| {
                    let (start_epoch, parameters) = (res.start_epoch, res.parameters.into_owned());
                    version_map.insert(parameters.version(), i);
                    epoch_map.insert(start_epoch, i);
                    parameters
                })
                .collect();
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
    pub decimals: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subunit: Option<String>,
    pub use_metric_prefix: bool,
}

/// Response of
/// - GET /api/core/v3/committee
/// The validator information of the committee.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct CommitteeResponse {
    pub epoch_index: EpochIndex,
    #[serde(with = "crate::utils::serde::string")]
    pub total_stake: u64,
    #[serde(with = "crate::utils::serde::string")]
    pub total_validator_stake: u64,
    pub committee: Box<[CommitteeMember]>,
}

/// Validator information.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct CommitteeMember {
    pub account_id: AccountId,
    #[serde(with = "crate::utils::serde::string")]
    pub pool_stake: u64,
    #[serde(with = "crate::utils::serde::string")]
    pub validator_stake: u64,
    #[serde(with = "crate::utils::serde::string")]
    pub fixed_cost: u64,
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

/// Response of GET /api/core/v3/blocks/{block_id}/metadata.
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referenced_by_milestone_index: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone_index: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ledger_inclusion_state: Option<LedgerInclusionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_reason: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub white_flag_index: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub should_promote: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub should_reattach: Option<bool>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub relation: Relation,
    pub connected: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
