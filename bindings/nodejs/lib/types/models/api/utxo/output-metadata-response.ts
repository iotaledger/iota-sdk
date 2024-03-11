// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { OutputIdProof, TransactionId } from '../../..';
import { BlockId } from '../../../block/id';
import { Output, OutputId } from '../../../block/output';
import { SlotCommitmentId, SlotIndex } from '../../../block/slot';

/**
 * Metadata of the output if it is included in the ledger.
 */
export interface OutputInclusionMetadata {
    /**
     * Slot in which the output was included.
     */
    slot: SlotIndex;
    /**
     * Transaction ID that created the output.
     */
    transactionId: TransactionId;
    /**
     * Commitment ID that includes the creation of the output.
     */
    commitmentId?: SlotCommitmentId;
}

/**
 * Metadata of the output if it is marked as spent in the ledger.
 */
export interface OutputConsumptionMetadata {
    /**
     * Slot in which the output was spent.
     */
    slot: SlotIndex;
    /**
     * Transaction ID that spent the output.
     */
    transactionId: TransactionId;
    /**
     * Commitment ID that includes the spending of the output.
     */
    commitmentId?: SlotCommitmentId;
}

/**
 * Metadata of an output.
 */
export interface OutputMetadataResponse {
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
    included: OutputInclusionMetadata;
    /**
     * Metadata of the output if it is marked as spent in the ledger.
     */
    spent?: OutputConsumptionMetadata;
    /**
     * Latest commitment ID of the node.
     */
    latestCommitmentId: SlotCommitmentId;
}

/**
 * An output with its output id proof and its metadata.
 * Response of GET /api/core/v3/outputs/{outputId}/full.
 */
export class OutputWithMetadataResponse {
    /**
     * One of the possible output types.
     */
    output!: Output;
    /**
     * The associated Output ID proof.
     */
    outputIdProof!: OutputIdProof;
    /**
     * The metadata of the output.
     */
    metadata!: OutputMetadataResponse;
}

/**
 * An output and its metadata.
 */
export class OutputWithMetadata {
    /**
     * One of the possible output types.
     */
    output!: Output;
    /**
     * The metadata of the output.
     */
    metadata!: OutputMetadataResponse;
}
