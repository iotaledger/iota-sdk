// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { TransactionState } from '../state';
import { SlotIndex, TransactionId } from '../../block';
import { TransactionFailureReason } from './transaction-failure-reason';

/**
 * Metadata of a transaction.
 */
export interface TransactionMetadataResponse {
    /**
     * The transaction id.
     */
    transactionId: TransactionId;
    /**
     * The transaction state.
     */
    transactionState: TransactionState;
    /**
    * The slot of the earliest included valid block that contains an attachment of the transaction.
    */
    earliestAttachmentSlot: SlotIndex;
    /**
     * The transaction failure reason.
     */
    transactionFailureReason?: TransactionFailureReason;
}
