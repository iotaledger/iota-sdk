// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { CoinType } from './constants';
import type { Range } from './range';

/**
 * Input options for GenerateAddresses
 */
export interface GenerateAddressesOptions {
    coinType?: CoinType;
    accountIndex?: number;
    range?: Range;
    /**
     * Bech32 human readable part
     */
    bech32Hrp?: string;
    options?: GenerateAddressOptions;
}

/**
 * Options provided to Generate Address
 */
export interface GenerateAddressOptions {
    /**
     * Whether to generate an internal address.
     */
    internal: boolean;
    /**
     * Display the address on ledger devices.
     */
    ledgerNanoPrompt: boolean;
}
