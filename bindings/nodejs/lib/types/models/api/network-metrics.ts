// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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
