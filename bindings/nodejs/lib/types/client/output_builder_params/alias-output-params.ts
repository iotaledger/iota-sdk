// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AliasId, Feature, HexEncodedString } from '../..';
import type { BasicOutputBuilderParams } from './basic-output-params';

/**
 * Options for building an Alias Output.
 */
export interface AliasOutputBuilderParams extends BasicOutputBuilderParams {
    /**
     * Unique identifier of an alias, which is the BLAKE2b-256 hash of the Output ID that created it.
     */
    aliasId: AliasId;
    /**
     * A counter that must increase by 1 every time the alias is state transitioned.
     */
    stateIndex?: number;
    /**
     * Metadata that can only be changed by the state controller.
     */
    stateMetadata?: HexEncodedString;
    /**
     * A counter that denotes the number of foundries created by this alias account.
     */
    foundryCounter?: number;
    /**
     * A list of immutable features.
     */
    immutableFeatures?: Feature[];
}
