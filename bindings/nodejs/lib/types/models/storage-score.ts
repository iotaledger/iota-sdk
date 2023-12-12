// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { u64 } from "../utils/type-aliases";

/**
 * Defines the parameters of storage score calculations on objects which take node resources.
 */
export interface StorageScoreParameters {
    /**
     *  Defines the number of IOTA tokens required per unit of storage score.
     */
    storageCost: u64;
    /**
     * Defines the factor to be used for data only fields.
     */
    factorData: number;
    /**
     * Defines the offset to be applied to all outputs for the overhead of handling them in storage.
     */
    offsetOutputOverhead: u64;
    /**
     * Defines the offset to be used for block issuer feature public keys.
     */
    offsetEd25519BlockIssuerKey: u64;
    /**
     * Defines the offset to be used for staking feature.
     */
    offsetStakingFeature: u64;
    /**
     * Defines the offset to be used for delegation output.
     */
    offsetDelegation: u64;
}
