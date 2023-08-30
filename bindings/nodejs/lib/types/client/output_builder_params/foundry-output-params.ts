// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Feature, SimpleTokenScheme } from '../..';
import type { BasicOutputBuilderParams } from './basic-output-params';

/**
 * Options for building a Foundry Output
 */
export interface FoundryOutputBuilderParams extends BasicOutputBuilderParams {
    /**
     * The serial number of the foundry with respect to the controlling account.
     */
    serialNumber: number;
    tokenScheme: SimpleTokenScheme;
    immutableFeatures?: Feature[];
}
