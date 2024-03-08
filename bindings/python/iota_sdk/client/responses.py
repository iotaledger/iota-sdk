# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from typing import Dict, List, Optional
from enum import Enum
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.block.block import Block
from iota_sdk.types.block.id import BlockId
from iota_sdk.types.common import HexStr, json, EpochIndex, SlotIndex
from iota_sdk.types.node_info import BaseTokenResponse, StatusResponse, ProtocolParameters
from iota_sdk.types.output import Output, deserialize_output
from iota_sdk.types.output_id import OutputId, OutputWithId
from iota_sdk.types.output_id_proof import OutputIdProof
from iota_sdk.types.output_metadata import OutputMetadata
from iota_sdk.types.slot import SlotCommitment, SlotCommitmentId
from iota_sdk.types.transaction_id import TransactionId
from iota_sdk.types.transaction_metadata import TransactionFailureReason, TransactionState


# Node routes responses

@json
@dataclass
class RoutesResponse:
    """API route groups of the node.
    GET /api/routes.

    Attributes:
        routes: The available API route groups of the node.
    """
    routes: List[str]


@json
@dataclass
class ProtocolParametersResponse:
    """Protocol Parameters with start epoch.

    Attributes:
        parameters: The protocol parameters.
        start_epoch: The start epoch of the set of protocol parameters.
    """
    parameters: ProtocolParameters
    start_epoch: EpochIndex


@json
@dataclass
class InfoResponse:
    """General information about the node.
    GET /api/core/v3/info.

    Attributes:
        name: The name of the node (e.g. Hornet).
        version: The semantic version of the node.
        status: The status of the node.
        protocol_parameters: Supported protocol versions by the node.
        base_token: Gives info about the base token the network uses.
    """
    name: str
    version: str
    status: StatusResponse
    protocol_parameters: List[ProtocolParametersResponse]
    base_token: BaseTokenResponse


@json
@dataclass
class NodeInfoResponse:
    """General information about the node and its URL.
    GET /api/core/v3/info.

    Attributes:
        info: A NodeInfo object.
        url: The URL of the node.
    """
    info: InfoResponse
    url: str


@json
@dataclass
class NetworkMetricsResponse:
    """Network metrics.

    Attributes:
        blocks_per_second: The current rate of new blocks per second.
        confirmed_blocks_per_second: The current rate of confirmed blocks per second.
        confirmation_rate: The ratio of confirmed blocks to new blocks of the last confirmed slot.
    """
    blocks_per_second: float = field(metadata=config(
        encoder=str
    ))
    confirmed_blocks_per_second: float = field(metadata=config(
        encoder=str
    ))
    confirmation_rate: float = field(metadata=config(
        encoder=str
    ))

# Accounts routes responses


@json
@dataclass
class CongestionResponse:
    """Provides the cost and readiness to issue estimates.
    Response of GET /api/core/v3/accounts/{accountId}/congestion.

    Attributes:
        slot: Slot for which the estimate is provided.
        ready: Indicates if a node is ready to schedule a block issued by the specified account, or if the issuer should wait.
        reference_mana_cost: Mana cost a user needs to burn to issue a block in the slot.
        block_issuance_credits: BIC of the account in the slot. This balance needs to be non-negative, otherwise account is locked.
    """
    slot: SlotIndex
    ready: bool
    reference_mana_cost: int = field(metadata=config(
        encoder=str
    ))
    block_issuance_credits: int = field(metadata=config(
        encoder=str
    ))


# Rewards routes responses

@json
@dataclass
class ManaRewardsResponse:
    """The mana rewards of an account or delegation output.
    Response of GET /api/core/v3/rewards/{outputId}.

    Attributes:
        start_epoch: First epoch for which rewards can be claimed. This value is useful for checking if rewards have expired (by comparing against the staking or delegation start) or would expire soon (by checking its relation to the rewards retention period).
        end_epoch: Last epoch for which rewards can be claimed.
        rewards: Amount of totally available decayed rewards the requested output may claim.
        latest_committed_epoch_pool_rewards: Rewards of the latest committed epoch of the staking pool to which this validator or delegator belongs. The ratio of this value and the maximally possible rewards for the latest committed epoch can be used to determine how well the validator of this staking pool performed in that epoch. Note that if the pool was not part of the committee in the latest committed epoch, this value is 0.
    """
    start_epoch: EpochIndex
    end_epoch: EpochIndex
    rewards: int = field(metadata=config(
        encoder=str
    ))
    latest_committed_epoch_pool_rewards: int = field(metadata=config(
        encoder=str
    ))


# Validators routes responses

@json
@dataclass
class ValidatorResponse:
    """Information of a validator.
    Response of GET /api/core/v3/validators/{bech32Address}

    Attributes:
        address: Account address of the validator.
        staking_end_epoch: The epoch index until which the validator registered to stake.
        pool_stake: The total stake of the pool, including delegators.
        validator_stake: The stake of a validator.
        fixed_cost: The fixed cost of the validator, which it receives as part of its Mana rewards.
        active: Shows whether the validator was active recently.
        latest_supported_protocol_version: The latest protocol version the validator supported.
        latest_supported_protocol_hash: The protocol hash of the latest supported protocol of the validator.
    """
    address: str
    staking_end_epoch: EpochIndex
    pool_stake: int = field(metadata=config(
        encoder=str
    ))
    validator_stake: int = field(metadata=config(
        encoder=str
    ))
    fixed_cost: int = field(metadata=config(
        encoder=str
    ))
    active: bool
    latest_supported_protocol_version: int
    latest_supported_protocol_hash: HexStr


@json
@dataclass
class ValidatorsResponse:
    """A paginated list of all registered validators ready for the next epoch and indicates if they were active recently
    (are eligible for committee selection).
    Response of GET /api/core/v3/validators

    Attributes:
        validators: List of registered validators ready for the next epoch.
        page_size: The number of validators returned per one API request with pagination.
        cursor: The cursor that needs to be provided as cursor query parameter to request the next page. If empty, this was the last page.
    """
    validators: List[ValidatorResponse]
    page_size: int
    cursor: Optional[str] = None


# Committee routes responses

@json
@dataclass
class CommitteeMember:
    """Information of a committee member.

    Attributes:
        address: Account address of the validator.
        pool_stake: The total stake of the pool, including delegators.
        validator_stake: The stake of a validator.
        fixed_cost: The fixed cost of the validator, which it receives as part of its Mana rewards.
    """
    address: str
    pool_stake: int = field(metadata=config(
        encoder=str
    ))
    validator_stake: int = field(metadata=config(
        encoder=str
    ))
    fixed_cost: int = field(metadata=config(
        encoder=str
    ))


@json
@dataclass
class CommitteeResponse:
    """The validator information of the committee.
    Response of GET /api/core/v3/committee

    Attributes:
        epoch: The epoch index of the committee.
        total_stake: The total amount of delegated and staked IOTA coins in the selected committee.
        total_validator_stake: The total amount of staked IOTA coins in the selected committee.
        committee: The validators of the committee.
    """
    epoch: EpochIndex
    total_stake: int = field(metadata=config(
        encoder=str
    ))
    total_validator_stake: int = field(metadata=config(
        encoder=str
    ))
    committee: List[CommitteeMember]


# Blocks routes responses

@json
@dataclass
class IssuanceBlockHeaderResponse:
    """Information that is used to attach a block in the network.
    Response of GET /api/core/v3/blocks/issuance

    Attributes:
        strong_parents: Blocks that are strongly directly approved.
        latest_parent_block_issuing_time: Latest issuing time of the returned parents.
        latest_finalized_slot: The slot index of the latest finalized slot.
        latest_commitment: The latest slot commitment.
        weak_parents: Blocks that are weakly directly approved.
        shallow_like_parents: Blocks that are directly referenced to adjust opinion.
    """
    strong_parents: List[BlockId]
    latest_parent_block_issuing_time: int = field(metadata=config(
        encoder=str
    ))
    latest_finalized_slot: SlotIndex
    latest_commitment: SlotCommitment
    weak_parents: Optional[List[BlockId]] = None
    shallow_like_parents: Optional[List[BlockId]] = None


class BlockState(str, Enum):
    """Describes the state of a block.

    Attributes:
        Pending:    The block has been booked by the node but not yet accepted.
        Accepted:   The block has been referenced by the super majority of the online committee.
        Confirmed:  The block has been referenced by the super majority of the total committee.
        Finalized:  The commitment containing the block has been finalized.
                    This state is computed based on the accepted/confirmed block's slot being smaller or equal than the latest finalized slot.
        Dropped:    The block has been dropped due to congestion control.
        Orphaned:   The block's slot has been committed by the node without the block being included.
                    In this case, the block will never be finalized unless there is a chain switch.
                    This state is computed based on the pending block's slot being smaller or equal than the latest committed slot.
    """
    Pending = 'pending'
    Accepted = 'accepted'
    Confirmed = 'confirmed'
    Finalized = 'finalized'
    Dropped = 'dropped'
    Orphaned = 'orphaned'


@json
@dataclass
class BlockMetadataResponse:
    """The metadata of a block.
    Response of GET /api/core/v3/blocks/{blockId}/metadata.

    Attributes:
        block_id: The identifier of the block. Hex-encoded with 0x prefix.
        block_state: If pending, the block is stored but not confirmed. If confirmed, the block is confirmed with the first level of knowledge. If finalized, the block is included and cannot be reverted anymore. If rejected, the block is rejected by the node, and user should reissue payload if it contains one. If failed, the block is not successfully issued due to failure reason.
    """
    block_id: BlockId
    block_state: BlockState


@json
@dataclass
class BlockWithMetadataResponse:
    """A block with its metadata.
    Response of GET /api/core/v3/blocks/{blockId}/full.

    Attributes:
        block: The block.
        metadata: The block metadata.
    """
    block: Block
    metadata: BlockMetadataResponse


# UTXO routes responses

@json
@dataclass
class OutputIdsResponse:
    """Response type for output IDs.

    Attributes:
        committed_slot: The committed slot at which these outputs were available at.
        page_size: The maximum amount of items returned in one call. If there are more items, a cursor to the next page is returned too.
        cursor: The cursor to the next page of results.
        items: The query results.
    """

    def __init__(self, output_dict: Dict):
        self.committed_slot = output_dict["committedSlot"]
        self.page_size = output_dict["pageSize"]
        self.cursor = output_dict["cursor"]
        self.items = [OutputId(
            output_id) for output_id in output_dict["items"]]


@json
@dataclass
class OutputResponse:
    """An output with its output id proof.
    Response of GET /api/core/v3/outputs/{outputId}.

    Attributes:
        output: One of the possible output types.
        output_id_proof: The proof of the output identifier.
    """
    output: Output = field(metadata=config(
        decoder=deserialize_output
    ))
    output_id_proof: OutputIdProof


@json
@dataclass
class OutputWithMetadataResponse:
    """An output with its output id proof and its metadata.
    Response of GET /api/core/v3/outputs/{outputId}/full.

    Attributes:
        output: One of the possible output types.
        output_id_proof: The associated Output ID proof.
        metadata: The metadata of the output.
    """
    output: Output = field(metadata=config(
        decoder=deserialize_output
    ))
    output_id_proof: OutputIdProof
    metadata: OutputMetadata


@json
@dataclass
class TransactionMetadataResponse:
    """Response of a GET transaction metadata REST API call.

    Attributes:
        transaction_id: The identifier of the transaction. Hex-encoded with 0x prefix.
        transaction_state: If 'pending', the transaction is not included yet. If 'accepted', the transaction is included. If 'confirmed' means transaction is included and its included block is confirmed. If 'finalized' means transaction is included, its included block is finalized and cannot be reverted anymore. If 'failed' means transaction is issued but failed due to the transaction failure reason.
        earliest_attachment_slot: The slot of the earliest included valid block that contains an attachment of the transaction.
        transaction_failure_reason: If applicable, indicates the error that occurred during the transaction processing.
        transaction_failure_details: Contains the detailed error message that occurred during the transaction processing if the debug mode was activated in the retainer.
    """
    transaction_id: TransactionId
    transaction_state: TransactionState
    earliest_attachment_slot: SlotIndex
    transaction_failure_reason: Optional[TransactionFailureReason] = None
    transaction_failure_details: Optional[str] = None


# Commitment routes responses

@json
@dataclass
class UtxoChangesResponse:
    """All UTXO changes that happened at a specific slot.
    Response of
    - GET /api/core/v3/commitments/{commitmentId}/utxo-changes
    - GET /api/core/v3/commitments/by-slot/{slot}/utxo-changes

    Arguments:
        commitment_id: The commitment ID of the requested slot that contains the changes. Hex-encoded with 0x prefix.
        created_outputs: The created outputs of the given slot.
        consumed_outputs: The consumed outputs of the given slot.
    """
    commitment_id: SlotCommitmentId
    created_outputs: List[OutputId]
    consumed_outputs: List[OutputId]


@json
@dataclass
class UtxoChangesFullResponse:
    """All full UTXO changes that happened at a specific slot.
    Response of
    - GET /api/core/v3/commitments/{commitmentId}/utxo-changes/full
    - GET /api/core/v3/commitments/by-slot/{slot}/utxo-changes/full

    Arguments:
        commitment_id: The commitment ID of the requested slot that contains the changes. Hex-encoded with 0x prefix.
        created_outputs: The created outputs of the given slot.
        consumed_outputs: The consumed outputs of the given slot.
    """
    commitment_id: SlotCommitmentId
    created_outputs: List[OutputWithId]
    consumed_outputs: List[OutputWithId]
