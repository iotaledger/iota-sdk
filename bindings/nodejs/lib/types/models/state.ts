// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * The different states of a block.
 * 'pending': stored but not accepted/confirmed.
 * 'accepted': valid block referenced by some validators.
 * 'confirmed': valid block referenced by more than 2/3 of the validators.
 * 'finalized': accepted/confirmed block and the slot was finalized, can no longer be reverted.
 * 'rejected': rejected by the node, and user should reissue payload if it contains one.
 * 'failed': not successfully issued due to failure reason.
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
 * 'pending': the transaction is not included yet.
 * 'accepted': the transaction is included.
 * 'confirmed': the transaction is included and its included block is confirmed.
 * 'finalized': the transaction is included, its included block is finalized and cannot be reverted anymore.
 * 'failed': the transaction is not successfully issued due to failure reason.
 */
export declare type TransactionState =
    | 'pending'
    | 'accepted'
    | 'confirmed'
    | 'finalized'
    | 'failed';
