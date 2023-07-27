// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Output, OutputDiscriminator } from '../../block/output';
import type { IOutputMetadataResponse } from './output-metadata-response';

import { Type } from 'class-transformer';

/**
 * Details of an output.
 */
export class OutputResponse {
    /**
     * The metadata about the output.
     */
    metadata!: IOutputMetadataResponse;
    /**
     * The output.
     */
    @Type(() => Output, {
        discriminator: OutputDiscriminator,
    })
    output!: Output;
}
