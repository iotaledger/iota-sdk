// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * The different states of a block.
 * 'pending':   The block has been booked by the node but not yet accepted.
 * 'accepted':  The block has been referenced by the super majority of the online committee.
 * 'confirmed': The block has been referenced by the super majority of the total committee.
 * 'finalized': The commitment containing the block has been finalized.
 *              This state is computed based on the accepted/confirmed block's slot being smaller or equal than the latest finalized slot.
 * 'dropped':   The block has been dropped due to congestion control.
 * 'orphaned':  The block's slot has been committed by the node without the block being included.
 *              In this case, the block will never be finalized unless there is a chain switch.
 *              This state is computed based on the pending block's slot being smaller or equal than the latest committed slot.
 */
export declare type BlockState =
    | 'pending'
    | 'accepted'
    | 'confirmed'
    | 'finalized'
    | 'dropped'
    | 'orphaned';

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
