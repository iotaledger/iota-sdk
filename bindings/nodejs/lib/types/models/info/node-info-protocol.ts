// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { RentStructure } from '../rent-structure';

/**
 * The Protocol Info.
 */
export interface ProtocolInfo {
    /**
     * The start epoch of the set of protocol parameters.
     */
    startEpoch: string;
    /**
     * The protocol parameters.
     */
    parameters: ProtocolParameters[];
}

/**
 * The Protocol Parameters.
 */
export interface ProtocolParameters {
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
     * The rent structure according to TIP-19.
     */
    rentStructure: RentStructure;
    /**
     * Work structure lists the Work Score of each type, it is used to denote the computation costs of processing an object.
     */
    workScoreStructure: WorkScoreStructure;
    /**
     * Current supply of base token. Plain string encoded number.
     */
    tokenSupply: string;
    /**
     * The genesis timestamp at which the slots start to count.
     */
    genesisUnixTimestamp: string;
    /**
     * The duration of a slot, in seconds.
     */
    slotDurationInSeconds: number;
    /**
     * The number of slots in an epoch expressed as an exponent of 2.
     */
    slotsPerEpochExponent: number;
    /**
     * Mana Bits is the number of bits used to represent Mana expressed as an exponent of 2.
     */
    manaBitsExponent: number;
    /**
     * The amount of potential Mana generated by 1 IOTA in 1 slot.
     */
    manaGenerationRate: number;
    /**
     * The scaling of ManaGenerationRate expressed as an exponent of 2.
     */
    manaGenerationRateExponent: number;
    /**
     * A lookup table of epoch index diff to mana decay factor (slice index 0 = 1 epoch).
     */
    manaDecayFactors: number[];
    /**
     * The scaling of ManaDecayFactors expressed as an exponent of 2.
     */
    manaDecayFactorsExponent: number;
    /**
     * An integer approximation of the sum of decay over epochs.
     */
    manaDecayFactorEpochsSum: number;
    /**
     * The scaling of ManaDecayFactorEpochsSum expressed as an exponent of 2.
     */
    manaDecayFactorEpochsSumExponent: number;
    /**
     * The unbonding period in epochs before an account can stop staking.
     */
    stakingUnbondingPeriod: string;
    /**
     * Determine if a block is eligible by evaluating issuingTime and commitments in its pastcone to ATT and lastCommittedSlot respectively.
     */
    livenessThreshold: string;
    /**
     * MinCommittableAge is the minimum age relative to the accepted tangle time slot index that a slot can be committed.
     */
    minCommittableAge: string;
    /**
     * MaxCommittableAge is the maximum age for a slot commitment to be included in a block relative to the slot index of the block issuing time.
     */
    maxCommittableAge: string;
    /**
     * Determine the slot that should trigger a new committee selection for the next and upcoming epoch.
     */
    epochNearingThreshold: string;
    /**
     * Congestion Control Parameters defines the parameters used to calculate the Reference Mana Cost (RMC).
     */
    congestionControlParameters: CongestionControlParameters;
    /**
     * The version signaling parameters.
     */
    versionSignaling: VersionSignalingParameters;
}

/**
 * Work structure lists the Work Score of each type, it is used to denote the computation costs of processing an object.
 */
export interface WorkScoreStructure {
    /**
     * DataKilobyte accounts for the network traffic per kilobyte.
     */
    dataKilobyte: number;
    /**
     * Block accounts for work done to process a block in the node software.
     */
    block: number;
    /**
     * MissingParent is used for slashing if there are not enough strong tips.
     */
    missingParent: number;
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
    /**
     * MinStrongParentsThreshold is the minimum amount of strong parents in a basic block, otherwise the issuer gets slashed.
     */
    minStrongParentsThreshold: number;
}

/**
 * Congestion Control Parameters defines the parameters used to calculate the Reference Mana Cost (RMC).
 */
export interface CongestionControlParameters {
    /**
     * RMCMin is the minimum value of the reference Mana cost.
     */
    rmcMin: string;
    /**
     * Increase is the increase step size of the reference Mana cost.
     */
    increase: string;
    /**
     * Decrease is the decrease step size of the reference Mana cost.
     */
    decrease: string;
    /**
     * IncreaseThreshold is the threshold for increasing the reference Mana cost.
     */
    increaseThreshold: number;
    /**
     * DecreaseThreshold is the threshold for decreasing the reference Mana cost.
     */
    decreaseThreshold: number;
    /**
     * SchedulerRate is the rate at which the scheduler runs in workscore units per second.
     */
    schedulerRate: number;
    /**
     * MinMana is the minimum amount of Mana that an account must have to have a block scheduled.
     */
    minMana: string;
    /**
     * MaxBufferSize is the maximum size of the buffer.
     */
    maxBufferSize: number;
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
