// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { CoinType } from '../../client';
import type { IRange } from './range';
import type { Burn } from './burn';
import { UTXOInput } from '../block/input';
import { Output } from '../block/output';

/**
 * Options to build a new block, possibly with payloads.
 */
export interface IBuildBlockOptions {
    /** A coin type. */
    coinType?: CoinType;
    /** An account index. */
    accountIndex?: number;
    /** An initial address index. */
    initialAddressIndex?: number;
    /** A list of inputs. */
    inputs?: UTXOInput[];
    /** An input range. */
    inputRange?: IRange;
    /** Bech32 encoded output address and amount. */
    output?: IClientBlockBuilderOutputAddress;
    /** Hex encoded output address and amount. */
    outputHex?: IClientBlockBuilderOutputAddress;
    /** A list of outputs. */
    outputs?: Output[];
    /** A custom remainder address. */
    customRemainderAddress?: string;
    /** A tag. */
    tag?: string;
    /** Some metadata. */
    data?: string;
    /** Some parent block IDs. */
    parents?: string[];
    /** Parameters for explicit burning of aliases, nfts, foundries and native tokens. */
    burn?: Burn;
}

/** Address with base coin amount. */
export interface IClientBlockBuilderOutputAddress {
    /** An address. */
    address: string;
    /** Some base coin amount. */
    amount: bigint | string;
}
