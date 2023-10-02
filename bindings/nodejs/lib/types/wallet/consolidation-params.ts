// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/** Parameters for consolidation */
export interface ConsolidationParams {
    /** Ignores the output threshold if set to `true`. */
    force: boolean;
    /** Consolidates if the output number is >= the output_threshold. */
    outputThreshold?: number;
    /** Address to which the consolidated output should be sent. */
    targetAddress?: string;
}
