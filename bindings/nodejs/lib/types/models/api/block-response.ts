// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BlockState } from '../state';
import { BlockFailureReason } from './block-failure-reason';
import { Block, BlockId } from '../../block';
import { TransactionMetadataResponse } from './transaction-response';

/**
 * Response from the metadata endpoint.
 */
export interface BlockMetadataResponse {
    /**
     * The block id.
     */
    blockId: BlockId;
    /**
     * The block state.
     */
    blockState: BlockState;
    /**
     * The block failure reason.
     */
    blockFailureReason?: BlockFailureReason;
    /**
     * The metadata of the transaction in the block.
     */
    transactionMetadata?: TransactionMetadataResponse;
}

/**
 * Response from the full endpoint.
 */
export interface BlockFullResponse {
    /**
     * The block.
     */
    block: Block;
    /**
     * The block metadata.
     */
    metadata: BlockMetadataResponse;
}
