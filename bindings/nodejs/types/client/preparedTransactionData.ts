// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type {
    AddressTypes,
    IOutputMetadataResponse,
    ITransactionEssence,
    OutputTypes,
} from '@iota/types';

/**
 * Helper struct for offline signing
 */
export interface IPreparedTransactionData {
    /**
     * Transaction essence
     */
    essence: ITransactionEssence;
    /**
     * Required address information for signing
     */
    inputsData: IInputSigningData[];
    /**
     * Optional remainder output information
     */
    remainder?: IRemainder;
}

/**
 * Data for transaction inputs for signing and ordering of unlock blocks
 */
export interface IInputSigningData {
    /**
     * The output
     */
    output: OutputTypes;
    /**
     * The output metadata
     */
    outputMetadata: IOutputMetadataResponse;
    /**
     * The chain derived from seed, only for ed25519 addresses
     */
    chain?: IBip32Chain;
}

export interface IRemainder {
    /**
     * The remainder output
     */
    output: OutputTypes;
    /**
     * The chain derived from seed, for the remainder addresses
     */
    chain?: IBip32Chain;
    /**
     * The remainder address
     */
    address: AddressTypes;
}

export interface ISegment {
    hardened: boolean;
    bs: number[];
}

/**
 * BIP 32 chain.
 */
export type IBip32Chain = ISegment[];
