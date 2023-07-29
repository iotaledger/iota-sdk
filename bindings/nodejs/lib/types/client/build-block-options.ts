// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { CoinType } from '../../client';
import type { IRange } from './range';
import type { Burn } from './burn';
import { UTXOInput } from '../block/input';
import { Output } from '../block/output';

/** Options to build a new block, possibly with payloads */
export interface IBuildBlockOptions {
    /** TODO */
    coinType?: CoinType;
    /** TODO */
    accountIndex?: number;
    /** TODO */
    initialAddressIndex?: number;
    /** TODO */
    inputs?: UTXOInput[];
    /** TODO */
    inputRange?: IRange;
    /** Bech32 encoded output address and amount */
    output?: IClientBlockBuilderOutputAddress;
    /** Hex encoded output address and amount */
    outputHex?: IClientBlockBuilderOutputAddress;
    /** TODO */
    outputs?: Output[];
    /** TODO */
    customRemainderAddress?: string;
    /** TODO */
    tag?: string;
    /** TODO */
    data?: string;
    /** Parent block IDs */
    parents?: string[];
    /** Explicit burning of aliases, nfts, foundries and native tokens */
    burn?: Burn;
}

/** Address with base coin amount */
export interface IClientBlockBuilderOutputAddress {
    /** TODO */
    address: string;
    /** TODO */
    amount: bigint | string;
}
