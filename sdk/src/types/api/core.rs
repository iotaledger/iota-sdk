// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    string::String,
    vec::Vec,
};

use serde::{Deserialize, Serialize};

use crate::{
    types::block::{
        address::Bech32Address,
        core::Parents,
        output::{Output, OutputId, OutputIdProof, OutputMetadata},
        payload::signed_transaction::TransactionId,
        protocol::{ProtocolParameters, ProtocolParametersHash},
        semantic::TransactionFailureReason,
        slot::{EpochIndex, SlotCommitment, SlotCommitmentId, SlotIndex},
        BlockDto, BlockError, BlockId,
    },
    utils::serde::{option_string, string},
};

/// Response of GET /api/routes.
/// The available API route groups of the node.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutesResponse {
    pub routes: Vec<String>,
}

/// Response of GET /api/core/v3/info.
/// General information about the node.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    pub name: String,
    pub version: String,
    pub status: StatusResponse,
    pub protocol_parameters: ProtocolParametersMap,
    pub base_token: BaseTokenResponse,
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

/// Response of GET /api/core/v3/info.
/// General information about the node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PermanodeInfoResponse {
    pub name: String,
    pub version: String,
    pub is_healthy: bool,
    pub latest_commitment_id: SlotCommitmentId,
    pub protocol_parameters: ProtocolParametersMap,
    pub base_token: BaseTokenResponse,
}

impl PermanodeInfoResponse {
    pub(crate) fn protocol_parameters_by_version(&self, protocol_version: u8) -> Option<&ProtocolParametersResponse> {
        self.protocol_parameters.by_version(protocol_version)
    }
}

/// Returned in [`InfoResponse`].
/// Status information about the node.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    pub is_healthy: bool,
    pub is_network_healthy: bool,
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

/// Metrics information about the network.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkMetricsResponse {
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
}

/// Information of a validator.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorResponse {
    /// Account address of the validator.
    pub address: Bech32Address,
    /// The epoch index until which the validator registered to stake.
    pub staking_end_epoch: EpochIndex,
    /// The total stake of the pool, including delegators.
    #[serde(with = "string")]
    pub pool_stake: u64,
    /// The stake of a validator.
    #[serde(with = "string")]
    pub validator_stake: u64,
    /// The fixed cost of the validator, which it receives as part of its Mana rewards.
    #[serde(with = "string")]
    pub fixed_cost: u64,
    /// Shows whether the validator was active recently.
    pub active: bool,
    /// The latest protocol version the validator supported.
    pub latest_supported_protocol_version: u8,
    /// The protocol hash of the latest supported protocol of the validator.
    pub latest_supported_protocol_hash: ProtocolParametersHash,
}

/// Response of GET /api/core/v3/blocks/validators.
/// A paginated list of all registered validators ready for the next epoch and indicates if they were active recently
/// (are eligible for committee selection).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorsResponse {
    /// List of registered validators ready for the next epoch.
    pub validators: Vec<ValidatorResponse>,
    /// The number of validators returned per one API request with pagination.
    pub page_size: u32,
    /// The cursor that needs to be provided as cursor query parameter to request the next page. If empty, this was the
    /// last page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Response of GET /api/core/v3/rewards/{outputId}.
/// The mana rewards of an account or delegation output.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManaRewardsResponse {
    /// First epoch for which rewards can be claimed.
    /// This value is useful for checking if rewards have expired (by comparing against the staking or delegation
    /// start) or would expire soon (by checking its relation to the rewards retention period).
    pub start_epoch: EpochIndex,
    /// Last epoch for which rewards can be claimed.
    pub end_epoch: EpochIndex,
    /// Amount of totally available decayed rewards the requested output may claim.
    #[serde(with = "string")]
    pub rewards: u64,
    /// Rewards of the latest committed epoch of the staking pool to which this validator or delegator belongs.
    /// The ratio of this value and the maximally possible rewards for the latest committed epoch can be used to
    /// determine how well the validator of this staking pool performed in that epoch.
    /// Note that if the pool was not part of the committee in the latest committed epoch, this value is 0.
    #[serde(with = "string")]
    pub latest_committed_epoch_pool_rewards: u64,
}

/// Response of GET /api/core/v3/committee
/// The validator information of the committee.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitteeResponse {
    /// The validators of the committee.
    pub committee: Box<[CommitteeMember]>,
    /// The total amount of delegated and staked IOTA coins in the selected committee.
    #[serde(with = "string")]
    pub total_stake: u64,
    /// The total amount of staked IOTA coins in the selected committee.
    #[serde(with = "string")]
    pub total_validator_stake: u64,
    /// The epoch index of the committee.
    pub epoch: EpochIndex,
}

/// Information of a committee member.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitteeMember {
    /// Account address of the validator.
    pub address: Bech32Address,
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

/// Response of GET /api/core/v3/blocks/issuance
/// Information that is ideal for attaching a block in the network.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssuanceBlockHeaderResponse {
    /// Blocks that are strongly directly approved.
    pub strong_parents: BTreeSet<BlockId>,
    /// Blocks that are weakly directly approved.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub weak_parents: BTreeSet<BlockId>,
    /// Blocks that are directly referenced to adjust opinion.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub shallow_like_parents: BTreeSet<BlockId>,
    /// Latest issuing time of the returned parents.
    #[serde(with = "string")]
    pub latest_parent_block_issuing_time: u64,
    /// The slot index of the latest finalized slot.
    pub latest_finalized_slot: SlotIndex,
    /// The latest slot commitment.
    pub latest_commitment: SlotCommitment,
}

impl IssuanceBlockHeaderResponse {
    pub fn strong_parents<const MIN: u8, const MAX: u8>(&self) -> Result<Parents<MIN, MAX>, BlockError> {
        Parents::from_set(self.strong_parents.clone())
    }

    pub fn weak_parents<const MIN: u8, const MAX: u8>(&self) -> Result<Parents<MIN, MAX>, BlockError> {
        Parents::from_set(self.weak_parents.clone())
    }

    pub fn shallow_like_parents<const MIN: u8, const MAX: u8>(&self) -> Result<Parents<MIN, MAX>, BlockError> {
        Parents::from_set(self.shallow_like_parents.clone())
    }
}

/// Response of GET /api/core/v3/accounts/{accountId}/congestion.
/// Provides the cost and readiness to issue estimates.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CongestionResponse {
    /// Slot for which the estimate is provided.
    pub slot: SlotIndex,
    /// Indicates if a node is ready to schedule a block issued by the specified account, or if the issuer should wait.
    pub ready: bool,
    /// Mana cost a user needs to burn to issue a block in the slot.
    #[serde(with = "string")]
    pub reference_mana_cost: u64,
    /// BIC of the account in the slot. This balance needs to be non-negative, otherwise account is locked.
    #[serde(with = "string")]
    pub block_issuance_credits: i128,
}

/// Response of POST /api/core/v3/blocks.
/// Returns the block identifier of the submitted block.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitBlockResponse {
    pub block_id: BlockId,
}

/// Describes the state of a block.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, strum::AsRefStr, strum::EnumString)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum BlockState {
    /// The block has been booked by the node but not yet accepted.
    Pending,
    /// The block has been referenced by the super majority of the online committee.
    Accepted,
    /// The block has been referenced by the super majority of the total committee.
    Confirmed,
    /// The commitment containing the block has been finalized.
    /// This state is computed based on the accepted/confirmed block's slot being smaller or equal than the latest
    /// finalized slot.
    Finalized,
    /// The block has been dropped due to congestion control.
    Dropped,
    /// The block's slot has been committed by the node without the block being included.
    /// In this case, the block will never be finalized unless there is a chain switch.
    /// This state is computed based on the pending block's slot being smaller or equal than the latest committed slot.
    Orphaned,
}

/// Describes the state of a transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, strum::AsRefStr, strum::EnumString)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum TransactionState {
    /// The transaction has been booked by the node but not yet accepted.
    Pending,
    /// The transaction meets the following 4 conditions:
    /// - Signatures of the transaction are valid.
    /// - The transaction has been approved by the super majority of the online committee (potential conflicts are
    ///   resolved by this time).
    /// - The transactions that created the inputs were accepted (monotonicity).
    /// - At least one valid attachment was accepted.
    Accepted,
    /// The slot of the earliest accepted attachment of the transaction was committed.
    Committed,
    /// The transaction is accepted and the slot containing the transaction has been finalized by the node.
    /// This state is computed based on the accepted transaction's earliest included attachment slot being smaller or
    /// equal than the latest finalized slot.
    Finalized,
    /// The transaction has not been executed by the node due to a failure during processing.
    Failed,
}

// Response of a GET transaction metadata REST API call.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMetadataResponse {
    /// The transaction ID.
    pub transaction_id: TransactionId,
    /// The transaction state.
    pub transaction_state: TransactionState,
    /// The slot of the earliest included valid block that contains an attachment of the transaction.
    pub earliest_attachment_slot: SlotIndex,
    /// If applicable, indicates the error that occurred during the transaction processing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_failure_reason: Option<TransactionFailureReason>,
    /// Contains the detailed error message that occurred during the transaction processing if the debug mode was
    /// activated in the retainer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_failure_details: Option<String>,
}

/// Response of GET /api/core/v3/blocks/{blockId}/metadata.
/// The metadata of a block.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockMetadataResponse {
    pub block_id: BlockId,
    pub block_state: BlockState,
}

/// Response of GET /api/core/v3/blocks/{blockId}/full.
/// A block and its metadata.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlockWithMetadataResponse {
    pub block: BlockDto,
    pub metadata: BlockMetadataResponse,
}

/// Response of GET /api/core/v3/outputs/{output_id}.
/// Contains the generic [`Output`] with associated [`OutputIdProof`].
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputResponse {
    pub output: Output,
    pub output_id_proof: OutputIdProof,
}

impl From<&OutputWithMetadataResponse> for OutputResponse {
    fn from(value: &OutputWithMetadataResponse) -> Self {
        Self {
            output: value.output().clone(),
            output_id_proof: value.output_id_proof().clone(),
        }
    }
}

impl From<OutputWithMetadataResponse> for OutputResponse {
    fn from(value: OutputWithMetadataResponse) -> Self {
        Self {
            output: value.output,
            output_id_proof: value.output_id_proof,
        }
    }
}

/// Contains the generic [`Output`] with associated [`OutputIdProof`] and [`OutputMetadata`].
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct OutputWithMetadataResponse {
    pub output: Output,
    pub output_id_proof: OutputIdProof,
    pub metadata: OutputMetadata,
}

impl OutputWithMetadataResponse {
    /// Creates a new [`OutputWithMetadataResponse`].
    pub fn new(output: Output, output_id_proof: OutputIdProof, metadata: OutputMetadata) -> Self {
        Self {
            output,
            output_id_proof,
            metadata,
        }
    }

    /// Returns the [`Output`].
    pub fn output(&self) -> &Output {
        &self.output
    }

    /// Consumes self and returns the [`Output`].
    pub fn into_output(self) -> Output {
        self.output
    }

    /// Returns the [`OutputIdProof`].
    pub fn output_id_proof(&self) -> &OutputIdProof {
        &self.output_id_proof
    }

    /// Consumes self and returns the [`OutputIdProof`].
    pub fn into_output_id_proof(self) -> OutputIdProof {
        self.output_id_proof
    }

    /// Returns the [`OutputMetadata`].
    pub fn metadata(&self) -> &OutputMetadata {
        &self.metadata
    }

    /// Consumes self and returns the [`OutputMetadata`].
    pub fn into_metadata(self) -> OutputMetadata {
        self.metadata
    }
}

/// Response of
/// - GET /api/core/v3/commitments/{commitmentId}/utxo-changes
/// - GET /api/core/v3/commitments/by-slot/{slot}/utxo-changes
/// All UTXO changes that happened at a specific slot.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoChangesResponse {
    pub commitment_id: SlotCommitmentId,
    pub created_outputs: Vec<OutputId>,
    pub consumed_outputs: Vec<OutputId>,
}

/// Response of
/// - GET /api/core/v3/commitments/{commitmentId}/utxo-changes/full
/// - GET /api/core/v3/commitments/by-slot/{slot}/utxo-changes/full
/// All full UTXO changes that happened at a specific slot.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoChangesFullResponse {
    pub commitment_id: SlotCommitmentId,
    pub created_outputs: Vec<OutputWithId>,
    pub consumed_outputs: Vec<OutputWithId>,
}

/// An Output and its ID.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputWithId {
    pub output: Output,
    pub output_id: OutputId,
}
