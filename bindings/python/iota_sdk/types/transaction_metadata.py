# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import Enum


class TransactionState(str, Enum):
    """Describes the state of a transaction.

    Attributes:
        Pending: Not included yet.
        Accepted: Included.
        Confirmed: Included and its included block is confirmed.
        Finalized: Included, its included block is finalized and cannot be reverted anymore.
        Failed: Not successfully issued due to failure reason.
    """
    Pending = 'pending'
    Accepted = 'accepted'
    Confirmed = 'confirmed'
    Finalized = 'finalized'
    Failed = 'failed'


class TransactionFailureReason(Enum):
    """Represents the possible reasons for a failing transaction.
    """
    Null = 0
    ConflictRejected = 1
    InputAlreadySpent = 2
    InputCreationAfterTxCreation = 3
    UnlockSignatureInvalid = 4
    ChainAddressUnlockInvalid = 5
    DirectUnlockableAddressUnlockInvalid = 6
    MultiAddressUnlockInvalid = 7
    CommitmentInputReferenceInvalid = 8
    BicInputReferenceInvalid = 9
    RewardInputReferenceInvalid = 10
    StakingRewardCalculationFailure = 11
    DelegationRewardCalculationFailure = 12
    InputOutputBaseTokenMismatch = 13
    ManaOverflow = 14
    InputOutputManaMismatch = 15
    ManaDecayCreationIndexExceedsTargetIndex = 16
    NativeTokenSumUnbalanced = 17
    SimpleTokenSchemeMintedMeltedTokenDecrease = 18
    SimpleTokenSchemeMintingInvalid = 19
    SimpleTokenSchemeMeltingInvalid = 20
    SimpleTokenSchemeMaximumSupplyChanged = 21
    SimpleTokenSchemeGenesisInvalid = 22
    MultiAddressLengthUnlockLengthMismatch = 23
    MultiAddressUnlockThresholdNotReached = 24
    SenderFeatureNotUnlocked = 25
    IssuerFeatureNotUnlocked = 26
    StakingRewardInputMissing = 27
    StakingCommitmentInputMissing = 28
    StakingRewardClaimingInvalid = 29
    StakingFeatureRemovedBeforeUnbonding = 30
    StakingFeatureModifiedBeforeUnbonding = 31
    StakingStartEpochInvalid = 32
    StakingEndEpochTooEarly = 33
    BlockIssuerCommitmentInputMissing = 34
    BlockIssuanceCreditInputMissing = 35
    BlockIssuerNotExpired = 36
    BlockIssuerExpiryTooEarly = 37
    ManaMovedOffBlockIssuerAccount = 38
    AccountLocked = 39
    TimelockCommitmentInputMissing = 40
    TimelockNotExpired = 41
    ExpirationCommitmentInputMissing = 42
    ExpirationNotUnlockable = 43
    ReturnAmountNotFulFilled = 44
    NewChainOutputHasNonZeroedId = 45
    ChainOutputImmutableFeaturesChanged = 46
    ImplicitAccountDestructionDisallowed = 47
    MultipleImplicitAccountCreationAddresses = 48
    AccountInvalidFoundryCounter = 49
    AnchorInvalidStateTransition = 50
    AnchorInvalidGovernanceTransition = 51
    FoundryTransitionWithoutAccount = 52
    FoundrySerialInvalid = 53
    DelegationCommitmentInputMissing = 54
    DelegationRewardInputMissing = 54
    DelegationRewardsClaimingInvalid = 56
    DelegationOutputTransitionedTwice = 57
    DelegationModified = 58
    DelegationStartEpochInvalid = 59
    DelegationAmountMismatch = 60
    DelegationEndEpochNotZero = 61
    DelegationEndEpochInvalid = 62
    CapabilitiesNativeTokenBurningNotAllowed = 63
    CapabilitiesManaBurningNotAllowed = 64
    CapabilitiesAccountDestructionNotAllowed = 65
    CapabilitiesAnchorDestructionNotAllowed = 66
    CapabilitiesFoundryDestructionNotAllowed = 67
    CapabilitiesNftDestructionNotAllowed = 68
    SemanticValidationFailed = 255

    def __str__(self):
        return {
            0: "Null.",
            1: "Transaction was conflicting and was rejected.",
            2: "Input already spent.",
            3: "Input creation slot after tx creation slot.",
            4: "Signature in unlock is invalid.",
            5: "Invalid unlock for chain address.",
            6: "Invalid unlock for direct unlockable address.",
            7: "Invalid unlock for multi address.",
            8: "Commitment input references an invalid or non-existent commitment.",
            9: "BIC input reference cannot be loaded.",
            10: "Reward input does not reference a staking account or a delegation output.",
            11: "Staking rewards could not be calculated due to storage issues or overflow.",
            12: "Delegation rewards could not be calculated due to storage issues or overflow.",
            13: "Inputs and outputs do not spend/deposit the same amount of base tokens.",
            14: "Under- or overflow in Mana calculations.",
            15: "Inputs and outputs do not contain the same amount of Mana.",
            16: "Mana decay creation slot/epoch index exceeds target slot/epoch index.",
            17: "Native token sums are unbalanced.",
            18: "Simple token scheme minted/melted value decreased.",
            19: "Simple token scheme's minted tokens did not increase by the minted amount or melted tokens changed.",
            20: "Simple token scheme's melted tokens did not increase by the melted amount or minted tokens changed.",
            21: "Simple token scheme's maximum supply cannot change during transition.",
            22: "Newly created simple token scheme's melted tokens are not zero or minted tokens do not equal native token amount in transaction.",
            23: "Multi address length and multi unlock length do not match.",
            24: "Multi address unlock threshold not reached.",
            25: "Sender feature is not unlocked.",
            26: "Issuer feature is not unlocked.",
            27: "Staking feature removal or resetting requires a reward input.",
            28: "Staking feature validation requires a commitment input.",
            29: "Staking feature must be removed or reset in order to claim rewards.",
            30: "Staking feature can only be removed after the unbonding period.",
            31: "Staking start epoch, fixed cost and staked amount cannot be modified while bonded.",
            32: "Staking start epoch must be the epoch of the transaction.",
            33: "Staking end epoch must be set to the transaction epoch plus the unbonding period.",
            34: "Commitment input missing for block issuer feature.",
            35: "Block issuance credit input missing for account with block issuer feature.",
            36: "Block issuer feature has not expired.",
            37: "Block issuer feature expiry set too early.",
            38: "Mana cannot be moved off block issuer accounts except with manalocks.",
            39: "Account is locked due to negative block issuance credits.",
            40: "Transaction's containing a timelock condition require a commitment input.",
            41: "Timelock not expired.",
            42: "Transaction's containing an expiration condition require a commitment input.",
            43: "Expiration unlock condition cannot be unlocked.",
            44: "Return amount not fulfilled.",
            45: "New chain output has non-zeroed ID.",
            46: "Immutable features in chain output modified during transition.",
            47: "Cannot destroy implicit account; must be transitioned to account.",
            48: "Multiple implicit account creation addresses on the input side.",
            49: "Foundry counter in account decreased or did not increase by the number of new foundries.",
            50: "Anchor has an invalid state transition.",
            51: "Anchor has an invalid governance transition.",
            52: "Foundry output transitioned without accompanying account on input or output side.",
            53: "Foundry output serial number is invalid.",
            54: "Delegation output validation requires a commitment input.",
            55: "Delegation output cannot be destroyed without a reward input.",
            56: "Invalid delegation mana rewards claiming.",
            57: "Delegation output attempted to be transitioned twice.",
            58: "Delegated amount, validator ID and start epoch cannot be modified.",
            59: "Invalid start epoch.",
            60: "Delegated amount does not match amount.",
            61: "End epoch must be set to zero at output genesis.",
            62: "Delegation end epoch does not match current epoch.",
            63: "Native token burning is not allowed by the transaction capabilities.",
            64: "Mana burning is not allowed by the transaction capabilities.",
            65: "Account destruction is not allowed by the transaction capabilities.",
            66: "Anchor destruction is not allowed by the transaction capabilities.",
            67: "Foundry destruction is not allowed by the transaction capabilities.",
            68: "NFT destruction is not allowed by the transaction capabilities.",
            255: "Semantic validation failed.",
        }[self.value]
