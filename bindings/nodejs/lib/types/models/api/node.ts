// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { StorageScoreParameters } from '../storage-score';
import { SlotIndex, EpochIndex } from '../../block/slot';
import { NumericString } from '../../utils';

/**
 * Response from the /info endpoint.
 */
export interface StatusResponse {
    /**
     * Tells whether the node is healthy or not.
     */
    isHealthy: boolean;
    /**
     * A notion of time that is anchored to the latest accepted block.
     */
    acceptedTangleTime?: string;
    /**
     * The Accepted Tangle Time after it has advanced with the system clock.
     */
    relativeAcceptedTangleTime?: string;
    /**
     * A notion of time that is anchored to the latest confirmed block.
     */
    confirmedTangleTime?: string;
    /**
     * The Confirmed Tangle Time after it has advanced with the system clock.
     */
    relativeConfirmedTangleTime?: string;
    /**
     * The latest slot that the node has committed to.
     */
    latestCommitmentId: string;
    /**
     * The index of latest finalized slot.
     */
    latestFinalizedSlot: SlotIndex;
    /**
     * The slot index of the latest accepted block.
     */
    latestAcceptedBlockSlot?: SlotIndex;
    /**
     * The slot index of the latest confirmed block.
     */
    latestConfirmedBlockSlot?: SlotIndex;
    /**
     * The index of the epoch before which the tangle history is pruned.
     */
    pruningEpoch: EpochIndex;
}

/**
 * The Protocol Parameters response.
 */
export interface ProtocolParametersResponse {
    /**
     * The protocol parameters.
     */
    parameters: ProtocolParameters;
    /**
     * The start epoch of the set of protocol parameters.
     */
    startEpoch: EpochIndex;
}

/**
 * The Protocol Parameters.
 */
export interface ProtocolParameters {
    /**
     * Set to value 0 to denote a IOTA 2.0 protocol parameter.
     */
    type: number;
    /**
     * Protocol version used by the network.
     */
    version: number;
    /**
     * The human friendly name of the network on which the node operates on.
     */
    networkName: string;
    /**
     * Tells whether the node supports mainnet or testnet addresses. Value `iota` indicates that the node supports mainnet addresses. Value `atoi` indicates that the node supports testnet addresses.
     */
    bech32Hrp: string;
    /**
     * The storage score parameters used by given node/network.
     */
    storageScoreParameters: StorageScoreParameters;
    /**
     * Work Score Parameters lists the work score of each type, it is used to denote the computation costs of processing an object.
     */
    workScoreParameters: WorkScoreParameters;
    /**
     * The parameters used by mana calculation.
     */
    manaParameters: ManaParameters;
    /**
     * Current supply of base token.
     */
    tokenSupply: NumericString;
    /**
     * Genesis Slot defines the slot of the genesis.
     */
    genesisSlot: number;
    /**
     * The genesis timestamp at which the slots start to count.
     */
    genesisUnixTimestamp: NumericString;
    /**
     * The duration of a slot, in seconds.
     */
    slotDurationInSeconds: number;
    /**
     * The number of slots in an epoch expressed as an exponent of 2.
     */
    slotsPerEpochExponent: number;
    /**
     * The unbonding period in epochs before an account can stop staking.
     */
    stakingUnbondingPeriod: number;
    /**
     * The number of validation blocks that each validator should issue each slot.
     */
    validationBlocksPerSlot: number;
    /**
     * The number of epochs worth of Mana that a node is punished with for each additional validation block it issues.
     */
    punishmentEpochs: number;
    /**
     * Liveness Threshold Lower Bound is used by tip-selection to determine if a block is eligible by evaluating issuingTimes.
     */
    livenessThresholdLowerBound: number;
    /**
     * Liveness Threshold Upper Bound is used by tip-selection to determine if a block is eligible by evaluating issuingTimes.
     */
    livenessThresholdUpperBound: number;
    /**
     * MinCommittableAge is the minimum age relative to the accepted tangle time slot index that a slot can be committed.
     */
    minCommittableAge: number;
    /**
     * MaxCommittableAge is the maximum age for a slot commitment to be included in a block relative to the slot index of the block issuing time.
     */
    maxCommittableAge: number;
    /**
     * Determine the slot that should trigger a new committee selection for the next and upcoming epoch.
     */
    epochNearingThreshold: number;
    /**
     * Congestion Control Parameters defines the parameters used to calculate the Reference Mana Cost (RMC).
     */
    congestionControlParameters: CongestionControlParameters;
    /**
     * The parameters used by signaling protocol parameters upgrade.
     */
    versionSignalingParameters: VersionSignalingParameters;
    /**
     * Rewards Parameters defines the parameters that are used to calculate Mana rewards.
     */
    rewardsParameters: RewardsParameters;
    /**
     * Target Committee Size defines the target size of the committee. If there's fewer candidates the actual committee size could be smaller in a given epoch.
     */
    targetCommitteeSize: number;
    /**
     * Defines the number of heavier slots that a chain needs to be ahead of the current chain to be considered for
     * switching.
     */
    chainSwitchingThreshold: number;
}

/**
 * Rewards Parameters defines the parameters that are used to calculate Mana rewards.
 */
export interface RewardsParameters {
    /**
     * Profit Margin Exponent is used for shift operation for calculation of profit margin.
     */
    profitMarginExponent: number;
    /**
     * The length in epochs of the bootstrapping phase.
     */
    bootstrappingDuration: number;
    /**
     * The rate of Mana rewards at the start of the bootstrapping phase.
     */
    rewardToGenerationRatio: number;
    /**
     * Decay Balancing Constant Exponent is the exponent used for calculation of the initial reward.
     */
    initialTargetRewardsRate: NumericString;
    /**
     * The rate of Mana rewards after the bootstrapping phase.
     */
    finalTargetRewardsRate: NumericString;
    /**
     * Pool Coefficient Exponent is the exponent used for shifting operation
     * in the pool rewards calculations.
     */
    poolCoefficientExponent: number;
    /**
     * The number of epochs for which rewards are retained.
     */
    retentionPeriod: number;
}

/**
 * Work Score Parameters lists the work score of each type, it is used to denote the computation costs of processing an object.
 */
export interface WorkScoreParameters {
    /**
     * DataByte accounts for the network traffic per kibibyte.
     */
    dataByte: number;
    /**
     * Block accounts for work done to process a block in the node software.
     */
    block: number;
    /**
     * Input accounts for loading the UTXO from the database and performing the mana calculations.
     */
    input: number;
    /**
     * ContextInput accounts for loading and checking the context input.
     */
    contextInput: number;
    /**
     * Output accounts for storing the UTXO in the database.
     */
    output: number;
    /**
     * NativeToken accounts for calculations done with native tokens.
     */
    nativeToken: number;
    /**
     * Staking accounts for the existence of a staking feature in the output.
     */
    staking: number;
    /**
     * BlockIssuer accounts for the existence of a block issuer feature in the output.
     */
    blockIssuer: number;
    /**
     * Allotment accounts for accessing the account based ledger to transform the mana to block issuance credits.
     */
    allotment: number;
    /**
     * SignatureEd25519 accounts for an Ed25519 signature check.
     */
    signatureEd25519: number;
}

/**
 * ManaParameters defines the parameters used by mana calculation.
 */
export interface ManaParameters {
    /**
     * The number of bits used to represent Mana.
     */
    bitsCount: number;
    /**
     * The amount of potential Mana generated by 1 IOTA in 1 slot.
     */
    generationRate: number;
    /**
     * The scaling of ManaGenerationRate expressed as an exponent of 2.
     */
    generationRateExponent: number;
    /**
     * A lookup table of epoch index diff to mana decay factor (slice index 0 = 1 epoch).
     */
    decayFactors: number[];
    /**
     * The scaling of ManaDecayFactors expressed as an exponent of 2.
     */
    decayFactorsExponent: number;
    /**
     * An integer approximation of the sum of decay over epochs.
     */
    decayFactorEpochsSum: number;
    /**
     * The scaling of ManaDecayFactorEpochsSum expressed as an exponent of 2.
     */
    decayFactorEpochsSumExponent: number;
    /**
     * Decay factor for 1 year.
     */
    annualDecayFactorPercentage: number;
}

/**
 * Congestion Control Parameters defines the parameters used to calculate the Reference Mana Cost (RMC).
 */
export interface CongestionControlParameters {
    /**
     * The minimum value of the reference Mana cost.
     */
    minReferenceManaCost: NumericString;
    /**
     * The increase step size of the reference Mana cost.
     */
    increase: NumericString;
    /**
     * The decrease step size of the reference Mana cost.
     */
    decrease: NumericString;
    /**
     * The threshold for increasing the reference Mana cost.
     */
    increaseThreshold: number;
    /**
     * The threshold for decreasing the reference Mana cost.
     */
    decreaseThreshold: number;
    /**
     * The rate at which the scheduler runs in workscore units per second.
     */
    schedulerRate: number;
    /**
     * The maximum size of the buffer.
     */
    maxBufferSize: number;
    /**
     * The maximum number of blocks in the validation buffer.
     */
    maxValidationBufferSize: number;
}

/**
 * The version signaling parameters.
 */
export interface VersionSignalingParameters {
    /**
     * The size of the window in epochs to find which version of protocol parameters was most signaled, from currentEpoch - windowSize to currentEpoch.
     */
    windowSize: number;
    /**
     * The target number of supporters for a version to win in a windowSize.
     */
    windowTargetRatio: number;
    /**
     * The offset in epochs to activate the new version of protocol parameters.
     */
    activationOffset: number;
}

/**
 * The base token info of the node.
 */
export interface BaseTokenResponse {
    /**
     * The base token name.
     */
    name: string;
    /**
     * The base token ticker symbol.
     */
    tickerSymbol: string;
    /**
     * The base token unit.
     */
    unit: string;
    /**
     * The base token decimals.
     */
    decimals: number;
    /**
     * The base token sub-unit.
     */
    subunit?: string;
    /**
     * The use metric prefix flag.
     */
    useMetricPrefix: boolean;
}

/**
 * Response from the /info endpoint.
 */
export interface InfoResponse {
    /**
     * The name of the node.
     */
    name: string;
    /**
     * The semantic version of the node.
     */
    version: string;
    /**
     * The status of the node.
     */
    status: StatusResponse;
    /**
     * The protocol parameters.
     */
    protocolParameters: ProtocolParametersResponse[];
    /**
     * The base token info of the node.
     */
    baseToken: BaseTokenResponse;
}

/**
 * Metrics information about the network.
 */
export interface NetworkMetricsResponse {
    /**
     * The current rate of new blocks per second.
     */
    blocksPerSecond: string;
    /**
     * The current rate of confirmed blocks per second.
     */
    confirmedBlocksPerSecond: string;
    /**
     * The ratio of confirmed blocks to new blocks of the last confirmed slot.
     */
    confirmationRate: string;
}

/**
 * Response from the /routes endpoint.
 */
export interface RoutesResponse {
    /**
     * The routes the node exposes.
     */
    routes: string[];
}
