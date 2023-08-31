// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Response from the /info endpoint.
 */
export interface INodeInfoMetrics {
    /**
     * The current rate of new blocks per second.
     */
    blocksPerSecond: string;
    /**
     * The current rate of confirmed blocks per second.
     */
    confirmedBlocksPerSecond: string;
    /**
     * The ratio of confirmed blocks in relation to new blocks of the last confirmed slot.
     */
    confirmationRate: string;
}
