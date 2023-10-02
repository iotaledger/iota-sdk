// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
export const IOTA_BECH32_HRP = 'iota';
export const IOTA_TESTNET_BECH32_HRP = 'atoi';
export const SHIMMER_BECH32_HRP = 'smr';
export const SHIMMER_TESTNET_BECH32_HRP = 'rms';

/** BIP44 Coin Types for IOTA and Shimmer. */
export enum CoinType {
    /** The IOTA coin. */
    IOTA = 4218,
    /** The Shimmer coin. */
    Shimmer = 4219,
    /** The Ether coin. */
    Ether = 60,
}
