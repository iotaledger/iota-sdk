// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Output, OutputDiscriminator } from '../../../block/output';
import { OutputIdProof } from './output-id-proof';

import { Type } from 'class-transformer';

/**
 * An output with its output id proof.
 * Response of GET /api/core/v3/outputs/{outputId}.
 */
export class OutputResponse {
    /**
     * One of the possible output types.
     */
    @Type(() => Output, {
        discriminator: OutputDiscriminator,
    })
    output!: Output;
    /**
     * The proof of the output identifier.
     */
    outputIdProof!: OutputIdProof;
}
