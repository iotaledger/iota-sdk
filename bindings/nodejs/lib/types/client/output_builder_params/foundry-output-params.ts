// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Feature, SimpleTokenScheme } from '../..';
import type { BasicOutputBuilderParams } from './basic-output-params';

/**
 * Options for building a Foundry Output.
 */
export interface FoundryOutputBuilderParams extends BasicOutputBuilderParams {
    /**
     * The serial number of the foundry with respect to the controlling alias.
     */
    serialNumber: number;
    /**
     * Defines the supply control scheme of the tokens controlled by the foundry.
     * Currently only a simple scheme is supported.
     */
    tokenScheme: SimpleTokenScheme;
    /**
     * Features that add utility to the output but do not impose unlocking conditions.
     * These features need to be kept in future transitions of the UTXO state machine.
     */
    immutableFeatures?: Feature[];
}
