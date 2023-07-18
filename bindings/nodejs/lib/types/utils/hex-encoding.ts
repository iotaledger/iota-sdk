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

/**
 * Converts `bigint` value to hexadecimal string representation prefixed with "0x".
 */
export function bigIntToHex(value: bigint): string {
    return '0x' + value.toString(16);
}

/**
 * Converts hex encoded string to `bigint` value.
 */
export function hexToBigInt(
    value: HexEncodedAmount | HexEncodedString,
): bigint {
    if (!value.startsWith('0x')) {
        value = '0x' + value;
    }
    return BigInt(value);
}
