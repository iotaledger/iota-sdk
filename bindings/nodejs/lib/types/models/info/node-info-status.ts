// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Response from the /info endpoint.
 */
export interface INodeInfoStatus {
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
    latestFinalizedSlot: string;
    /**
     * The slot index of the latest accepted block.
     */
    latestAcceptedBlockSlot?: string;
    /**
     * The slot index of the latest confirmed block.
     */
    latestConfirmedBlockSlot?: string;
    /**
     * The index of the epoch before which the tangle history is pruned.
     */
    pruningEpoch: string;
}
