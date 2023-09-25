// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { u64 } from '../../utils';
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
     * The rent structure according to TIP-19.
     */
    rentStructure: RentStructure;
    /**
     * Work structure lists the Work Score of each type, it is used to denote the computation costs of processing an object.
     */
    workScoreStructure: WorkScoreStructure;
    /**
     * Current supply of base token.
     */
    tokenSupply: u64;
    /**
     * The genesis timestamp at which the slots start to count.
     */
    genesisUnixTimestamp: u64;
    /**
     * The duration of a slot, in seconds.
     */
    slotDurationInSeconds: number;
    /**
     * The number of slots in an epoch expressed as an exponent of 2.
     */
    slotsPerEpochExponent: number;
    /**
     * The parameters used by mana calculation.
     */
    manaStructure: ManaStructure;
    /**
     * The unbonding period in epochs before an account can stop staking.
     */
    stakingUnbondingPeriod: u64;
    /**
     * The number of validation blocks that each validator should issue each slot.
     */
    validationBlocksPerSlot: number;
    /**
     * The number of epochs worth of Mana that a node is punished with for each additional validation block it issues.
     */
    punishmentEpochs: u64;
    /**
     * Determine if a block is eligible by evaluating issuingTime and commitments in its pastcone to ATT and lastCommittedSlot respectively.
     */
    livenessThreshold: u64;
    /**
     * MinCommittableAge is the minimum age relative to the accepted tangle time slot index that a slot can be committed.
     */
    minCommittableAge: u64;
    /**
     * MaxCommittableAge is the maximum age for a slot commitment to be included in a block relative to the slot index of the block issuing time.
     */
    maxCommittableAge: u64;
    /**
     * Determine the slot that should trigger a new committee selection for the next and upcoming epoch.
     */
    epochNearingThreshold: u64;
    /**
     * Congestion Control Parameters defines the parameters used to calculate the Reference Mana Cost (RMC).
     */
    congestionControlParameters: CongestionControlParameters;
    /**
     * The parameters used by signaling protocol parameters upgrade.
     */
    versionSignaling: VersionSignalingParameters;
}

/**
 * Work structure lists the Work Score of each type, it is used to denote the computation costs of processing an object.
 */
export interface WorkScoreStructure {
    /**
     * DataByte accounts for the network traffic per kibibyte.
     */
    dataByte: number;
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
 * Mana Structure defines the parameters used by mana calculation.
 */
export interface ManaStructure {
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
}

/**
 * Congestion Control Parameters defines the parameters used to calculate the Reference Mana Cost (RMC).
 */
export interface CongestionControlParameters {
    /**
     * The minimum value of the reference Mana cost.
     */
    minReferenceManaCost: u64;
    /**
     * The increase step size of the reference Mana cost.
     */
    increase: u64;
    /**
     * The decrease step size of the reference Mana cost.
     */
    decrease: u64;
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
     * The minimum amount of Mana that an account must have to have a block scheduled.
     */
    minMana: u64;
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
