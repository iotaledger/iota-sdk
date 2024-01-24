// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Transform } from 'class-transformer';
import type { HexEncodedString } from '../utils/hex-encoding';
import { hexToBigInt } from '../utils/hex-encoding';
/**
 * Native token.
 */
export class INativeToken {
    /**
     * Identifier of the native token.
     */
    id!: HexEncodedString;
    /**
     * Amount of native tokens of the given Token ID.
     */
    @Transform((value) => hexToBigInt(value.value))
    amount!: bigint;
}
