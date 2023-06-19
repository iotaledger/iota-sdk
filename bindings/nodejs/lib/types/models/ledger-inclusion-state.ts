// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * The different states of ledger inclusion.
 */
export declare type LedgerInclusionState =
    | 'noTransaction'
    | 'included'
    | 'conflicting';
