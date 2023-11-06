// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { TransactionFailureReason } from './transaction-failure-reason';
import type { HexEncodedString } from '../utils/hex-encoding';
import { BlockState, TransactionState } from './state';
import { BlockFailureReason } from './block-failure-reason';

/**
 * Response from the metadata endpoint.
 */
export interface IBlockMetadata {
    /**
     * The block id.
     */
    blockId: HexEncodedString;
    /**
     * The block state.
     */
    blockState: BlockState;
    /**
     * The transaction state.
     */
    transactionState?: TransactionState;
    /**
     * The block failure reason.
     */
    blockFailureReason?: BlockFailureReason;
    /**
     * The transaction failure reason.
     */
    transactionFailureReason?: TransactionFailureReason;
}
