// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountId, Feature } from '../..';
import type { BasicOutputBuilderParams } from './basic-output-params';

/**
 * Options for building an Account Output
 */
export interface AccountOutputBuilderParams extends BasicOutputBuilderParams {
    /**
     * Unique identifier of an account, which is the BLAKE2b-256 hash of the Output ID that created it.
     */
    accountId: AccountId;
    /**
     * A counter that denotes the number of foundries created by this account output.
     */
    foundryCounter?: number;
    /**
     * A list of immutable features.
     */
    immutableFeatures?: Feature[];
}
