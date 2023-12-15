// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { OutputId } from '../../block/output';
import { SlotCommitmentId } from '../../block';

/**
 * Returns all UTXO changes that happened at a specific slot.
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
