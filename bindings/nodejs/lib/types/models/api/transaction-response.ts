// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { TransactionState } from '../state';
import { TransactionId } from '../../block';
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
     * The transaction failure reason.
     */
    transactionFailureReason?: TransactionFailureReason;
}
