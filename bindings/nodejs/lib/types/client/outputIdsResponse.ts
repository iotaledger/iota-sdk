// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * OutputIdsResponse.
 */
export class OutputIdsResponse {
    ledgerIndex!: number;
    cursor?: string;
    items!: string[];
}
