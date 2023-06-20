// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Response from the /info endpoint.
 */
export interface INodeInfoStatus {
    /**
     * Is the node healthy.
     */
    isHealthy: boolean;
    /**
     * The pruning index.
     */
    pruningIndex: number;
}
