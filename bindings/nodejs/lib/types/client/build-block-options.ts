// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { CoinType } from '../../client';
import type { IRange } from './range';
import type { Burn } from './burn';
import { UTXOInput } from '../block/input';
import { Output } from '../block/output';

/** Options to build a new block, possibly with payloads */
export interface IBuildBlockOptions {
    coinType?: CoinType;
    accountIndex?: number;
    initialAddressIndex?: number;
    inputs?: UTXOInput[];
    inputRange?: IRange;
    /** Bech32 encoded output address and amount */
    output?: IClientBlockBuilderOutputAddress;
    /** Hex encoded output address and amount */
    outputHex?: IClientBlockBuilderOutputAddress;
    outputs?: Output[];
    customRemainderAddress?: string;
    tag?: string;
    data?: string;
    /** Parent block IDs */
    parents?: string[];
    /** Explicit burning of aliases, nfts, foundries and native tokens */
    burn?: Burn;
}

/** Address with base coin amount */
export interface IClientBlockBuilderOutputAddress {
    address: string;
    amount: bigint | string;
}
