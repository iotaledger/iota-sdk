// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { HexEncodedString } from '../utils/hex-encoding';
import { Payload, PayloadDiscriminator } from './payload';
import { Type } from 'class-transformer';

/**
 * Block layout.
 */
export class Block {
    /**
     * The protocol version under which this block operates.
     */
    protocolVersion!: number;
    /**
     * The parent block ids.
     */
    parents!: HexEncodedString[];
    /**
     * The payload contents.
     */
    @Type(() => Payload, {
        discriminator: PayloadDiscriminator,
    })
    payload?: Payload;
}
