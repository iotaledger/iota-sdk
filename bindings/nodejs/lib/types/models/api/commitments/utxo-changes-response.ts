// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { OutputId } from '../../../block/output';
import { SlotCommitmentId } from '../../../block';
import { Output, OutputDiscriminator } from '../../../block/output';
import { Type } from 'class-transformer';

/**
 * All UTXO changes that happened at a specific slot.
 */
export class UtxoChangesResponse {
    /**
     * The commitment ID of the requested slot that contains the changes.
     */
    commitmentId!: SlotCommitmentId;
    /**
     * The created outputs of the given slot.
     */
    createdOutputs!: OutputId[];
    /**
     * The consumed outputs of the given slot.
     */
    consumedOutputs!: OutputId[];
}

/**
 * An output with its id.
 */
export class OutputWithId {
    /**
     * The output id.
     */
    outputId!: OutputId;
    /**
     * The output.
     */
    @Type(() => Output, {
        discriminator: OutputDiscriminator,
    })
    output!: Output;
}

/**
 * All full UTXO changes that happened at a specific slot.
 */
export class UtxoChangesFullResponse {
    /**
     * The commitment ID of the requested slot that contains the changes.
     */
    commitmentId!: SlotCommitmentId;
    /**
     * The created outputs of the given slot.
     */
    createdOutputs!: OutputWithId[];
    /**
     * The consumed outputs of the given slot.
     */
    consumedOutputs!: OutputWithId[];
}
