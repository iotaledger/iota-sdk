# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import Enum, IntEnum
from dataclasses import dataclass
from typing import Optional
from iota_sdk.types.common import json
from iota_sdk.types.block.block import Block
from iota_sdk.types.block.id import BlockId


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
    block_id: BlockId
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
    """Represents the possible reasons for a failing transaction.
    """
    Null = 0
    TypeInvalid = 1
    Conflicting = 2
    InputAlreadySpent = 3
    InputCreationAfterTxCreation = 4
    UnlockSignatureInvalid = 5
    CommitmentInputMissing = 6
    CommitmentInputReferenceInvalid = 7
    BicInputReferenceInvalid = 8
    RewardInputReferenceInvalid = 9
    StakingRewardCalculationFailure = 10
    DelegationRewardCalculationFailure = 11
    InputOutputBaseTokenMismatch = 12
    ManaOverflow = 13
    InputOutputManaMismatch = 14
    ManaDecayCreationIndexExceedsTargetIndex = 15
    NativeTokenAmountLessThanZero = 16
    NativeTokenSumExceedsUint256 = 17
    NativeTokenSumUnbalanced = 18
    MultiAddressLengthUnlockLengthMismatch = 19
    MultiAddressUnlockThresholdNotReached = 20
    NestedMultiUnlock = 21
    SenderFeatureNotUnlocked = 22
    IssuerFeatureNotUnlocked = 23
    StakingRewardInputMissing = 24
    StakingBlockIssuerFeatureMissing = 25
    StakingCommitmentInputMissing = 26
    StakingRewardClaimingInvalid = 27
    StakingFeatureRemovedBeforeUnbonding = 28
    StakingFeatureModifiedBeforeUnbonding = 29
    StakingStartEpochInvalid = 30
    StakingEndEpochTooEarly = 31
    BlockIssuerCommitmentInputMissing = 32
    BlockIssuanceCreditInputMissing = 33
    BlockIssuerNotExpired = 34
    BlockIssuerExpiryTooEarly = 35
    ManaMovedOffBlockIssuerAccount = 36
    AccountLocked = 37
    TimelockCommitmentInputMissing = 38
    TimelockNotExpired = 39
    ExpirationCommitmentInputMissing = 40
    ExpirationNotUnlockable = 41
    ReturnAmountNotFulFilled = 42
    NewChainOutputHasNonZeroedId = 43
    ChainOutputImmutableFeaturesChanged = 44
    ImplicitAccountDestructionDisallowed = 45
    MultipleImplicitAccountCreationAddresses = 46
    AccountInvalidFoundryCounter = 47
    FoundryTransitionWithoutAccount = 48
    FoundrySerialInvalid = 49
    DelegationCommitmentInputMissing = 50
    DelegationRewardInputMissing = 51
    DelegationRewardsClaimingInvalid = 52
    DelegationOutputTransitionedTwice = 53
    DelegationModified = 54
    DelegationStartEpochInvalid = 55
    DelegationAmountMismatch = 56
    DelegationEndEpochNotZero = 57
    DelegationEndEpochInvalid = 58
    CapabilitiesNativeTokenBurningNotAllowed = 59
    CapabilitiesManaBurningNotAllowed = 60
    CapabilitiesAccountDestructionNotAllowed = 61
    CapabilitiesAnchorDestructionNotAllowed = 62
    CapabilitiesFoundryDestructionNotAllowed = 63
    CapabilitiesNftDestructionNotAllowed = 64
    SemanticValidationFailed = 255

    def __str__(self):
        return {
            0: "Null.",
            1: "Transaction type is invalid.",
            2: "Transaction is conflicting.",
            3: "Input already spent.",
            4: "Input creation slot after tx creation slot.",
            5: "Signature in unlock is invalid.",
            6: "Commitment input required with reward or BIC input.",
            7: "Commitment input references an invalid or non-existent commitment.",
            8: "BIC input reference cannot be loaded.",
            9: "Reward input does not reference a staking account or a delegation output.",
            10: "Staking rewards could not be calculated due to storage issues or overflow.",
            11: "Delegation rewards could not be calculated due to storage issues or overflow.",
            12: "Inputs and outputs do not spend/deposit the same amount of base tokens.",
            13: "Under- or overflow in Mana calculations.",
            14: "Inputs and outputs do not contain the same amount of Mana.",
            15: "Mana decay creation slot/epoch index exceeds target slot/epoch index.",
            16: "Native token amount must be greater than zero.",
            17: "Native token sum exceeds max value of a uint256.",
            18: "Native token sums are unbalanced.",
            19: "Multi address length and multi unlock length do not match.",
            20: "Multi address unlock threshold not reached.",
            21: "Multi unlocks can't be nested.",
            22: "Sender feature is not unlocked.",
            23: "Issuer feature is not unlocked.",
            24: "Staking feature removal or resetting requires a reward input.",
            25: "Block issuer feature missing for account with staking feature.",
            26: "Staking feature validation requires a commitment input.",
            27: "Staking feature must be removed or reset in order to claim rewards.",
            28: "Staking feature can only be removed after the unbonding period.",
            29: "Staking start epoch, fixed cost and staked amount cannot be modified while bonded.",
            30: "Staking start epoch must be the epoch of the transaction.",
            31: "Staking end epoch must be set to the transaction epoch plus the unbonding period.",
            32: "Commitment input missing for block issuer feature.",
            33: "Block issuance credit input missing for account with block issuer feature.",
            34: "Block issuer feature has not expired.",
            35: "Block issuer feature expiry set too early.",
            36: "Mana cannot be moved off block issuer accounts except with manalocks.",
            37: "Account is locked due to negative block issuance credits.",
            38: "Transaction's containing a timelock condition require a commitment input.",
            39: "Timelock not expired.",
            40: "Transaction's containing an expiration condition require a commitment input.",
            41: "Expiration unlock condition cannot be unlocked.",
            42: "Return amount not fulfilled.",
            43: "New chain output has non-zeroed ID.",
            44: "Immutable features in chain output modified during transition.",
            45: "Cannot destroy implicit account; must be transitioned to account.",
            46: "Multiple implicit account creation addresses on the input side.",
            47: "Foundry counter in account decreased or did not increase by the number of new foundries.",
            48: "Foundry output transitioned without accompanying account on input or output side.",
            49: "Foundry output serial number is invalid.",
            50: "Delegation output validation requires a commitment input.",
            51: "Delegation output cannot be destroyed without a reward input.",
            52: "Invalid delegation mana rewards claiming.",
            53: "Delegation output attempted to be transitioned twice.",
            54: "Delegated amount, validator ID and start epoch cannot be modified.",
            55: "Invalid start epoch.",
            56: "Delegated amount does not match amount.",
            57: "End epoch must be set to zero at output genesis.",
            58: "Delegation end epoch does not match current epoch.",
            59: "Native token burning is not allowed by the transaction capabilities.",
            60: "Mana burning is not allowed by the transaction capabilities.",
            61: "Account destruction is not allowed by the transaction capabilities.",
            62: "Anchor destruction is not allowed by the transaction capabilities.",
            63: "Foundry destruction is not allowed by the transaction capabilities.",
            64: "NFT destruction is not allowed by the transaction capabilities.",
            255: "Semantic validation failed.",
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
