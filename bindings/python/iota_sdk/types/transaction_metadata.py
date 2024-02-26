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
    StakingBlockIssuerFeatureMissing = 28
    StakingCommitmentInputMissing = 29
    StakingRewardClaimingInvalid = 30
    StakingFeatureRemovedBeforeUnbonding = 31
    StakingFeatureModifiedBeforeUnbonding = 32
    StakingStartEpochInvalid = 33
    StakingEndEpochTooEarly = 34
    BlockIssuerCommitmentInputMissing = 35
    BlockIssuanceCreditInputMissing = 36
    BlockIssuerNotExpired = 37
    BlockIssuerExpiryTooEarly = 38
    ManaMovedOffBlockIssuerAccount = 39
    AccountLocked = 40
    TimelockCommitmentInputMissing = 41
    TimelockNotExpired = 42
    ExpirationCommitmentInputMissing = 43
    ExpirationNotUnlockable = 44
    ReturnAmountNotFulFilled = 45
    NewChainOutputHasNonZeroedId = 46
    ChainOutputImmutableFeaturesChanged = 47
    ImplicitAccountDestructionDisallowed = 48
    MultipleImplicitAccountCreationAddresses = 49
    AccountInvalidFoundryCounter = 50
    AnchorInvalidStateTransition = 51
    AnchorInvalidGovernanceTransition = 52
    FoundryTransitionWithoutAccount = 53
    FoundrySerialInvalid = 54
    DelegationCommitmentInputMissing = 55
    DelegationRewardInputMissing = 56
    DelegationRewardsClaimingInvalid = 57
    DelegationOutputTransitionedTwice = 58
    DelegationModified = 59
    DelegationStartEpochInvalid = 60
    DelegationAmountMismatch = 61
    DelegationEndEpochNotZero = 62
    DelegationEndEpochInvalid = 63
    CapabilitiesNativeTokenBurningNotAllowed = 64
    CapabilitiesManaBurningNotAllowed = 65
    CapabilitiesAccountDestructionNotAllowed = 66
    CapabilitiesAnchorDestructionNotAllowed = 67
    CapabilitiesFoundryDestructionNotAllowed = 68
    CapabilitiesNftDestructionNotAllowed = 69
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
            8: "Commitment input required with reward or BIC input.",
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
            19: "Simple token scheme minting invalid.",
            20: "Simple token scheme melting invalid.",
            21: "Simple token scheme maximum supply changed.",
            22: "Simple token scheme genesis invalid.",
            23: "Multi address length and multi unlock length do not match.",
            24: "Multi address unlock threshold not reached.",
            25: "Sender feature is not unlocked.",
            26: "Issuer feature is not unlocked.",
            27: "Staking feature removal or resetting requires a reward input.",
            28: "Block issuer feature missing for account with staking feature.",
            29: "Staking feature validation requires a commitment input.",
            30: "Staking feature must be removed or reset in order to claim rewards.",
            31: "Staking feature can only be removed after the unbonding period.",
            32: "Staking start epoch, fixed cost and staked amount cannot be modified while bonded.",
            33: "Staking start epoch must be the epoch of the transaction.",
            34: "Staking end epoch must be set to the transaction epoch plus the unbonding period.",
            35: "Commitment input missing for block issuer feature.",
            36: "Block issuance credit input missing for account with block issuer feature.",
            37: "Block issuer feature has not expired.",
            38: "Block issuer feature expiry set too early.",
            39: "Mana cannot be moved off block issuer accounts except with manalocks.",
            40: "Account is locked due to negative block issuance credits.",
            41: "Transaction's containing a timelock condition require a commitment input.",
            42: "Timelock not expired.",
            43: "Transaction's containing an expiration condition require a commitment input.",
            44: "Expiration unlock condition cannot be unlocked.",
            45: "Return amount not fulfilled.",
            46: "New chain output has non-zeroed ID.",
            47: "Immutable features in chain output modified during transition.",
            48: "Cannot destroy implicit account; must be transitioned to account.",
            49: "Multiple implicit account creation addresses on the input side.",
            50: "Foundry counter in account decreased or did not increase by the number of new foundries.",
            51: "Anchor has an invalid state transition.",
            52: "Anchor has an invalid governance transition.",
            53: "Foundry output transitioned without accompanying account on input or output side.",
            54: "Foundry output serial number is invalid.",
            55: "Delegation output validation requires a commitment input.",
            56: "Delegation output cannot be destroyed without a reward input.",
            57: "Invalid delegation mana rewards claiming.",
            58: "Delegation output attempted to be transitioned twice.",
            59: "Delegated amount, validator ID and start epoch cannot be modified.",
            60: "Invalid start epoch.",
            61: "Delegated amount does not match amount.",
            62: "End epoch must be set to zero at output genesis.",
            63: "Delegation end epoch does not match current epoch.",
            64: "Native token burning is not allowed by the transaction capabilities.",
            65: "Mana burning is not allowed by the transaction capabilities.",
            66: "Account destruction is not allowed by the transaction capabilities.",
            67: "Anchor destruction is not allowed by the transaction capabilities.",
            68: "Foundry destruction is not allowed by the transaction capabilities.",
            69: "NFT destruction is not allowed by the transaction capabilities.",
            255: "Semantic validation failed.",
        }[self.value]
