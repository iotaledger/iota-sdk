# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from typing import Dict, List, Optional
from enum import Enum, IntEnum
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.block.block import Block
from iota_sdk.types.block.id import BlockId
from iota_sdk.types.common import HexStr, json, EpochIndex, SlotIndex
from iota_sdk.types.node_info import BaseTokenResponse, MetricsResponse, StatusResponse, ProtocolParameters
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
        Pending: Stored but not accepted/confirmed.
        Accepted: Valid block referenced by some validators.
        Confirmed: Valid block referenced by more than 2/3 of the validators.
        Finalized: Accepted/confirmed block and the slot was finalized, can no longer be reverted.
        Rejected: Rejected by the node, and user should reissue payload if it contains one.
        Failed: Not successfully issued due to failure reason.
    """
    Pending = 'pending'
    Accepted = 'accepted'
    Confirmed = 'confirmed'
    Finalized = 'finalized'
    Rejected = 'rejected'
    Failed = 'failed'


class BlockFailureReason(IntEnum):
    """Describes the reason of a block failure.

    Attributes:
        TooOldToIssue (1): The block is too old to issue.
        ParentTooOld (2): One of the block's parents is too old.
        ParentDoesNotExist (3): One of the block's parents does not exist.
        IssuerAccountNotFound (4): The block's issuer account could not be found.
        ManaCostCalculationFailed (5): The mana cost could not be calculated.
        BurnedInsufficientMana (6): The block's issuer account burned insufficient Mana for a block.
        AccountLocked (7): The account is locked.
        AccountExpired (8): The account is expired.
        SignatureInvalid (9): The block's signature is invalid.
        DroppedDueToCongestion (10): The block is dropped due to congestion.
        PayloadInvalid (11): The block payload is invalid.
        Invalid (255): The block is invalid.
    """
    TooOldToIssue = 1
    ParentTooOld = 2
    ParentDoesNotExist = 3
    IssuerAccountNotFound = 4
    ManaCostCalculationFailed = 5
    BurnedInsufficientMana = 6
    AccountLocked = 7
    AccountExpired = 8
    SignatureInvalid = 9
    DroppedDueToCongestion = 10
    PayloadInvalid = 11
    Invalid = 255

    def __str__(self):
        return {
            1: "The block is too old to issue.",
            2: "One of the block's parents is too old.",
            3: "One of the block's parents does not exist.",
            4: "The block's issuer account could not be found.",
            5: "The mana cost could not be calculated.",
            6: "The block's issuer account burned insufficient Mana for a block.",
            7: "The account is locked.",
            8: "The account is expired.",
            9: "The block's signature is invalid.",
            10: "The block is dropped due to congestion.",
            11: "The block payload is invalid.",
            255: "The block is invalid."
        }[self.value]


@json
@dataclass
class BlockMetadataResponse:
    """The metadata of a block.
    Response of GET /api/core/v3/blocks/{blockId}/metadata.

    Attributes:
        block_id: The identifier of the block. Hex-encoded with 0x prefix.
        block_state: If pending, the block is stored but not confirmed. If confirmed, the block is confirmed with the first level of knowledge. If finalized, the block is included and cannot be reverted anymore. If rejected, the block is rejected by the node, and user should reissue payload if it contains one. If failed, the block is not successfully issued due to failure reason.
        block_failure_reason: The optional block failure reason.
        transaction_metadata: The optional metadata of a given transaction.
    """
    block_id: BlockId
    block_state: BlockState
    block_failure_reason: Optional[BlockFailureReason] = None
    transaction_metadata: Optional[TransactionMetadataResponse] = None


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
        transaction_failure_reason: The optional transaction failure reason.
    """
    transaction_id: TransactionId
    transaction_state: TransactionState
    transaction_failure_reason: Optional[TransactionFailureReason] = None


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
