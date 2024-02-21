// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BlockId, SlotCommitment, SlotIndex } from '../../block';

/**
 * Block id response.
 */
export interface IBlockIdResponse {
    /**
     * The block id.
     */
    blockId: BlockId;
}

/**
 * Information that is used to attach a block in the network.
 * Response of GET /api/core/v3/blocks/issuance
 */
export interface IIssuanceBlockHeaderResponse {
    /**
     * Blocks that are strongly directly approved.
     */
    strongParents: BlockId[];
    /**
     * Latest issuing time of the returned parents.
     */
    latestParentBlockIssuingTime: number;
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
