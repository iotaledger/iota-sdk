// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { u64 } from '../..';

/**
 * Timeline is divided into slots, and each slot has a corresponding slot index.
 * To calculate the slot index of a timestamp, `genesisTimestamp` and the duration of a slot are needed.
 * The slot index of timestamp `ts` is `(ts - genesisTimestamp)/duration + 1`.
 */
type SlotIndex = number;

/**
 * The tangle timeline is divided into epochs, and each epoch has a corresponding epoch index.
 * Epochs are further subdivided into slots, each with a `SlotIndex.
 * To calculate the epoch index of a timestamp, `slotsPerEpochExponent` and `slotDurationInSeconds` are needed.
 * An epoch consists of `2^slotsPerEpochExponent` slots.
 */
type EpochIndex = number;

/**
 * Identifier of a slot commitment
 */
type SlotCommitmentId = string;

/**
 * A BLAKE2b-256 hash of concatenating multiple sparse merkle tree roots of a slot.
 */
type RootsId = string;

/**
 * Contains a summary of a slot.
 * It is linked to the commitment of the previous slot, which forms a commitment chain.
 */
class SlotCommitment {
    /**
     * The slot index of this commitment.
     * It is calculated based on genesis timestamp and the duration of a slot.
     */
    readonly index: SlotIndex;
    /**
     * The commitment ID of the previous slot.
     */
    readonly prevId: SlotCommitmentId;
    /**
     * A BLAKE2b-256 hash of concatenating multiple sparse merkle tree roots of a slot.
     */
    readonly rootsId: RootsId;

    /**
     * The sum of previous slot commitment cumulative weight and weight of issuers of accepted blocks within this
     * slot. It is just an indication of "committed into" this slot, and can not strictly be used for evaluating
     * the switching of a chain.
     */
    readonly cumulativeWeight: u64;

    constructor(
        index: SlotIndex,
        previousSlotCommitmentId: SlotCommitmentId,
        rootsId: RootsId,
        cumulativeWeight: u64,
    ) {
        this.index = index;
        this.prevId = previousSlotCommitmentId;
        this.rootsId = rootsId;
        this.cumulativeWeight = cumulativeWeight;
    }
}

export { SlotCommitment, SlotIndex, EpochIndex, SlotCommitmentId, RootsId };
