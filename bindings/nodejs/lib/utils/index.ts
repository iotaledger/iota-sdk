// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

export * from './utf8';
export * from './utils';
export * from '../types/utils';

/**
 * Converts a byte array to a hexadecimal string.
 *
 * @param {Uint8Array} byteArray - The bytes to encode.
 * @param {boolean} [prefix=false] - Whether to include the '0x' prefix in the resulting hexadecimal string.
 * @returns {string} The hexadecimal representation of the input byte array.
 */
export const bytesToHex = (bytes: ArrayLike<number>, prefix = false) => {
    const hexArray = Array.from(bytes, (byte) =>
        byte.toString(16).padStart(2, '0'),
    );
    const hexString = hexArray.join('');
    return prefix ? '0x' + hexString : hexString;
};

/**
 * Converts a hexadecimal string to a Uint8Array byte array.
 *
 * @param {string} hexString - The hexadecimal string to be converted.
 * @returns {Uint8Array} The Uint8Array byte array representation of the input hexadecimal string.
 * @throws {Error} Will throw an error if the input string is not a valid hexadecimal string.
 */
export const hexToBytes = (hexString: string) => {
    const hex = hexString.replace(/^0x/, '');
    const bytes = [];
    for (let i = 0; i < hex.length; i += 2) {
        bytes.push(parseInt(hex.substr(i, 2), 16));
    }
    return new Uint8Array(bytes);
};
