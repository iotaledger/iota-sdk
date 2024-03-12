// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BlockState } from '../state';
import { Block, BlockId, SlotCommitment, SlotIndex } from '../../block';
import { NumericString } from '../../utils';

/**
 * Information that is used to attach a block in the network.
 * Response of GET /api/core/v3/blocks/issuance
 */
export interface IssuanceBlockHeaderResponse {
    /**
     * Blocks that are strongly directly approved.
     */
    strongParents: BlockId[];
    /**
     * Latest issuing time of the returned parents.
     */
    latestParentBlockIssuingTime: NumericString;
    /**
     * The slot index of the latest finalized slot.
     */
    latestFinalizedSlot: SlotIndex;
    /**
     * The latest slot commitment.
     */
    latestCommitment: SlotCommitment;
    /**
     * Blocks that are weakly directly approved.
     */
    weakParents?: BlockId[];
    /**
     * Blocks that are directly referenced to adjust opinion.
     */
    shallowLikeParents?: BlockId[];
}

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
}

/**
 * Response from the full endpoint.
 */
export interface BlockWithMetadataResponse {
    /**
     * The block.
     */
    block: Block;
    /**
     * The block metadata.
     */
    metadata: BlockMetadataResponse;
}
