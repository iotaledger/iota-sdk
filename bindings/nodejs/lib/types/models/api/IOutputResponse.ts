// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Output } from '../../block/output';
import type { IOutputMetadataResponse } from './IOutputMetadataResponse';
/**
 * Details of an output.
 */
export interface IOutputResponse {
    /**
     * The metadata about the output.
     */
    metadata: IOutputMetadataResponse;
    /**
     * The output.
     */
    output: Output;
}
