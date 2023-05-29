// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Address } from '../block/address';
import { Output } from '../block/output/output';
import { TransactionEssence } from '../block/payload/transaction/essence';
import { IOutputMetadataResponse } from '../models/api';

/**
 * Helper struct for offline signing
 */
export interface IPreparedTransactionData {
    /**
     * Transaction essence
     */
    essence: TransactionEssence;
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
    output: Output;
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
    output: Output;
    /**
     * The chain derived from seed, for the remainder addresses
     */
    chain?: IBip32Chain;
    /**
     * The remainder address
     */
    address: Address;
}

/**
 * BIP 32 chain.
 */
export type IBip32Chain = number[];
