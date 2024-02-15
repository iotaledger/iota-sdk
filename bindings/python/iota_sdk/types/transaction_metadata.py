# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import Enum


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


class TransactionFailureReason(Enum):
    """Represents the possible reasons for a failing transaction.
    """
    Null = 0
    ConflictRejected = 1
    InputAlreadySpent = 2
    InputCreationAfterTxCreation = 3
    UnlockSignatureInvalid = 4
    CommitmentInputReferenceInvalid = 5
    BicInputReferenceInvalid = 6
    RewardInputReferenceInvalid = 7
    StakingRewardCalculationFailure = 8
    DelegationRewardCalculationFailure = 9
    InputOutputBaseTokenMismatch = 10
    ManaOverflow = 11
    InputOutputManaMismatch = 12
    ManaDecayCreationIndexExceedsTargetIndex = 13
    NativeTokenSumUnbalanced = 14
    SimpleTokenSchemeMintedMeltedTokenDecrease = 15
    SimpleTokenSchemeMintingInvalid = 16
    SimpleTokenSchemeMeltingInvalid = 17
    SimpleTokenSchemeMaximumSupplyChanged = 18
    SimpleTokenSchemeGenesisInvalid = 19
    MultiAddressLengthUnlockLengthMismatch = 20
    MultiAddressUnlockThresholdNotReached = 21
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
    AnchorInvalidStateTransition = 48
    AnchorInvalidGovernanceTransition = 49
    FoundryTransitionWithoutAccount = 50
    FoundrySerialInvalid = 51
    DelegationCommitmentInputMissing = 52
    DelegationRewardInputMissing = 53
    DelegationRewardsClaimingInvalid = 54
    DelegationOutputTransitionedTwice = 55
    DelegationModified = 56
    DelegationStartEpochInvalid = 57
    DelegationAmountMismatch = 58
    DelegationEndEpochNotZero = 59
    DelegationEndEpochInvalid = 60
    CapabilitiesNativeTokenBurningNotAllowed = 61
    CapabilitiesManaBurningNotAllowed = 62
    CapabilitiesAccountDestructionNotAllowed = 63
    CapabilitiesAnchorDestructionNotAllowed = 64
    CapabilitiesFoundryDestructionNotAllowed = 65
    CapabilitiesNftDestructionNotAllowed = 66
    SemanticValidationFailed = 255

    def __str__(self):
        return {
            0: "Null.",
            1: "Transaction was conflicting and was rejected.",
            2: "Input already spent.",
            3: "Input creation slot after tx creation slot.",
            4: "Signature in unlock is invalid.",
            5: "Commitment input required with reward or BIC input.",
            6: "BIC input reference cannot be loaded.",
            7: "Reward input does not reference a staking account or a delegation output.",
            8: "Staking rewards could not be calculated due to storage issues or overflow.",
            9: "Delegation rewards could not be calculated due to storage issues or overflow.",
            10: "Inputs and outputs do not spend/deposit the same amount of base tokens.",
            11: "Under- or overflow in Mana calculations.",
            12: "Inputs and outputs do not contain the same amount of Mana.",
            13: "Mana decay creation slot/epoch index exceeds target slot/epoch index.",
            14: "Native token sums are unbalanced.",
            15: "Simple token scheme minted/melted value decreased.",
            16: "Simple token scheme minting invalid.",
            17: "Simple token scheme melting invalid.",
            18: "Simple token scheme maximum supply changed.",
            19: "Simple token scheme genesis invalid.",
            20: "Multi address length and multi unlock length do not match.",
            21: "Multi address unlock threshold not reached.",
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
            48: "Anchor has an invalid state transition.",
            49: "Anchor has an invalid governance transition.",
            50: "Foundry output transitioned without accompanying account on input or output side.",
            51: "Foundry output serial number is invalid.",
            52: "Delegation output validation requires a commitment input.",
            53: "Delegation output cannot be destroyed without a reward input.",
            54: "Invalid delegation mana rewards claiming.",
            55: "Delegation output attempted to be transitioned twice.",
            56: "Delegated amount, validator ID and start epoch cannot be modified.",
            57: "Invalid start epoch.",
            58: "Delegated amount does not match amount.",
            59: "End epoch must be set to zero at output genesis.",
            60: "Delegation end epoch does not match current epoch.",
            61: "Native token burning is not allowed by the transaction capabilities.",
            62: "Mana burning is not allowed by the transaction capabilities.",
            63: "Account destruction is not allowed by the transaction capabilities.",
            64: "Anchor destruction is not allowed by the transaction capabilities.",
            65: "Foundry destruction is not allowed by the transaction capabilities.",
            66: "NFT destruction is not allowed by the transaction capabilities.",
            255: "Semantic validation failed.",
        }[self.value]
