// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Hex encoded bytes.
 */
export declare type HexEncodedString = string;
/**
 * Hex encoded U256.
 */
export declare type HexEncodedAmount = string;

export function bigIntToHex(value: BigInt): string {
    return '0x' + value.toString(16);
}

export function hexToBigInt(
    value: HexEncodedAmount | HexEncodedString,
): bigint {
    if (!value.startsWith('0x')) {
        value = '0x' + value;
    }
    return BigInt(value);
}
