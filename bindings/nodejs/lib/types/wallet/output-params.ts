// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { INativeToken } from '../models';
import { HexEncodedString } from '../utils/hex-encoding';

/** Options for the creation of an output */
export interface OutputParams {
    /** TODO */
    recipientAddress: string;
    /** TODO */
    amount: bigint | string;
    /** TODO */
    assets?: Assets;
    /** TODO */
    features?: Features;
    /** TODO */
    unlocks?: Unlocks;
    /** TODO */
    storageDeposit?: StorageDeposit;
}

/** Assets to include in the output */
export interface Assets {
    /** TODO */
    nativeTokens?: INativeToken[];
    /** TODO */
    nftId?: HexEncodedString;
}

/** Features to include in the output */
export interface Features {
    /** TODO */
    tag?: HexEncodedString;
    /** TODO */
    metadata?: HexEncodedString;
    /** TODO */
    sender?: string;
    /** TODO */
    issuer?: string;
}

/** Time unlocks to include in the output */
export interface Unlocks {
    /** TODO */
    expirationUnixTime?: number;
    /** TODO */
    timelockUnixTime?: number;
}

/** Storage deposit strategy to be used for the output */
export interface StorageDeposit {
    /** TODO */
    returnStrategy?: ReturnStrategy;
    /** TODO */
    useExcessIfLow?: boolean;
}

/** Return strategy for the storage deposit */
export enum ReturnStrategy {
    /** TODO */
    Return = 'Return',
    /** TODO */
    Gift = 'Gift',
}
