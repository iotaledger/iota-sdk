// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { u256 } from '../utils';
import type { HexEncodedString } from '../utils/hex-encoding';
import { Transform } from 'class-transformer';
import { hexToBigInt } from '../utils/hex-encoding';

/**
 * Native token.
 */
export class NativeToken {
    /**
     * Identifier of the native token.
     */
    id!: HexEncodedString;
    /**
     * Amount of native tokens of the given Token ID.
     */
    @Transform((value) => hexToBigInt(value.value))
    amount!: u256;
}
