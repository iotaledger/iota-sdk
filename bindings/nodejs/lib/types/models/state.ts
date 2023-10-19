// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * The different states of a block.
 */
export declare type BlockState =
    | 'pending'
    | 'confirmed'
    | 'finalized'
    | 'rejected'
    | 'failed';

/**
 * The different states of a transaction.
 */
export declare type TransactionState =
    | 'pending'
    | 'confirmed'
    | 'finalized'
    | 'failed';
