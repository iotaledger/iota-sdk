// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { INativeToken } from '../models';
import { HexEncodedString } from '../utils/hex-encoding';

/** Options for the creation of an output */
export interface OutputParams {
    recipientAddress: string;
    amount: bigint | string;
    assets?: Assets;
    features?: Features;
    unlocks?: Unlocks;
    storageDeposit?: StorageDeposit;
}

/** Assets to include in the output */
export interface Assets {
    nativeTokens?: INativeToken[];
    nftId?: HexEncodedString;
}

/** Features to include in the output */
export interface Features {
    tag?: HexEncodedString;
    metadata?: HexEncodedString;
    sender?: string;
    issuer?: string;
}

/** Time unlocks to include in the output */
export interface Unlocks {
    expirationUnixTime?: number;
    timelockUnixTime?: number;
}

/** Storage deposit strategy to be used for the output */
export interface StorageDeposit {
    returnStrategy?: ReturnStrategy;
    useExcessIfLow?: boolean;
}

/** Return strategy for the storage deposit */
export enum ReturnStrategy {
    Return = 'Return',
    Gift = 'Gift',
}
