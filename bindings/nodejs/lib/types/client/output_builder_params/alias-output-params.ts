// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Feature, HexEncodedString } from '../..';
import type { BasicOutputBuilderParams } from './basic-output-params';

/**
 * Options for building an Alias Output
 */
export interface AccountOutputBuilderParams extends BasicOutputBuilderParams {
    accountId: HexEncodedString;
    stateIndex?: number;
    stateMetadata?: HexEncodedString;
    foundryCounter?: number;
    immutableFeatures?: Feature[];
}
