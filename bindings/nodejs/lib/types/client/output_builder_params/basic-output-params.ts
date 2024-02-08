// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    UnlockCondition,
    Feature,
    NativeToken,
    NumericString,
    u64,
} from '../..';

/**
 * Options for building a Basic Output
 */
export interface BasicOutputBuilderParams {
    /**
     * If not provided, minimum amount will be used
     */
    amount?: u64 | NumericString;
    /**
     * The native tokens to be held by the output.
     */
    nativeTokens?: NativeToken[];
    /**
     * The unlock conditions for the output.
     */
    unlockConditions: UnlockCondition[];
    /**
     * Features to be contained by the output.
     */
    features?: Feature[];
}
