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
        Pending: Stored but not accepted/confirmed.
        Accepted: Valid block referenced by some validators.
        Confirmed: Valid block referenced by more than 2/3 of the validators.
        Finalized: Accepted/confirmed block and the slot was finalized, can no longer be reverted.
        Rejected: Rejected by the node, and user should reissue payload if it contains one.
        Failed: Not successfully issued due to failure reason.
    """
    Pending = 0
    Accepted = 1
    Confirmed = 2
    Finalized = 3
    Rejected = 4
    Failed = 5


class TransactionState(Enum):
    """Describes the state of a transaction.

    Attributes:
        Pending: Not included yet.
        Accepted: Included.
        Confirmed: Included and its included block is confirmed.
        Finalized: Included, its included block is finalized and cannot be reverted anymore.
        Failed: Not successfully issued due to failure reason.
    """
    Pending = 0
    Accepted = 1
    Confirmed = 2
    Finalized = 3
    Failed = 4


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

    def __str__(self):
        return {
            1: "The block is too old to issue.",
            2: "One of the block's parents is too old.",
            3: "One of the block's parents does not exist.",
            4: "One of the block's parents is invalid.",
            5: "The block's issuer account could not be found.",
            6: "The block's protocol version is invalid.",
            7: "The mana cost could not be calculated.",
            8: "The block's issuer account burned insufficient Mana for a block.",
            9: "The account is invalid.",
            10: "The block's signature is invalid.",
            11: "The block is dropped due to congestion.",
            12: "The block payload is invalid.",
            255: "The block is invalid."
        }[self.value]


class TransactionFailureReason(Enum):
    """Represents the possible reasons for a conflicting transaction.
    """
    Null = 0,
    TypeInvalid = 1,
    Conflicting = 2,
    InputAlreadySpent = 3,
    InputCreationAfterTxCreation = 4,
    UnlockSignatureInvalid = 5,
    CommitmentInputMissing = 6,
    CommitmentInputReferenceInvalid = 7,
    BicInputReferenceInvalid = 8,
    RewardInputReferenceInvalid = 9,
    StakingRewardCalculationFailure = 10,
    DelegationRewardCalculationFailure = 11,
    InputOutputBaseTokenMismatch = 12,
    ManaOverflow = 13,
    InputOutputManaMismatch = 14,
    ManaDecayCreationIndexExceedsTargetIndex = 15,
    NativeTokenAmountLessThanZero = 16,
    NativeTokenSumExceedsUint256 = 17,
    NativeTokenSumUnbalanced = 18,
    MultiAddressLengthUnlockLengthMismatch = 19,
    MultiAddressUnlockThresholdNotReached = 20,
    NestedMultiUnlock = 21,
    SenderFeatureNotUnlocked = 22,
    IssuerFeatureNotUnlocked = 23,
    StakingRewardInputMissing = 24,
    StakingBlockIssuerFeatureMissing = 25,
    StakingCommitmentInputMissing = 26,
    StakingRewardClaimingInvalid = 27,
    StakingFeatureRemovedBeforeUnbonding = 28,
    StakingFeatureModifiedBeforeUnbonding = 29,
    StakingStartEpochInvalid = 30,
    StakingEndEpochTooEarly = 31,
    BlockIssuerCommitmentInputMissing = 32,
    BlockIssuanceCreditInputMissing = 33,
    BlockIssuerNotExpired = 34,
    BlockIssuerExpiryTooEarly = 35,
    ManaMovedOffBlockIssuerAccount = 36,
    AccountLocked = 37,
    TimelockCommitmentInputMissing = 38,
    TimelockNotExpired = 39,
    ExpirationCommitmentInputMissing = 40,
    ExpirationNotUnlockable = 41,
    ReturnAmountNotFulFilled = 42,
    NewChainOutputHasNonZeroedId = 43,
    ChainOutputImmutableFeaturesChanged = 44,
    ImplicitAccountDestructionDisallowed = 45,
    MultipleImplicitAccountCreationAddresses = 46,
    AccountInvalidFoundryCounter = 47,
    FoundryTransitionWithoutAccount = 48,
    FoundrySerialInvalid = 49,
    DelegationCommitmentInputMissing = 50,
    DelegationRewardInputMissing = 51,
    DelegationRewardsClaimingInvalid = 52,
    DelegationOutputTransitionedTwice = 53,
    DelegationModified = 54,
    DelegationStartEpochInvalid = 55,
    DelegationAmountMismatch = 56,
    DelegationEndEpochNotZero = 57,
    DelegationEndEpochInvalid = 58,
    CapabilitiesNativeTokenBurningNotAllowed = 59,
    CapabilitiesManaBurningNotAllowed = 60,
    CapabilitiesAccountDestructionNotAllowed = 61,
    CapabilitiesAnchorDestructionNotAllowed = 62,
    CapabilitiesFoundryDestructionNotAllowed = 63,
    CapabilitiesNftDestructionNotAllowed = 64,
    SemanticValidationFailed = 255,

    def __str__(self):
        return {
            0: "",
            1: "",
            2: "",
            3: "",
            4: "",
            5: "",
            6: "",
            7: "",
            8: "",
            9: "",
            10: "",
            11: "",
            12: "",
            13: "",
            14: "",
            15: "",
            16: "",
            17: "",
            18: "",
            19: "",
            20: "",
            21: "",
            22: "",
            23: "",
            24: "",
            25: "",
            26: "",
            27: "",
            28: "",
            29: "",
            30: "",
            31: "",
            32: "",
            33: "",
            34: "",
            35: "",
            36: "",
            37: "",
            38: "",
            39: "",
            40: "",
            41: "",
            42: "",
            43: "",
            44: "",
            45: "",
            46: "",
            47: "",
            48: "",
            49: "",
            50: "",
            51: "",
            52: "",
            53: "",
            54: "",
            55: "",
            56: "",
            57: "",
            58: "",
            59: "",
            60: "",
            61: "",
            62: "",
            63: "",
            64: "",
            255: "",
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
