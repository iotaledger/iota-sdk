// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Feature, HexEncodedString } from '../..';
import type { BasicOutputBuilderParams } from './basic-output-params';

/**
 * Options for building an Alias Output.
 */
export interface AliasOutputBuilderParams extends BasicOutputBuilderParams {
    /**
     * A unique ID for the new alias.
     */
    aliasId: HexEncodedString;
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
