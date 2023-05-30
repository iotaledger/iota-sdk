// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { HexEncodedString } from '../utils/hexEncodedTypes';
import { Payload, PayloadDiscriminator } from './payload';
import { Type } from 'class-transformer';
/**
 * The default protocol version.
 */
export declare const DEFAULT_PROTOCOL_VERSION: number;
/**
 * Block layout.
 */
export class IBlock {
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
    /**
     * The nonce for the block.
     */
    nonce!: string;
}
