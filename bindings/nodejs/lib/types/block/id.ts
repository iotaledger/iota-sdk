// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../utils';

export type AliasId = HexEncodedString;

/** A block identifier, the BLAKE2b-256 hash of the block bytes.
 * See <https://www.blake2.net/> for more information.
 */
export type BlockId = HexEncodedString;

export type TokenId = HexEncodedString;
