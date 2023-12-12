# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import Enum, IntEnum
from dataclasses import dataclass
from typing import Optional
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.block.block import Block


@json
@dataclass
class BlockMetadata:
    """Represents the metadata of a block.
    Response of GET /api/core/v3/blocks/{blockId}/metadata.

    Attributes:
        block_state: The block state.
        transaction_state: The transaction state.
        block_failure_reason: The block failure reason.
        transaction_failure_reason: The transaction failure reason.
    """
    block_id: HexStr
    block_state: BlockState
    transaction_state: Optional[TransactionState] = None
    block_failure_reason: Optional[BlockFailureReason] = None
    transaction_failure_reason: Optional[TransactionFailureReason] = None


class BlockState(Enum):
    """Describes the state of a block.

    Attributes:
        Pending: Stored but not confirmed.
        Confirmed: Confirmed with the first level of knowledge.
        Finalized: Included and can no longer be reverted.
        Rejected: Rejected by the node, and user should reissue payload if it contains one.
        Failed: Not successfully issued due to failure reason.
    """
    Pending = 0
    Confirmed = 1
    Finalized = 2
    Rejected = 3
    Failed = 4


class TransactionState(Enum):
    """Describes the state of a transaction.

    Attributes:
        Pending: Stored but not confirmed.
        Confirmed: Confirmed with the first level of knowledge.
        Finalized: Included and can no longer be reverted.
        Failed: The block is not successfully issued due to failure reason.
    """
    Pending = 0
    Confirmed = 1
    Finalized = 2
    Failed = 3


class BlockFailureReason(IntEnum):
    """Describes the reason of a block failure.

    Attributes:
        TooOldToIssue (1): The block is too old to issue.
        ParentTooOld (2): One of the block's parents is too old.
        ParentDoesNotExist (3): One of the block's parents does not exist.
        ParentInvalid (4): One of the block's parents is invalid.
        IssuerAccountNotFound (5): The block's issuer account could not be found.
        VersionInvalid (6): The block's protocol version is invalid.
        ManaCostCalculationFailed (7): The mana cost could not be calculated.
        BurnedInsufficientMana (8): The block's issuer account burned insufficient Mana for a block.
        AccountInvalid (9): The account is invalid.
        SignatureInvalid (10): The block's signature is invalid.
        DroppedDueToCongestion (11): The block is dropped due to congestion.
        PayloadInvalid (12): The block payload is invalid.
        Invalid (255): The block is invalid.
    """
    TooOldToIssue = 1
    ParentTooOld = 2
    ParentDoesNotExist = 3
    ParentInvalid = 4
    IssuerAccountNotFound = 5
    VersionInvalid = 6
    ManaCostCalculationFailed = 7
    BurnedInsufficientMana = 8
    AccountInvalid = 9
    SignatureInvalid = 10
    DroppedDueToCongestion = 11
    PayloadInvalid = 12
    Invalid = 255


class TransactionFailureReason(Enum):
    """Represents the possible reasons for a conflicting transaction.

    Attributes:
        InputUtxoAlreadySpent: The referenced UTXO was already spent.
        ConflictingWithAnotherTx: The transaction is conflicting with another transaction. Conflicting specifically means a double spend situation that both transactions pass all validation rules, eventually losing one(s) should have this reason.
        InvalidReferencedUtxo: The referenced UTXO is invalid.
        InvalidTransaction: The transaction is invalid.
        SumInputsOutputsAmountMismatch: The sum of the inputs and output base token amount does not match.
        InvalidUnlockBlockSignature: The unlock block signature is invalid.
        TimelockNotExpired: The configured timelock is not yet expired.
        InvalidNativeTokens: The given native tokens are invalid.
        StorageDepositReturnUnfulfilled: The return amount in a transaction is not fulfilled by the output side.
        InvalidInputUnlock: An input unlock was invalid.
        SenderNotUnlocked: The output contains a Sender with an ident (address) which is not unlocked.
        InvalidChainStateTransition: The chain state transition is invalid.
        InvalidTransactionIssuingTime: The referenced input is created after the transaction issuing time.
        InvalidManaAmount: The mana amount is invalid.
        InvalidBlockIssuanceCreditsAmount: The Block Issuance Credits amount is invalid.
        InvalidRewardContextInput: Reward Context Input is invalid.
        InvalidCommitmentContextInput: Commitment Context Input is invalid.
        MissingStakingFeature: Staking Feature is not provided in account output when claiming rewards.
        FailedToClaimStakingReward: Failed to claim staking reward.
        FailedToClaimDelegationReward: Failed to claim delegation reward.
        TransactionCapabilityNativeTokenBurningNotAllowed: Burning of native tokens is not allowed in the transaction capabilities.
        TransactionCapabilityManaBurningNotAllowed: Burning of mana is not allowed in the transaction capabilities.
        TransactionCapabilityAccountDestructionNotAllowed: Destruction of accounts is not allowed in the transaction capabilities.
        TransactionCapabilityAnchorDestructionNotAllowed: Destruction of anchors is not allowed in the transaction capabilities.
        TransactionCapabilityFoundryDestructionNotAllowed: Destruction of foundries is not allowed in the transaction capabilities.
        TransactionCapabilityNftDestructionNotAllowed: Destruction of nfts is not allowed in the transaction capabilities.
        SemanticValidationFailed: The semantic validation failed for a reason not covered by the previous variants.
    """
    InputUtxoAlreadySpent = 1
    ConflictingWithAnotherTx = 2
    InvalidReferencedUtxo = 3
    InvalidTransaction = 4
    SumInputsOutputsAmountMismatch = 5
    InvalidUnlockBlockSignature = 6
    TimelockNotExpired = 7
    InvalidNativeTokens = 8
    StorageDepositReturnUnfulfilled = 9
    InvalidInputUnlock = 10
    SenderNotUnlocked = 11
    InvalidChainStateTransition = 12
    InvalidTransactionIssuingTime = 13
    InvalidManaAmount = 14
    InvalidBlockIssuanceCreditsAmount = 15
    InvalidRewardContextInput = 16
    InvalidCommitmentContextInput = 17
    MissingStakingFeature = 18
    FailedToClaimStakingReward = 19
    FailedToClaimDelegationReward = 20
    TransactionCapabilityNativeTokenBurningNotAllowed = 21
    TransactionCapabilityManaBurningNotAllowed = 22
    TransactionCapabilityAccountDestructionNotAllowed = 23
    TransactionCapabilityAnchorDestructionNotAllowed = 24
    TransactionCapabilityFoundryDestructionNotAllowed = 25
    TransactionCapabilityNftDestructionNotAllowed = 26
    SemanticValidationFailed = 255

    def __str__(self):
        return {
            1: "The referenced UTXO was already spent.",
            2: "The transaction is conflicting with another transaction. Conflicting specifically means a double spend situation that both transactions pass all validation rules, eventually losing one(s) should have this reason.",
            3: "The referenced UTXO is invalid.",
            4: "The transaction is invalid.",
            5: "The sum of the inputs and output base token amount does not match.",
            6: "The unlock block signature is invalid.",
            7: "The configured timelock is not yet expired.",
            8: "The given native tokens are invalid.",
            9: "The return amount in a transaction is not fulfilled by the output side.",
            10: "An input unlock was invalid.",
            11: "The output contains a Sender with an ident (address) which is not unlocked.",
            12: "The chain state transition is invalid.",
            13: "The referenced input is created after the transaction issuing time.",
            14: "The mana amount is invalid.",
            15: "The Block Issuance Credits amount is invalid.",
            16: "Reward Context Input is invalid.",
            17: "Commitment Context Input is invalid.",
            18: "Staking Feature is not provided in account output when claiming rewards.",
            19: "Failed to claim staking reward.",
            20: "Failed to claim delegation reward.",
            21: "Burning of native tokens is not allowed in the transaction capabilities.",
            22: "Burning of mana is not allowed in the transaction capabilities.",
            23: "Destruction of accounts is not allowed in the transaction capabilities.",
            24: "Destruction of anchors is not allowed in the transaction capabilities.",
            25: "Destruction of foundries is not allowed in the transaction capabilities.",
            26: "Destruction of nfts is not allowed in the transaction capabilities.",
            255: "The semantic validation failed for a reason not covered by the previous variants."
        }[self.value]


@json
@dataclass
class BlockWithMetadata:
    """Represents a block with its metadata.
    Response of GET /api/core/v3/blocks/{blockId}/full.

    Attributes:
        block: The block.
        metadata: The block metadata.
    """
    block: Block
    metadata: BlockMetadata
