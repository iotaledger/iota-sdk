// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { HexEncodedString } from '../utils/hex-encoding';
import { BlockState, TransactionState } from './state';
import { BlockFailureReason } from './block-failure-reason';
import { Block } from '../block';
import { TransactionId } from '../wallet';
import { TransactionFailureReason } from './transaction-failure-reason';

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
     * The block failure reason.
     */
    blockFailureReason?: BlockFailureReason;
    /**
     * The metadata of the transactions in the block.
     */
    transactionMetadata?: TransactionMetadata;
}

/**
 * Response from the full endpoint.
 */
export interface IBlockWithMetadata {
    /**
     * The block.
     */
    block: Block;
    /**
     * The block metadata.
     */
    metadata: IBlockMetadata;
}

/**
 * Returns the metadata of a given transaction.
 */
export interface TransactionMetadata {
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
