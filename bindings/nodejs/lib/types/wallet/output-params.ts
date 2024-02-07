// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SlotIndex } from '../block/slot';
import { NativeToken } from '../models';
import { u64 } from '../utils';
import { HexEncodedString } from '../utils/hex-encoding';

/** Options for the creation of an output. */
export interface OutputParams {
    /** A recipient address. */
    recipientAddress: string;
    /** An amount. */
    amount: u64 | string;
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
    /** Native Token to include. */
    nativeToken?: NativeToken;
}

/** Time unlocks to include in the output. */
export interface Unlocks {
    /** The expiration slot index that marks the end of the expiration period. */
    expirationSlotIndex?: SlotIndex;
    /** The timelock slot index that marks the end of the locking period. */
    timelockSlotIndex?: SlotIndex;
}

/** Storage deposit strategy to be used for the output. */
export interface StorageDeposit {
    /**
     * The return strategy.
     */
    returnStrategy?: ReturnStrategy;
    /**
     * Determines whether the storage deposit will automatically add excess small funds when necessary.
     * For example, given an account has 20 tokens and wants to send 15 tokens, and the minimum amount
     * is 10 tokens, it wouldn't be possible to create an output with the 5 token remainder. If this flag is enabled,
     * the 5 tokens will be added to the output automatically.
     */
    useExcessIfLow?: boolean;
}

/** Return strategy for the storage deposit. */
export enum ReturnStrategy {
    /** A storage deposit return unlock condition will be added with the additional amount needed to satisfy storage costs. */
    Return = 'Return',
    /** The recipient address will receive the additional amount needed to satisfy storage costs. */
    Gift = 'Gift',
}
