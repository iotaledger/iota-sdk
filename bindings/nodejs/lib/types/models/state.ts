// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * The different states of a block.
 */
export declare type BlockState =
    | 'pending'
    | 'accepted'
    | 'confirmed'
    | 'finalized'
    | 'rejected'
    | 'failed';

/**
 * The different states of a transaction.
 * If 'pending', the transaction is not included yet.
 * If 'accepted', the transaction is included.
 * If 'confirmed', the transaction is included and its included block is confirmed.
 * If 'finalized', the transaction is included, its included block is finalized and cannot be reverted anymore.
 * If 'failed', the transaction is not successfully issued due to failure reason.
 */
export declare type TransactionState =
    | 'pending'
    | 'accepted'
    | 'confirmed'
    | 'finalized'
    | 'failed';
