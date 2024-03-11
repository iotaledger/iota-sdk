# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import Enum


class TransactionState(str, Enum):
    """Describes the state of a transaction.

    Attributes:
        Pending:    The transaction has been booked by the node but not yet accepted.
        Accepted:   The transaction meets the following 4 conditions:
                        - Signatures of the transaction are valid.
                        - The transaction has been approved by the super majority of the online committee (potential conflicts are resolved by this time).
                        - The transactions that created the inputs were accepted (monotonicity).
                        - At least one valid attachment was accepted.
        Committed:  The slot of the earliest accepted attachment of the transaction was committed.
        Finalized:  The transaction is accepted and the slot containing the transaction has been finalized by the node.
                    This state is computed based on the accepted transaction's earliest included attachment slot being smaller or equal than the latest finalized slot.
        Failed:     The transaction has not been executed by the node due to a failure during processing.
    """
    Pending = 'pending'
    Accepted = 'accepted'
    Committed = 'committed'
    Finalized = 'finalized'
    Failed = 'failed'


class TransactionFailureReason(Enum):
    """Represents the possible reasons for a failing transaction.
    """
    Null = 0
    ConflictRejected = 1
    Orphaned = 2
    InputAlreadySpent = 3
    InputCreationAfterTxCreation = 4
    UnlockSignatureInvalid = 5
    ChainAddressUnlockInvalid = 6
    DirectUnlockableAddressUnlockInvalid = 7
    MultiAddressUnlockInvalid = 8
    CommitmentInputReferenceInvalid = 9
    BicInputReferenceInvalid = 10
    RewardInputReferenceInvalid = 11
    StakingRewardCalculationFailure = 12
    DelegationRewardCalculationFailure = 13
    InputOutputBaseTokenMismatch = 14
    ManaOverflow = 15
    InputOutputManaMismatch = 16
    ManaDecayCreationIndexExceedsTargetIndex = 17
    NativeTokenSumUnbalanced = 18
    SimpleTokenSchemeMintedMeltedTokenDecrease = 19
    SimpleTokenSchemeMintingInvalid = 20
    SimpleTokenSchemeMeltingInvalid = 21
    SimpleTokenSchemeMaximumSupplyChanged = 22
    SimpleTokenSchemeGenesisInvalid = 23
    MultiAddressLengthUnlockLengthMismatch = 24
    MultiAddressUnlockThresholdNotReached = 25
    SenderFeatureNotUnlocked = 26
    IssuerFeatureNotUnlocked = 27
    StakingRewardInputMissing = 28
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
    AccountInvalidFoundryCounter = 40
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
            2: "Transaction was orphaned.",
            3: "Input already spent.",
            4: "Input creation slot after tx creation slot.",
            5: "Signature in unlock is invalid.",
            6: "Invalid unlock for chain address.",
            7: "Invalid unlock for direct unlockable address.",
            8: "Invalid unlock for multi address.",
            9: "Commitment input references an invalid or non-existent commitment.",
            10: "BIC input reference cannot be loaded.",
            11: "Reward input does not reference a staking account or a delegation output.",
            12: "Staking rewards could not be calculated due to storage issues or overflow.",
            13: "Delegation rewards could not be calculated due to storage issues or overflow.",
            14: "Inputs and outputs do not spend/deposit the same amount of base tokens.",
            15: "Under- or overflow in Mana calculations.",
            16: "Inputs and outputs do not contain the same amount of Mana.",
            17: "Mana decay creation slot/epoch index exceeds target slot/epoch index.",
            18: "Native token sums are unbalanced.",
            19: "Simple token scheme minted/melted value decreased.",
            20: "Simple token scheme's minted tokens did not increase by the minted amount or melted tokens changed.",
            21: "Simple token scheme's melted tokens did not increase by the melted amount or minted tokens changed.",
            22: "Simple token scheme's maximum supply cannot change during transition.",
            23: "Newly created simple token scheme's melted tokens are not zero or minted tokens do not equal native token amount in transaction.",
            24: "Multi address length and multi unlock length do not match.",
            25: "Multi address unlock threshold not reached.",
            26: "Sender feature is not unlocked.",
            27: "Issuer feature is not unlocked.",
            28: "Staking feature removal or resetting requires a reward input.",
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
