// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BlockId } from '../../block/id';
import { OutputId } from '../../block/output';
import { SlotCommitmentId, SlotIndex } from '../../block/slot';
import type { HexEncodedString } from '../../utils/hex-encoding';

/**
 * Metadata of an output.
 */
export interface IOutputMetadataResponse {
    /**
     * The ID of the output.
     */
    outputId: OutputId;
    /**
     * The ID of the block in which the output was included.
     */
    blockId: BlockId;
    /**
     * Metadata of the output if it is included in the ledger.
     */
    included: IOutputInclusionMetadata;
    /**
     * Metadata of the output if it is marked as spent in the ledger.
     */
    spent?: IOutputConsumptionMetadata;
    /**
     * Latest commitment ID of the node.
     */
    latestCommitmentId: HexEncodedString;
}

/**
 * Metadata of the output if it is included in the ledger.
 */
export interface IOutputInclusionMetadata {
    /**
     * Slot in which the output was included.
     */
    slot: SlotIndex;
    /**
     * Transaction ID that created the output.
     */
    transactionId: BlockId;
    /**
     * Commitment ID that includes the creation of the output.
     */
    commitmentId?: SlotCommitmentId;
}

/**
 * Metadata of the output if it is marked as spent in the ledger.
 */
export interface IOutputConsumptionMetadata {
    /**
     * Slot in which the output was spent.
     */
    slot: SlotIndex;
    /**
     * Transaction ID that spent the output.
     */
    transactionId: BlockId;
    /**
     * Commitment ID that includes the spending of the output.
     */
    commitmentId?: SlotCommitmentId;
}
