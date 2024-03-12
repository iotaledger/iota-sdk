// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Temp solution for not double parsing JSON
import { plainToInstance, Type } from 'class-transformer';
import { BlockId, TransactionId } from '../../block/id';
import { Output, OutputId, OutputDiscriminator } from '../../block/output';
import { SlotCommitmentId, SlotIndex } from '../../block/slot';
import { TransactionState } from '../state';
import { HexEncodedString } from '../../utils';

/**
 * OutputCommitmentProof types.
 */
export enum OutputCommitmentProofType {
    /** Denotes a HashableNode. */
    HashableNode = 0,
    /** Denotes a LeafHash. */
    LeafHash = 1,
    /** Denotes a ValueHash. */
    ValueHash = 2,
}

export abstract class OutputCommitmentProof {
    readonly type: OutputCommitmentProofType;

    /**
     * @param type The type of OutputCommitmentProof.
     */
    constructor(type: OutputCommitmentProofType) {
        this.type = type;
    }

    /**
     * Parse an OutputCommitmentProof from a plain JSON object.
     */
    public static parse(data: any): OutputCommitmentProof {
        if (data.type == OutputCommitmentProofType.HashableNode) {
            return plainToInstance(HashableNode, data) as any as HashableNode;
        } else if (data.type == OutputCommitmentProofType.LeafHash) {
            return plainToInstance(LeafHash, data) as any as LeafHash;
        } else if (data.type == OutputCommitmentProofType.ValueHash) {
            return plainToInstance(ValueHash, data) as any as ValueHash;
        }
        throw new Error('Invalid JSON');
    }
}

/**
 * Contains the hashes of the left and right children of a node in the OutputCommitmentProof tree.
 */
export class HashableNode extends OutputCommitmentProof {
    readonly l: OutputCommitmentProof;
    readonly r: OutputCommitmentProof;

    /**
     * @param l Output commitment proof of the left subtree.
     * @param r Output commitment proof of the right subtree.
     */
    constructor(l: OutputCommitmentProof, r: OutputCommitmentProof) {
        super(OutputCommitmentProofType.HashableNode);
        this.l = l;
        this.r = r;
    }
}

/**
 * Contains the hash of a leaf in the OutputCommitmentProof tree.
 */
export class LeafHash extends OutputCommitmentProof {
    readonly hash: HexEncodedString;

    /**
     * @param hash The hash of the leaf.
     */
    constructor(hash: HexEncodedString) {
        super(OutputCommitmentProofType.LeafHash);
        this.hash = hash;
    }
}

/**
 * Contains the hash of the value for which the OutputCommitmentProof is being computed.
 */
export class ValueHash extends OutputCommitmentProof {
    readonly hash: HexEncodedString;

    /**
     * @param hash The hash of the value.
     */
    constructor(hash: HexEncodedString) {
        super(OutputCommitmentProofType.ValueHash);
        this.hash = hash;
    }
}

/**
 * The proof of the output identifier.
 */
export class OutputIdProof {
    readonly slot: SlotIndex;
    readonly outputIndex: number;
    readonly transactionCommitment: HexEncodedString;
    readonly outputCommitmentProof: OutputCommitmentProof;

    /**
     * @param slot The slot index of the output.
     * @param outputIndex The index of the output within the corresponding transaction.
     * @param transactionCommitment The commitment of the transaction that created the output. Hex-encoded with 0x prefix.
     * @param outputCommitmentProof The proof of the output commitment.
     */
    constructor(
        slot: SlotIndex,
        outputIndex: number,
        transactionCommitment: HexEncodedString,
        outputCommitmentProof: OutputCommitmentProof,
    ) {
        this.slot = slot;
        this.outputIndex = outputIndex;
        this.transactionCommitment = transactionCommitment;
        this.outputCommitmentProof = outputCommitmentProof;
    }
}

/**
 * An output with its output id proof.
 * Response of GET /api/core/v3/outputs/{outputId}.
 */
export class OutputResponse {
    /**
     * One of the possible output types.
     */
    @Type(() => Output, {
        discriminator: OutputDiscriminator,
    })
    output!: Output;
    /**
     * The proof of the output identifier.
     */
    outputIdProof!: OutputIdProof;
}

/**
 * Metadata of the output if it is included in the ledger.
 */
export interface OutputInclusionMetadata {
    /**
     * Slot in which the output was included.
     */
    slot: SlotIndex;
    /**
     * Transaction ID that created the output.
     */
    transactionId: TransactionId;
    /**
     * Commitment ID that includes the creation of the output.
     */
    commitmentId?: SlotCommitmentId;
}

/**
 * Metadata of the output if it is marked as spent in the ledger.
 */
export interface OutputConsumptionMetadata {
    /**
     * Slot in which the output was spent.
     */
    slot: SlotIndex;
    /**
     * Transaction ID that spent the output.
     */
    transactionId: TransactionId;
    /**
     * Commitment ID that includes the spending of the output.
     */
    commitmentId?: SlotCommitmentId;
}

/**
 * Metadata of an output.
 */
export interface OutputMetadataResponse {
    /**
     * The ID of the output.
     */
    outputId: OutputId;
    /**
     * The ID of the block in which the output was included.
     */
    blockId: BlockId;
    /**
     * Metadata of the output if it is included in the ledger.
     */
    included: OutputInclusionMetadata;
    /**
     * Metadata of the output if it is marked as spent in the ledger.
     */
    spent?: OutputConsumptionMetadata;
    /**
     * Latest commitment ID of the node.
     */
    latestCommitmentId: SlotCommitmentId;
}

/**
 * An output with its output id proof and its metadata.
 * Response of GET /api/core/v3/outputs/{outputId}/full.
 */
export class OutputWithMetadataResponse {
    /**
     * One of the possible output types.
     */
    output!: Output;
    /**
     * The associated Output ID proof.
     */
    outputIdProof!: OutputIdProof;
    /**
     * The metadata of the output.
     */
    metadata!: OutputMetadataResponse;
}

/**
 * An output and its metadata.
 */
export class OutputWithMetadata {
    /**
     * One of the possible output types.
     */
    output!: Output;
    /**
     * The metadata of the output.
     */
    metadata!: OutputMetadataResponse;
}

/**
 * Metadata of a transaction.
 */
export interface TransactionMetadataResponse {
    /**
     * The transaction id.
     */
    transactionId: TransactionId;
    /**
     * The transaction state.
     */
    transactionState: TransactionState;
    /**
     * The slot of the earliest included valid block that contains an attachment of the transaction.
     */
    earliestAttachmentSlot: SlotIndex;
    /**
     * If applicable, indicates the error that occurred during the transaction processing.
     */
    transactionFailureReason?: TransactionFailureReason;
    /**
     * Contains the detailed error message that occurred during the transaction processing if the debug mode was activated in the retainer.
     */
    transactionFailureDetails?: string;
}

/**
 * Reason for transaction failure.
 */
export enum TransactionFailureReason {
    None = 0,
    ConflictRejected = 1,
    Orphaned = 2,
    InputAlreadySpent = 3,
    InputCreationAfterTxCreation = 4,
    UnlockSignatureInvalid = 5,
    ChainAddressUnlockInvalid = 6,
    DirectUnlockableAddressUnlockInvalid = 7,
    MultiAddressUnlockInvalid = 8,
    CommitmentInputReferenceInvalid = 9,
    BicInputReferenceInvalid = 10,
    RewardInputReferenceInvalid = 11,
    StakingRewardCalculationFailure = 12,
    DelegationRewardCalculationFailure = 13,
    InputOutputBaseTokenMismatch = 14,
    ManaOverflow = 15,
    InputOutputManaMismatch = 16,
    ManaDecayCreationIndexExceedsTargetIndex = 17,
    NativeTokenSumUnbalanced = 18,
    SimpleTokenSchemeMintedMeltedTokenDecrease = 19,
    SimpleTokenSchemeMintingInvalid = 20,
    SimpleTokenSchemeMeltingInvalid = 21,
    SimpleTokenSchemeMaximumSupplyChanged = 22,
    SimpleTokenSchemeGenesisInvalid = 23,
    MultiAddressLengthUnlockLengthMismatch = 24,
    MultiAddressUnlockThresholdNotReached = 25,
    SenderFeatureNotUnlocked = 26,
    IssuerFeatureNotUnlocked = 27,
    StakingRewardInputMissing = 28,
    StakingCommitmentInputMissing = 29,
    StakingRewardClaimingInvalid = 30,
    StakingFeatureRemovedBeforeUnbonding = 31,
    StakingFeatureModifiedBeforeUnbonding = 32,
    StakingStartEpochInvalid = 33,
    StakingEndEpochTooEarly = 34,
    BlockIssuerCommitmentInputMissing = 35,
    BlockIssuanceCreditInputMissing = 36,
    BlockIssuerNotExpired = 37,
    BlockIssuerExpiryTooEarly = 38,
    ManaMovedOffBlockIssuerAccount = 39,
    AccountLocked = 40,
    TimelockCommitmentInputMissing = 41,
    TimelockNotExpired = 42,
    ExpirationCommitmentInputMissing = 43,
    ExpirationNotUnlockable = 44,
    ReturnAmountNotFulFilled = 45,
    NewChainOutputHasNonZeroedId = 46,
    ChainOutputImmutableFeaturesChanged = 47,
    ImplicitAccountDestructionDisallowed = 48,
    MultipleImplicitAccountCreationAddresses = 49,
    AccountInvalidFoundryCounter = 50,
    AnchorInvalidStateTransition = 51,
    AnchorInvalidGovernanceTransition = 52,
    FoundryTransitionWithoutAccount = 53,
    FoundrySerialInvalid = 54,
    DelegationCommitmentInputMissing = 55,
    DelegationRewardInputMissing = 56,
    DelegationRewardsClaimingInvalid = 57,
    DelegationOutputTransitionedTwice = 58,
    DelegationModified = 59,
    DelegationStartEpochInvalid = 60,
    DelegationAmountMismatch = 61,
    DelegationEndEpochNotZero = 62,
    DelegationEndEpochInvalid = 63,
    CapabilitiesNativeTokenBurningNotAllowed = 64,
    CapabilitiesManaBurningNotAllowed = 65,
    CapabilitiesAccountDestructionNotAllowed = 66,
    CapabilitiesAnchorDestructionNotAllowed = 67,
    CapabilitiesFoundryDestructionNotAllowed = 68,
    CapabilitiesNftDestructionNotAllowed = 69,
    SemanticValidationFailed = 255,
}

/**
 * Transaction failure reason strings.
 */
export const TRANSACTION_FAILURE_REASON_STRINGS: {
    [key in TransactionFailureReason]: string;
} = {
    [TransactionFailureReason.None]: 'None.',
    [TransactionFailureReason.ConflictRejected]:
        'Transaction was conflicting and was rejected.',
    [TransactionFailureReason.Orphaned]: 'Transaction was orphaned.',
    [TransactionFailureReason.InputAlreadySpent]: 'Input already spent.',
    [TransactionFailureReason.InputCreationAfterTxCreation]:
        'Input creation slot after tx creation slot.',
    [TransactionFailureReason.UnlockSignatureInvalid]:
        'Signature in unlock is invalid.',
    [TransactionFailureReason.ChainAddressUnlockInvalid]:
        'invalid unlock for chain address.',
    [TransactionFailureReason.DirectUnlockableAddressUnlockInvalid]:
        'invalid unlock for direct unlockable address.',
    [TransactionFailureReason.MultiAddressUnlockInvalid]:
        'invalid unlock for multi address.',
    [TransactionFailureReason.CommitmentInputReferenceInvalid]:
        'Commitment input references an invalid or non-existent commitment.',
    [TransactionFailureReason.BicInputReferenceInvalid]:
        'BIC input reference cannot be loaded.',
    [TransactionFailureReason.RewardInputReferenceInvalid]:
        'Reward input does not reference a staking account or a delegation output.',
    [TransactionFailureReason.StakingRewardCalculationFailure]:
        'Staking rewards could not be calculated due to storage issues or overflow.',
    [TransactionFailureReason.DelegationRewardCalculationFailure]:
        'Delegation rewards could not be calculated due to storage issues or overflow.',
    [TransactionFailureReason.InputOutputBaseTokenMismatch]:
        'Inputs and outputs do not spend/deposit the same amount of base tokens.',
    [TransactionFailureReason.ManaOverflow]:
        'Under- or overflow in Mana calculations.',
    [TransactionFailureReason.InputOutputManaMismatch]:
        'Inputs and outputs do not contain the same amount of Mana.',
    [TransactionFailureReason.ManaDecayCreationIndexExceedsTargetIndex]:
        'Mana decay creation slot/epoch index exceeds target slot/epoch index.',
    [TransactionFailureReason.NativeTokenSumUnbalanced]:
        'Native token sums are unbalanced.',
    [TransactionFailureReason.SimpleTokenSchemeMintedMeltedTokenDecrease]:
        "Simple token scheme's minted or melted tokens decreased.",
    [TransactionFailureReason.SimpleTokenSchemeMintingInvalid]:
        "Simple token scheme's minted tokens did not increase by the minted amount or melted tokens changed.",
    [TransactionFailureReason.SimpleTokenSchemeMeltingInvalid]:
        "Simple token scheme's melted tokens did not increase by the melted amount or minted tokens changed.",
    [TransactionFailureReason.SimpleTokenSchemeMaximumSupplyChanged]:
        "Simple token scheme's maximum supply cannot change during transition.",
    [TransactionFailureReason.SimpleTokenSchemeGenesisInvalid]:
        "Newly created simple token scheme's melted tokens are not zero or minted tokens do not equal native token amount in transaction.",
    [TransactionFailureReason.MultiAddressLengthUnlockLengthMismatch]:
        'Multi address length and multi unlock length do not match.',
    [TransactionFailureReason.MultiAddressUnlockThresholdNotReached]:
        'Multi address unlock threshold not reached.',
    [TransactionFailureReason.SenderFeatureNotUnlocked]:
        'Sender feature is not unlocked.',
    [TransactionFailureReason.IssuerFeatureNotUnlocked]:
        'Issuer feature is not unlocked.',
    [TransactionFailureReason.StakingRewardInputMissing]:
        'Staking feature removal or resetting requires a reward input.',
    [TransactionFailureReason.StakingCommitmentInputMissing]:
        'Staking feature validation requires a commitment input.',
    [TransactionFailureReason.StakingRewardClaimingInvalid]:
        'Staking feature must be removed or reset in order to claim rewards.',
    [TransactionFailureReason.StakingFeatureRemovedBeforeUnbonding]:
        'Staking feature can only be removed after the unbonding period.',
    [TransactionFailureReason.StakingFeatureModifiedBeforeUnbonding]:
        'Staking start epoch, fixed cost and staked amount cannot be modified while bonded.',
    [TransactionFailureReason.StakingStartEpochInvalid]:
        'Staking start epoch must be the epoch of the transaction.',
    [TransactionFailureReason.StakingEndEpochTooEarly]:
        'Staking end epoch must be set to the transaction epoch plus the unbonding period.',
    [TransactionFailureReason.BlockIssuerCommitmentInputMissing]:
        'Commitment input missing for block issuer feature.',
    [TransactionFailureReason.BlockIssuanceCreditInputMissing]:
        'Block issuance credit input missing for account with block issuer feature.',
    [TransactionFailureReason.BlockIssuerNotExpired]:
        'Block issuer feature has not expired.',
    [TransactionFailureReason.BlockIssuerExpiryTooEarly]:
        'Block issuer feature expiry set too early.',
    [TransactionFailureReason.ManaMovedOffBlockIssuerAccount]:
        'Mana cannot be moved off block issuer accounts except with manalocks.',
    [TransactionFailureReason.AccountLocked]:
        'Account is locked due to negative block issuance credits.',
    [TransactionFailureReason.TimelockCommitmentInputMissing]:
        "Transaction's containing a timelock condition require a commitment input.",
    [TransactionFailureReason.TimelockNotExpired]: 'Timelock not expired.',
    [TransactionFailureReason.ExpirationCommitmentInputMissing]:
        "Transaction's containing an expiration condition require a commitment input.",
    [TransactionFailureReason.ExpirationNotUnlockable]:
        'Expiration unlock condition cannot be unlocked.',
    [TransactionFailureReason.ReturnAmountNotFulFilled]:
        'Return amount not fulfilled.',
    [TransactionFailureReason.NewChainOutputHasNonZeroedId]:
        'New chain output has non-zeroed ID.',
    [TransactionFailureReason.ChainOutputImmutableFeaturesChanged]:
        'Immutable features in chain output modified during transition.',
    [TransactionFailureReason.ImplicitAccountDestructionDisallowed]:
        'Cannot destroy implicit account; must be transitioned to account.',
    [TransactionFailureReason.MultipleImplicitAccountCreationAddresses]:
        'Multiple implicit account creation addresses on the input side.',
    [TransactionFailureReason.AccountInvalidFoundryCounter]:
        'Foundry counter in account decreased or did not increase by the number of new foundries.',
    [TransactionFailureReason.AnchorInvalidStateTransition]:
        'Invalid anchor state transition.',
    [TransactionFailureReason.AnchorInvalidGovernanceTransition]:
        'Invalid anchor governance transition.',
    [TransactionFailureReason.FoundryTransitionWithoutAccount]:
        'Foundry output transitioned without accompanying account on input or output side.',
    [TransactionFailureReason.FoundrySerialInvalid]:
        'Foundry output serial number is invalid.',
    [TransactionFailureReason.DelegationCommitmentInputMissing]:
        'Delegation output validation requires a commitment input.',
    [TransactionFailureReason.DelegationRewardInputMissing]:
        'Delegation output cannot be destroyed without a reward input.',
    [TransactionFailureReason.DelegationRewardsClaimingInvalid]:
        'Invalid delegation mana rewards claiming.',
    [TransactionFailureReason.DelegationOutputTransitionedTwice]:
        'Delegation output attempted to be transitioned twice.',
    [TransactionFailureReason.DelegationModified]:
        'Delegated amount, validator ID and start epoch cannot be modified.',
    [TransactionFailureReason.DelegationStartEpochInvalid]:
        'Delegation output has invalid start epoch.',
    [TransactionFailureReason.DelegationAmountMismatch]:
        'Delegated amount does not match amount.',
    [TransactionFailureReason.DelegationEndEpochNotZero]:
        'End epoch must be set to zero at output genesis.',
    [TransactionFailureReason.DelegationEndEpochInvalid]:
        'Delegation end epoch does not match current epoch.',
    [TransactionFailureReason.CapabilitiesNativeTokenBurningNotAllowed]:
        'Native token burning is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesManaBurningNotAllowed]:
        'Mana burning is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesAccountDestructionNotAllowed]:
        'Account destruction is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesAnchorDestructionNotAllowed]:
        'Anchor destruction is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesFoundryDestructionNotAllowed]:
        'Foundry destruction is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesNftDestructionNotAllowed]:
        'NFT destruction is not allowed by the transaction capabilities.',
    [TransactionFailureReason.SemanticValidationFailed]:
        'Semantic validation failed.',
};
