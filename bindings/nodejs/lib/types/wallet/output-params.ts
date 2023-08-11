// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { INativeToken } from '../models';
import { HexEncodedString } from '../utils/hex-encoding';

/** Options for the creation of an output. */
export interface OutputParams {
    /** A recipient address. */
    recipientAddress: string;
    /** An amount. */
    amount: bigint | string;
    /** Assets to include. */
    assets?: Assets;
    /** Features to include. */
    features?: Features;
    /** Unlocks to include. */
    unlocks?: Unlocks;
    /** The storage deposit strategy to use. */
    storageDeposit?: StorageDeposit;
}

/** Assets to include in the output. */
export interface Assets {
    /** Native Token assets to include. */
    nativeTokens?: INativeToken[];
    /** The NFT to include. */
    nftId?: HexEncodedString;
}

/** Features to include in the output. */
export interface Features {
    /** A Tag feature to include. */
    tag?: HexEncodedString;
    /** A Metadata feature to include. */
    metadata?: HexEncodedString;
    /** A Sender feature to include. */
    sender?: string;
    /** An Issuer feature to include. */
    issuer?: string;
}

/** Time unlocks to include in the output. */
export interface Unlocks {
    /** The expiration Unix timestamp that marks the end of the expiration period. */
    expirationUnixTime?: number;
    /** The timelock Unix timestamp that marks the end of the locking period. */
    timelockUnixTime?: number;
}

/** Storage deposit strategy to be used for the output. */
export interface StorageDeposit {
    /**
     * The return strategy.
     */
    returnStrategy?: ReturnStrategy;
    /**
     * If account has 2 Mi, min storage deposit is 1 Mi and one wants to send 1.5 Mi, it wouldn't be possible with a
     * 0.5 Mi remainder. To still send a transaction, the 0.5 can be added to the output automatically, if set to true.
     */
    useExcessIfLow?: boolean;
}

/** Return strategy for the storage deposit. */
export enum ReturnStrategy {
    /** A storage deposit return unlock condition will be added with the required minimum storage deposit. */
    Return = 'Return',
    /** The recipient address will get the additional amount to reach the minimum storage deposit gifted. */
    Gift = 'Gift',
}
