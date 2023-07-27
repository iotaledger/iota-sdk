// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { INodeInfoMilestone } from './node-info-milestone';
/**
 * Response from the /info endpoint.
 */
export interface INodeInfoStatus {
    /**
     * Is the node healthy.
     */
    isHealthy: boolean;
    /**
     * The latest milestone info.
     */
    latestMilestone: INodeInfoMilestone;
    /**
     * The confirmed milestone info.
     */
    confirmedMilestone: INodeInfoMilestone;
    /**
     * The pruning index.
     */
    pruningIndex: number;
}
