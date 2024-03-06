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
 * 'pending':   The transaction has been booked by the node but not yet accepted.
 * 'accepted':  The transaction meets the following 4 conditions:
 *	                - Signatures of the transaction are valid.
 *                  - The transaction has been approved by the super majority of the online committee (potential conflicts are resolved by this time).
 *	                - The transactions that created the inputs were accepted (monotonicity).
 *                  - At least one valid attachment was accepted.
 * 'committed': The slot of the earliest accepted attachment of the transaction was committed.
 * 'finalized': The transaction is accepted and the slot containing the transaction has been finalized by the node.
 *              This state is computed based on the accepted transaction's earliest included attachment slot being smaller or equal than the latest finalized slot.
 * 'failed':    The transaction has not been executed by the node due to a failure during processing.
 */
export declare type TransactionState =
    | 'pending'
    | 'accepted'
    | 'committed'
    | 'finalized'
    | 'failed';
