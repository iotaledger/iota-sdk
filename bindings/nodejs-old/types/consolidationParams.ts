// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/** Parameters for consolidation */
export interface ConsolidationParams {
    force: boolean,
    outputThreshold?: number,
    targetAddress?: string,
}