// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { NumericString } from '../utils';

/**
 * Defines the parameters of storage score calculations on objects which take node resources.
 */
export interface StorageScoreParameters {
    /**
     *  Defines the number of IOTA tokens required per unit of storage score.
     */
    storageCost: NumericString;
    /**
     * Defines the factor to be used for data only fields.
     */
    factorData: number;
    /**
     * Defines the offset to be applied to all outputs for the overhead of handling them in storage.
     */
    offsetOutputOverhead: NumericString;
    /**
     * Defines the offset to be used for block issuer feature public keys.
     */
    offsetEd25519BlockIssuerKey: NumericString;
    /**
     * Defines the offset to be used for staking feature.
     */
    offsetStakingFeature: NumericString;
    /**
     * Defines the offset to be used for delegation output.
     */
    offsetDelegation: NumericString;
}
