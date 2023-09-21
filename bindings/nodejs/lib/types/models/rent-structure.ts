// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Defines the parameters of rent cost calculations on objects which take node resources.
 */
export interface RentStructure {
    /**
     * Defines the rent of a single virtual byte denoted in IOTA token.
     */
    vByteCost: number;
    /**
     * The factor to be used for data only fields.
     */
    vByteFactorData: number;
    /**
     * The factor to be used for key/lookup generating fields.
     */
    vByteFactorKey: number;
    /**
     * Defines the factor to be used for block issuer feature public keys.
     */
    vByteFactorBlockIssuerKey: number;
    /**
     * Defines the factor to be used for staking feature.
     */
    vByteFactorStakingFeature: number;
}
