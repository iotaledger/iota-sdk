// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { ConflictReason, LedgerInclusionState } from './internal';
import type { HexEncodedString } from '../utils';
/**
 * Response from the metadata endpoint.
 */
export interface IBlockMetadata {
    /**
     * The block id.
     */
    blockId: HexEncodedString;
    /**
     * The parent block ids.
     */
    parents: HexEncodedString[];
    /**
     * Is the block solid.
     */
    isSolid: boolean;
    /**
     * Is the block referenced by a milestone.
     */
    referencedByMilestoneIndex?: number;
    /**
     * Is this block a valid milestone.
     */
    milestoneIndex?: number;
    /**
     * The ledger inclusion state.
     */
    ledgerInclusionState?: LedgerInclusionState;
    /**
     * The conflict reason.
     */
    conflictReason?: ConflictReason;
    /**
     * Should the block be promoted.
     */
    shouldPromote?: boolean;
    /**
     * Should the block be reattached.
     */
    shouldReattach?: boolean;
}
